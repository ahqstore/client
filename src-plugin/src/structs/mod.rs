#![allow(unused)]

use ahqstore_types::{get_all_commits, Commits};
use serde::de::DeserializeOwned;
use tauri::{
  async_runtime::{self, Mutex},
  plugin::PluginApi,
  AppHandle, Runtime,
};

#[cfg(mobile)]
use tauri::plugin::PluginHandle;

use crate::models::*;

pub(crate) mod platform;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Ahqstore<R>> {
  let commits = Mutex::new(async_runtime::block_on(async {
    get_all_commits(None).await
  })?);

  Ok(Ahqstore {
    #[cfg(desktop)]
    handle: app.clone(),
    #[cfg(mobile)]
    handle: _api.register_android_plugin("com.plugin.ahqstore", "AHQStorePlugin")?,
    commits,
  })
}

/// Access to the ahqstore APIs.
pub struct Ahqstore<R: Runtime> {
  #[cfg(desktop)]
  pub(crate) handle: AppHandle<R>,
  #[cfg(mobile)]
  pub(crate) handle: PluginHandle<R>,
  pub commits: Mutex<Commits>,
}

impl<R: Runtime> Ahqstore<R> {
  pub async fn refresh(&self) -> crate::Result<()> {
    let mut lock = self.commits.lock().await;

    *lock = async_runtime::block_on(async { get_all_commits(None).await })?;

    Ok(())
  }

  #[cfg(mobile)]
  pub fn show_code(&self, code: String) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin("showCode", ShowCodeRequest { value: code })
      .map_err(Into::into)
  }

  #[cfg(mobile)]
  pub fn zoom(&self, zoom: f32) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin("zoom", ZoomRequest { zoom: zoom * 100.0 })
      .map_err(Into::into)
  }
}
