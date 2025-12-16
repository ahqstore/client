use tauri::AppHandle;

use ahqstore_types::{AHQStoreApplication, DownloadUrl};

pub mod notify;
pub mod install;
pub mod network;

pub fn is_supported(_: &AppHandle, app: &AHQStoreApplication) -> crate::Result<bool> {
  Ok(app.is_supported())
}

pub fn get_download(app: &AHQStoreApplication) -> (Option<&DownloadUrl>, Option<&'static str>) {
  (app.get_win_download(), app.get_win_extension())
}