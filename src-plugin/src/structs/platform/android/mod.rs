use tauri::AppHandle;
use crate::AhqstoreExt;
use ahqstore_types::{AHQStoreApplication, DownloadUrl};

pub fn is_supported(handle: &AppHandle, app: &AHQStoreApplication) -> crate::Result<bool> {
  let sdk = handle.ahqstore().android_build()?.sdk;
  
  Ok(app.is_supported_android(sdk))
}

pub fn get_download(app: &AHQStoreApplication) -> (Option<&DownloadUrl>, Option<&'static str>) {
  (app.get_android_download(), app.get_android_extension())
}