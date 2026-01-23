use std::{
  io::SeekFrom,
  sync::{Arc, LazyLock},
  time::Duration,
};

use reqwest::{Client, ClientBuilder, StatusCode};
use tauri::async_runtime::{self, JoinHandle};
use tokio::{
  fs::{create_dir_all, remove_dir_all, remove_file, File, OpenOptions},
  io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
  sync::mpsc::{channel, error::TryRecvError, Sender},
  task::spawn_blocking,
  time::sleep,
};

use crate::structs::platform::turbofile::TurboFile;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
    .build()
    .unwrap()
});

pub async fn download<F: FnMut(f64) -> (), T: FnOnce(u64) -> ()>(
  url: &str,
  out_file: &str,
  out_dir: &str,
  turbo: bool,
  mut len_fn: T,
  mut log: F,
) -> Option<()> {
  let _ = remove_dir_all(out_dir).await;
  create_dir_all(out_dir).await.ok()?;

  let ranged = turbo && supports_ranged(url).await;

  log(0.0);

  let size = get_size(url).await;

  if ranged && size.is_some() {
    dwn_ranged(size, url, format!("{out_dir}/{out_file}"), len_fn, log).await?;
  } else {
    let mut resp = CLIENT.get(url).send().await.ok()?;

    let len = resp.content_length().unwrap_or(1);

    len_fn(len);

    let mut file = TurboFile::create(format!("{out_dir}/{out_file}"), len).ok()?;

    let mut curr = 0u64;

    let mut least = 0.0;

    while let Some(x) = resp.chunk().await.ok()? {
      curr += x.len() as u64;

      file.write_all(&x).ok()?;

      let perc = (curr as f64 * 100.0) / (len as f64);

      if perc > (least + 0.5) || perc == 100.0 {
        log(perc);
        least = perc;
      }
    }

    spawn_blocking(move || {
      file.flush().ok()?; // Ensure the file is fully written before closing

      Some(())
    })
    .await
    .ok()?;
  }

  Some(())
}

async fn dwn_ranged<F: FnMut(f64) -> (), T: FnOnce(u64) -> ()>(
  size: Option<u64>,
  url: &str,
  out: String,
  mut len: T,
  mut log: F,
) -> Option<()> {
  let size = size?;

  let mut file = spawn_blocking(move || Some(Arc::new(TurboFile::create(out, size).ok()?)))
    .await
    .ok()??;

  let url = Arc::new(url.to_string());

  let mut done = 0u64;

  len(size);

  let (tx, mut rx) = channel::<u64>(20);

  let mut pool = vec![];

  for (start, end) in divide_into_ranges(size) {
    let tx = tx.clone();

    let url = url.clone();

    let fl = file.clone();

    pool.push(async_runtime::spawn(async move {
      download_range(&url, tx, start, end, fl).await
    }));
  }

  let mut last_prog: f64 = 0.0;

  log(0.0);

  loop {
    while let Ok(x) = rx.try_recv() {
      done += x;
    }

    match rx.try_recv() {
      Err(e) => match e {
        TryRecvError::Empty => {}
        TryRecvError::Disconnected => {
          break;
        }
      },
      Ok(x) => {
        done += x;
      }
    }

    let prog = (done as f64 * 100.0) / size as f64;

    if prog != (last_prog + 0.5) || prog == 100.0 {
      last_prog = prog;
      log(prog);
    }

    if pool.iter().all(|x| match x {
      JoinHandle::Tokio(x) => x.is_finished(),
    }) {
      break;
    }

    sleep(Duration::from_millis(100)).await;
  }

  drop(tx);

  for handle in pool {
    drop(handle.await);
  }

  log(100.0);

  let file = Arc::into_inner(file).expect("This ain't happening");

  file.flush().ok()?;

  Some(())
}

async fn download_range(
  url: &str,
  tx: Sender<u64>,
  start: u64,
  end: u64,
  file: Arc<TurboFile>,
) -> Option<()> {
  let mut resp = CLIENT
    .get(url)
    .header("Range", format!("bytes={}-{}", start, end))
    .send()
    .await
    .ok()?;

  let mut collected = 0;

  while let Some(chunk) = resp.chunk().await.ok()? {
    let _ = tx.send(chunk.len() as u64).await;

    let f2 = file.clone();

    f2.write_seekable(&chunk, start + collected).ok()?;

    collected += chunk.len() as u64;
  }

  Some(())
}

fn divide_into_ranges(total_size: u64) -> Vec<(u64, u64)> {
  let mut total = (total_size / (20 * 1024 * 1024)).max(1).min((|| {
    #[cfg(not(target_os = "android"))]
    return 16;

    #[cfg(target_os = "android")]
    return 8;
  })());

  let chunk_size = total_size / total;
  let mut ranges = Vec::new();

  for i in 0..total {
    let start = i * chunk_size;

    let end = if i == (total - 1) {
      total_size - 1 // Ensure the last chunk goes to the end
    } else {
      start + chunk_size - 1
    };

    ranges.push((start, end));
  }

  ranges
}

pub async fn get_size(url: &str) -> Option<u64> {
  let resp = CLIENT.head(url).send().await.ok()?;

  let headers = resp.headers();

  headers
    .get("content-length")
    .map_or(headers.get("Content-Length"), |x| Some(x))
    .map_or(None, |x| x.to_str().ok()?.parse().ok())
}

async fn supports_ranged(url: &str) -> bool {
  let res = CLIENT.get(url).header("Range", "bytes=0-0").send().await;

  if let Ok(res) = res {
    return res.status() == StatusCode::PARTIAL_CONTENT;
  }

  false
}
