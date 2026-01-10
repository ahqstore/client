#![allow(unused)]

use std::path::PathBuf;
use std::time::SystemTime;

use ahqstore_types::methods::OfficialManifestSource;
use ahqstore_types::{AHQStoreApplication, Commits, DevData, Home};

use anyhow::Context;

use tauri::ipc::{IpcResponse, Response};
use tauri::{command, AppHandle, Manager, Runtime};

use crate::models::*;
use crate::AhqstoreExt;

use ahqstore_types::internet;

#[cfg(desktop)]
use tauri::{
  window::{ProgressBarState, ProgressBarStatus},
  WebviewWindowBuilder,
};

use crate::error::Result;

use open as open_2;

mod download;
mod encrypt;

pub use download::*;
pub use encrypt::*;

#[command(async)]
pub(crate) async fn get_commit<R: Runtime>(app: tauri::AppHandle<R>) -> Response {
  Response::new(
    serde_json::to_string(&app.ahqstore().commits.read().await.commit)
      .unwrap()
      .into_bytes(),
  )
}

#[command(async)]
pub(crate) async fn set_scale(window: tauri::WebviewWindow, scale: f64) {
  #[cfg(desktop)]
  let _ = window.set_zoom(scale);

  #[cfg(mobile)]
  let _ = window.app_handle().ahqstore().zoom(scale as f32).await;
}

#[tauri::command]
pub async fn to_hash_uid(id: String) -> String {
  ahqstore_gh_hash::compute(id)
}

#[command(async)]
pub(crate) async fn refresh_commit(app: AppHandle) {
  app.ahqstore().refresh().await;
}

#[command(async)]
pub(crate) async fn get_all_search(app: AppHandle, query: &str) -> Result<Vec<String>> {
  Ok(
    app
      .ahqstore()
      .commits
      .read()
      .await
      .search(query)
      .map_err(|_| crate::Error::SearchError)?,
  )
}

#[command(async)]
pub(crate) async fn get_home(app: AppHandle) -> Result<Home> {
  Ok(
    internet::get_home(
      (|| {
        #[cfg(windows)]
        return OfficialManifestSource::WinGet;

        #[cfg(target_os = "linux")]
        return OfficialManifestSource::Linux;

        #[cfg(mobile)]
        return OfficialManifestSource::FDroid;
      })(),
      &app.ahqstore().commits.read().await.commit.alt,
    )
    .await?,
  )
}

#[command(async)]
pub(crate) async fn get_app(appl: AppHandle, app: &str) -> Result<AHQStoreApplication> {
  Ok(internet::get_app(&appl.ahqstore().commits.read().await.commit, app).await?)
}

#[command(async)]
pub(crate) async fn get_app_asset(appl: AppHandle, app: &str, asset: &str) -> Result<Response> {
  let bytes = internet::get_app_asset(&appl.ahqstore().commits.read().await.commit, app, asset)
    .await
    .context("")?;

  Ok(Response::new(bytes))
}

#[command(async)]
pub(crate) async fn get_dev_data(app: AppHandle, dev: &str) -> Result<DevData> {
  Ok(internet::get_dev_data(&app.ahqstore().commits.read().await.commit, dev).await?)
}

#[command(async)]
pub(crate) async fn get_devs_apps(app: AppHandle, dev: &str) -> Result<Vec<String>> {
  Ok(internet::get_devs_apps(&app.ahqstore().commits.read().await.commit, dev).await?)
}

#[command(async)]
pub(crate) fn hash_username(username: String) -> String {
  ahqstore_gh_hash::compute(username.as_str())
}

#[command(async)]
#[cfg(mobile)]
pub(crate) async fn show_code<R: Runtime>(app: AppHandle<R>, code: String) {
  app.ahqstore().show_code(code).await;
}

#[command(async)]
#[cfg(desktop)]
pub(crate) fn show_code<R: Runtime>(app: AppHandle<R>, code: String) {
  WebviewWindowBuilder::new(
    &app,
    "code",
    tauri::WebviewUrl::App(PathBuf::from(&format!("/{code}"))),
  )
  .skip_taskbar(true)
  .title("Login to GitHub")
  .inner_size(400.0, 150.0)
  .max_inner_size(400.0, 150.0)
  .min_inner_size(400.0, 150.0)
  .decorations(false)
  .always_on_top(true)
  .fullscreen(false)
  .content_protected(true)
  .maximizable(false)
  .minimizable(false)
  .closable(true)
  .focused(true)
  .build();
}

#[command(async)]
#[cfg(mobile)]
pub(crate) fn rem_code() {}

#[command(async)]
#[cfg(desktop)]
pub(crate) fn rem_code<R: Runtime>(app: tauri::AppHandle<R>) {
  app.get_webview_window("code").unwrap().destroy().unwrap()
}

#[command(async)]
pub(crate) fn is_development() -> bool {
  cfg!(debug_assertions) || env!("CARGO_PKG_VERSION").contains("-alpha")
}

#[command(async)]
pub(crate) fn open(url: String) -> Option<()> {
  match open_2::that(url) {
    Ok(_) => Some(()),
    _ => None,
  }
}

pub(crate) async fn now() -> u64 {
  use std::time::UNIX_EPOCH;

  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

#[command(async)]
#[cfg(mobile)]
pub(crate) fn set_progress(_: i32, _: Option<u64>, _: Option<u64>) {}

#[command(async)]
#[cfg(desktop)]
pub(crate) fn set_progress(
  window: tauri::WebviewWindow<tauri::Wry>,
  state: i32,
  c: Option<u64>,
  t: Option<u64>,
) {
  let progress = match (c, t) {
    (Some(c), Some(t)) => Some((c * 100) / t),
    _ => None,
  };
  let _ = window.set_progress_bar(ProgressBarState {
    progress,
    status: Some(match state {
      1 => ProgressBarStatus::Indeterminate,
      2 => ProgressBarStatus::Normal,
      4 => ProgressBarStatus::Error,
      8 => ProgressBarStatus::Paused,
      _ => ProgressBarStatus::None,
    }),
  });
}

#[command(async)]
#[cfg(desktop)]
pub(crate) fn get_linux_distro() -> Option<String> {
  #[cfg(windows)]
  return None;

  #[cfg(unix)]
  return Some(whatadistro::identify()?.name().into());
}

#[command(async)]
#[cfg(desktop)]
pub(crate) fn get_windows() -> &'static str {
  #[cfg(unix)]
  return "linux";

  #[cfg(windows)]
  return {
    if is_windows_11() {
      "11"
    } else {
      "10"
    }
  };
}

#[command(async)]
pub(crate) fn get_arch() -> &'static str {
  std::env::consts::ARCH
}

#[cfg(windows)]
#[command(async)]
pub(crate) fn is_windows_11() -> bool {
  use std::os::windows::process::CommandExt;
  use std::process::{Command, Stdio};

  let version = Command::new("cmd")
    .creation_flags(0x08000000)
    .args(["/c", "ver"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap()
    .stdout;

  let string = String::from_utf8(version).unwrap();
  let splitted = string
    .replace("\n", "")
    .replace("Microsoft Windows [", "")
    .replace("]", "");
  let version: Vec<&str> = splitted.split(".").collect();

  let version: usize = version[2].parse().unwrap_or(0);

  version >= 22000
}
