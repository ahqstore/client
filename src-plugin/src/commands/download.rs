use std::sync::Arc;

use tauri::{ipc::Channel, AppHandle, Manager};

use crate::{structs::platform::downloader::{self, get_size}, DownloadEvent, Result};

use anyhow::Context;

#[tauri::command(async)]
pub async fn download(app: AppHandle, url: &str, name: &str, path: &str, channel: Channel<DownloadEvent>) -> Result<()> {
  let temp = app.path().app_cache_dir()
    .context("Could not get cache dir")?
    .into_os_string()
    .into_string()
    .ok()
    .context("Unable to create string")?;

  let channel = Arc::new(channel);

  channel.send(DownloadEvent::Started {
    length: get_size(url).await.unwrap_or(1)
  });

  let ch = channel.clone();
  downloader::download(url, name, path, &temp, false, move |perc| {
    ch.send(DownloadEvent::Progress { progress: perc });
  }).await.context("Could not download")?;

  channel.send(DownloadEvent::Finished {}).unwrap();

  drop(channel);

  Ok(())
}