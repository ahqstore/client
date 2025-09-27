use std::{fs, thread};

use ahqstore_types::internet;
use anyhow::{Context, Error as AnyError};
use tauri::{command, ipc::Channel, AppHandle};
use tokio::sync::oneshot;

pub mod common;
pub mod downloader;

macro_rules! import {
  ($($x:tt)*) => {
    mod $($x)*;
    use $($x)* as platform;
  };
}

macro_rules! get_commit {
  ($x:ident) => {
    &*$x.ahqstore().commits.read().await
  };
}

#[cfg(target_os = "linux")]
import! { linux }

#[cfg(windows)]
import! { windows }

#[cfg(mobile)]
import! { android }

macro_rules! err {
  ($x:expr) => {
    Err(Error::AHQStore(AnyError::msg($x)))
  };
}

use crate::{
  structs::platform::common::{clear_dwnl, downloads, dwnl_tmp},
  AhqstoreExt, DownloadEvent, Error, InstallStat, Result,
};

#[command]
pub async fn download(
  handle: AppHandle,
  app_id: &str,
  prog: Channel<DownloadEvent>,
) -> Result<Channel<()>> {
  let commit = get_commit!(handle);

  let app = internet::get_app(commit, app_id).await?;

  let hwnd = handle.clone();
  let stat = tokio::spawn(async move {
    if !platform::is_supported(&hwnd, &app)? {
      return Err(Error::UnsupportedPlatform);
    }

    let (Some(url), Some(extension)) = platform::get_download(&app) else {
      return err!("Could not get data");
    };

    let turbo = cfg!(mobile);

    let dwnl = downloads(&hwnd)?;
    let dir_tmp = dwnl_tmp(&hwnd)?;

    downloader::download(
      &url.url,
      &format!("{}{}", &app.appId, extension),
      { dwnl.to_str().context("Invalid String")? },
      { dir_tmp.to_str().context("Invalid String")? },
      turbo,
      |length| {
        prog.send(DownloadEvent::Started { length });
      },
      |progress| {
        prog.send(DownloadEvent::Progress {
          progress,
        });
      },
    )
    .await
    .context("Unable to download")?;

    prog.send(DownloadEvent::Finished {  });

    Ok(())
  });

  let app_id = app_id.to_string();

  let handle = handle.clone();
  let handler = Channel::new(move |body| {
    if !stat.is_finished() {
      stat.abort();

      // Cleaning logic
      {
        clear_dwnl(&handle);
      }

      Ok(())
    } else {
      Err(tauri::Error::Anyhow(AnyError::msg("The download task is already complete")))
    }
  });

  Ok(handler)
}
