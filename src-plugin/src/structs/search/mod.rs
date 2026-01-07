use std::{num::NonZero, sync::Arc, thread::available_parallelism, time::Duration};

use ahqstore_types::{Commits, SearchEntry};
use tantivy::{
  collector::TopDocs,
  query::{BooleanQuery, FuzzyTermQuery, QueryParser},
  schema::{Field, Schema, Value, FAST, STORED, STRING, TEXT},
  Index, IndexReader, ReloadPolicy, TantivyDocument, Term,
};
use tauri::{async_runtime::spawn_blocking, AppHandle, Manager, Runtime};
use tokio::{fs, sync::RwLock, time::sleep};

use ahqstore_types::internet::get_all_search;

pub struct CommitSearchIndex {
  pub commit: Commits,
  pub meta: Option<SearchMeta>,
}

pub struct SearchMeta {
  pub reader: IndexReader,
  pub name_txt: Field,
  pub title_txt: Field,
  pub id: Field,
}

impl CommitSearchIndex {
  pub fn search(&self, query: &str) -> Result<Vec<String>, ()> {
    let Some(meta) = &self.meta else {
      return Err(());
    };

    let searcher = meta.reader.searcher();

    let name_data = Term::from_field_text(meta.name_txt, query);
    let name_query = FuzzyTermQuery::new(name_data, 2, true);

    let title_term = Term::from_field_text(meta.title_txt, query);
    let title_query = FuzzyTermQuery::new(title_term, 2, true);

    let query = BooleanQuery::union(vec![Box::new(name_query), Box::new(title_query)]);

    let Ok(mut search) = searcher.search(&query, &TopDocs::with_limit(500)) else {
      return Err(());
    };

    let results: Vec<String> = search
      .into_iter()
      .map(|(_, b)| {
        let data: TantivyDocument = searcher.doc(b).expect("Couldn't deserialize document");

        data
          .get_first(meta.id)
          .map(|x| x.as_str().map(|x| x.to_string()))
          .flatten()
      })
      // Flatten to auto ignore None values
      .flatten()
      .collect();

    Ok(results)
  }
}

pub async fn search_daemon<R: Runtime>(hwnd: AppHandle<R>, lck: Arc<RwLock<CommitSearchIndex>>) {
  let mut initial = lck.read().await.commit.clone();

  let mut builder = Schema::builder();
  let name_txt = builder.add_text_field("name", TEXT);
  let title_txt = builder.add_text_field("title", TEXT);
  let id = builder.add_text_field("id", STRING | STORED | FAST);
  let schema = builder.build();

  let index = {
    let mut cache = hwnd
      .path()
      .app_cache_dir()
      .expect("Cannot fetch cache dir, exiting");
    cache.push("searchdb");

    _ = fs::create_dir_all(&cache).await;

    {
      cache.push("commit.lck");
      let data = fs::read_to_string(&cache).await.unwrap_or_default();

      if &data != &format!("{initial:?}") {
        // Load from disk
        initial.ahqstore = Default::default();
        initial.alt = Default::default();
      }
      cache.pop();
    }

    cache.push("index");

    if cache.exists() && &initial.ahqstore != "" {
      Index::open_in_dir(&cache).expect("Unable to create db, exiting")
    } else {
      _ = fs::remove_dir_all(&cache).await;

      Index::create_in_dir(&cache, schema).expect("Unable to create db, exiting")
    }
  };

  let index = Arc::new(index);

  {
    let mut data = lck.write().await;

    data.meta = Some(SearchMeta {
      id,
      name_txt,
      reader: index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .num_warming_threads(
          available_parallelism()
            .unwrap_or(unsafe { NonZero::new_unchecked(4usize) })
            .get()
            .clamp(2, 4),
        )
        .try_into()
        .expect("Unable to construct reader"),
      title_txt,
    });

    drop(data);
  }

  loop {
    let mut to_update;

    // Checking phase
    {
      let commit_lck = lck.read().await;
      let commit = &commit_lck.commit;

      if &initial == commit {
        sleep(Duration::from_secs(30)).await;
        continue;
      }

      to_update = commit.clone();
      drop(commit_lck);
    }

    // Building phase
    if let Ok(searches) = get_all_search(&to_update).await {
      let idx_c = index.clone();
      // Fetching phase
      if let Ok(Ok(_)) =
        spawn_blocking(move || build_search(idx_c.as_ref(), name_txt, title_txt, id, searches))
          .await
      {
        let mut wrlck = lck.write().await;

        if &wrlck.commit == &to_update {
          initial = to_update;

          let mut cache = hwnd
            .path()
            .app_cache_dir()
            .expect("Cannot fetch cache dir, exiting");
          cache.push("searchdb");
          cache.push("commit.lck");

          _ = fs::write(cache, format!("{initial:?}")).await;
        }
      }
    }

    sleep(Duration::from_secs(30)).await;
  }
}

fn build_search(
  index: &Index,
  name_txt: Field,
  title_txt: Field,
  id: Field,
  data: Vec<SearchEntry>,
) -> tantivy::Result<()> {
  let mut index_writer = index.writer(
    data
      .len()
      // Let's assume each entry takes approx 128-bytes, plenty
      .saturating_mul(128)
      .clamp(10 * 1024 * 1024, 100 * 1024 * 1024),
  )?;

  // Clean up all documents as currently delta updates is not done
  index_writer.delete_all_documents()?;

  for item in data {
    index_writer.add_document(tantivy::doc!(
      name_txt => item.name,
      title_txt => item.title,
      id => item.id
    ))?;
  }

  index_writer.commit()?;
  index_writer.wait_merging_threads()?;

  Ok(())
}
