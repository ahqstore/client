use super::{get_all_commits, get_all_search, linux, Commits, SearchEntry};
use anyhow::{Context, Result};
use fuse_rust::{Fuse, Fuseable};
use serde::Serialize;
use std::{
  ptr::{addr_of, addr_of_mut},
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{RwLock, RwLockReadGuard};

fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

struct SCache {
  e: Vec<SearchEntry>,
  commit: Commits,
}

static SEARCH: LazyLock<RwLock<Option<SCache>>> = LazyLock::new(|| RwLock::new(None));
static FUSE: LazyLock<Fuse> = LazyLock::new(|| Fuse::default());

// async fn get_search_inner(commit: &Commits) -> Option<RwLockReadGuard<'static, Option<SCache>>> {
//   let search = SEARCH.read().await;
//   let search_ref = search.as_ref();

//   let Some(x) = search_ref else {
//     return None;
//   };

//   if &x.commit == commit {
//     return Some(search);
//   }

//   return None;
// }

#[derive(Debug)]
pub enum RespSearchEntry {
  Static(&'static SearchEntry),
  Owned(SearchEntry),
  None,
}

impl Fuseable for RespSearchEntry {
  fn lookup(&self, key: &str) -> Option<&str> {
    match self {
      RespSearchEntry::Owned(x) => x.lookup(key),
      RespSearchEntry::Static(x) => x.lookup(key),
      RespSearchEntry::None => None,
    }
  }

  fn properties(&self) -> Vec<fuse_rust::FuseProperty> {
    match self {
      RespSearchEntry::Owned(x) => x.properties(),
      RespSearchEntry::Static(x) => x.properties(),
      RespSearchEntry::None => vec![],
    }
  }
}

impl Serialize for RespSearchEntry {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      RespSearchEntry::Owned(x) => x.serialize(serializer),
      RespSearchEntry::Static(x) => x.serialize(serializer),
      RespSearchEntry::None => "".serialize(serializer),
    }
  }
}

/// Responds with ids of applications selected
/// Upto 500 results at a time
pub async fn get_search(commit: &Commits, query: &str) -> Result<Vec<String>> {
  let data = SEARCH.read().await;
  let s_ref = data.as_ref();

  let mut need_to_dwnl = false;

  if let Some(x) = s_ref {
    if &x.commit != commit {
      need_to_dwnl = true;
    }
  } else {
    need_to_dwnl = true;
  }

  // Drop read handle
  drop(data);

  if need_to_dwnl {
    let search = get_all_search(commit).await?;

    let mut s = SEARCH.write().await;
    *s = Some(SCache {
      e: search,
      commit: commit.clone()
    });

    // Drop write handle
    drop(s);
  }

  let search_obj: RwLockReadGuard<'static, Option<SCache>> = SEARCH.read().await;
  let search = search_obj.as_ref();
  let search: &[SearchEntry] = &search.unwrap().e;


  let mut res = vec![];
  let mut len: usize = 0;

  let query_casted: &'static str = unsafe { std::mem::transmute(query) };
  let search_casted: &'static [SearchEntry] = unsafe { std::mem::transmute(search) };

  let result = tokio::task::spawn_blocking(|| {
    FUSE.search_text_in_fuse_list(query_casted, search_casted)
  }).await?;

  for val in result {
    if len < 500 {
      res.push(search[val.index].id.clone());
    } else {
      break;
    }

    len += 1;
  }

  Ok(res)
}
