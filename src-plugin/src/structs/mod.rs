#![allow(unused)]

use std::sync::Arc;

use ahqstore_types::{get_all_commits, Commits};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tauri::{
  async_runtime::{self, RwLock},
  plugin::PluginApi,
  AppHandle, Runtime,
};

#[cfg(mobile)]
use tauri::plugin::PluginHandle;
use tokio::sync::Mutex;

use crate::{models::*, structs::daemon::{IPCSend, initialize}};

pub(crate) mod platform;
pub(crate) mod daemon;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Ahqstore<R>> {
  let commits = Arc::new(RwLock::new(async_runtime::block_on(async {
    get_all_commits(None).await
  })?));

  Ok(Ahqstore {
    #[cfg(desktop)]
    handle: app.clone(),
    #[cfg(mobile)]
    handle: _api.register_android_plugin("com.plugin.ahqstore", "AHQStorePlugin")?,
    commits,
    send_to_ipc: Mutex::new(None),
    preferences: Arc::new(RwLock::new(Preferences::init(app)))
  })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AutoUpdate {
  Never,
  CheckOnly,
  UpdateDuringUnmeteredWifi,
  UpdateDuringMeteredWifi,
  Always
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
  #[serde(rename = "autoUpdate")]
  pub auto_update: AutoUpdate
}

impl Preferences {
  pub fn init<R: Runtime>(h: &AppHandle<R>) -> Self {
    #[cfg(desktop)]
    {
      use std::fs::read_to_string;
      use tauri::Manager;

      let mut set_path = h.path().app_local_data_dir().expect("Impossible error");
      
      set_path.push("config.json");

      return serde_json::from_str(&read_to_string(&set_path).unwrap_or_default()).unwrap_or(
        Self {
          auto_update: AutoUpdate::CheckOnly
        }
      );
    }

    #[cfg(mobile)]
    // Only a polyfill
    return Self {
      auto_update: AutoUpdate::CheckOnly
    };
  }
}

/// Access to the ahqstore APIs.
pub struct Ahqstore<R: Runtime> {
  #[cfg(desktop)]
  pub(crate) handle: AppHandle<R>,
  #[cfg(mobile)]
  pub(crate) handle: PluginHandle<R>,
  pub preferences: Arc<RwLock<Preferences>>,
  pub commits: Arc<RwLock<Commits>>,
  pub send_to_ipc: Mutex<Option<IPCSend>>,
}

impl<R: Runtime> Ahqstore<R> {
  pub fn init(&self, hwnd: AppHandle<R>) {
    let mut lock = self.send_to_ipc.blocking_lock();

    if lock.is_none() {
      *lock = Some(initialize(hwnd, self.commits.clone()));
    }
  }

  pub async fn refresh(&self) -> crate::Result<()> {
    let mut lock = self.commits.write().await;

    *lock = get_all_commits(None).await?;

    Ok(())
  }

  #[cfg(mobile)]
  pub fn android_build(&self) -> crate::Result<AndroidBuildOutput> {
    self
      .handle
      .run_mobile_plugin("getAndroidBuild", ())
      .map_err(Into::into)
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
