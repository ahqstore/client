use ahqstore_types::internet;
use tauri::{command, AppHandle, Runtime};

pub mod downloader;

macro_rules! import {
  ($($x:tt)*) => {
    mod $($x)*;
    use $($x)* as module;
  };
}

macro_rules! get_commit {
  ($x:ident) => {
    &*$x.ahqstore().commits.lock().await
  };
}

#[cfg(target_os = "linux")]
import! { linux }

#[cfg(windows)]
import! { windows }

#[cfg(mobile)]
import! { android }

use module::*;

use crate::{AhqstoreExt, Result};

#[command]
pub async fn install<R: Runtime>(app: AppHandle<R>, app_id: &str) -> Result<()> {
  let commit = get_commit!(app);

  let app = internet::get_app(commit, app_id).await?;
  Ok(())
}
