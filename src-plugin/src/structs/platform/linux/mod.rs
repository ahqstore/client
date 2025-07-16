use tauri::AppHandle;

use ahqstore_types::{AHQStoreApplication, DownloadUrl};

pub fn is_supported(_: &AppHandle, app: &AHQStoreApplication) -> crate::Result<bool> {
  Ok(app.is_supported())
}

pub fn get_download(app: &AHQStoreApplication) -> (Option<&DownloadUrl>, Option<&'static str>) {
  (app.get_linux_download(), app.get_linux_extension())
}