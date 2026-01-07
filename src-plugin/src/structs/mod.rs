#![allow(unused)]

use std::sync::Arc;

use ahqstore_types::{get_all_commits, Commits};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::{
  async_runtime::{self, spawn, RwLock},
  plugin::PluginApi,
  AppHandle, Runtime,
};

#[cfg(mobile)]
use tauri::plugin::PluginHandle;
use tokio::sync::Mutex;

use crate::{
  models::*,
  structs::{
    daemon::{initialize, IPCSend},
    search::search_daemon,
  },
};

pub(crate) mod daemon;
pub(crate) mod platform;
pub(crate) mod search;

use search::CommitSearchIndex;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Ahqstore<R>> {
  #[cfg(desktop)]
  let commits = Arc::new(RwLock::new(async_runtime::block_on(async {
    Ok::<CommitSearchIndex, anyhow::Error>(CommitSearchIndex {
      commit: get_all_commits(None).await?,
      meta: None,
    })
  })?));

  #[cfg(mobile)]
  let mobile = _api.register_android_plugin("com.plugin.ahqstore", "AHQStorePlugin")?;

  #[cfg(mobile)]
  let commits = Arc::new(RwLock::new(
    mobile
      .run_mobile_plugin::<Commits>("getCommit", ())
      .map_err(Into::<crate::Error>::into)?,
  ));

  #[cfg(mobile)]
  let prefs = Preferences::init(app, &mobile)?;

  #[cfg(desktop)]
  let prefs = Preferences::init(app)?;

  Ok(Ahqstore {
    #[cfg(desktop)]
    handle: app.clone(),
    #[cfg(mobile)]
    handle: mobile,
    commits,
    send_to_ipc: Mutex::new(None),
    preferences: Arc::new(RwLock::new(prefs)),
  })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AutoUpdate {
  Never,
  CheckOnly,
  UpdateDuringUnmeteredWifi,
  UpdateDuringMeteredWifi,
  Always,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
  #[serde(rename = "autoUpdate")]
  pub auto_update: AutoUpdate,
}

impl Preferences {
  #[cfg(mobile)]
  pub fn init<R: Runtime>(h: &AppHandle<R>, m: &PluginHandle<R>) -> crate::Result<Self> {
    // Only a polyfill
    return Ok(Self {
      auto_update: AutoUpdate::CheckOnly,
    });
  }

  #[cfg(desktop)]
  pub fn init<R: Runtime>(h: &AppHandle<R>) -> crate::Result<Self> {
    use std::fs::read_to_string;
    use tauri::Manager;

    let mut set_path = h.path().app_local_data_dir()?;

    set_path.push("config.json");

    return Ok(
      serde_json::from_str(&read_to_string(&set_path).unwrap_or_default()).unwrap_or(Self {
        auto_update: AutoUpdate::CheckOnly,
      }),
    );
  }
}

/// Access to the ahqstore APIs.
pub struct Ahqstore<R: Runtime> {
  #[cfg(desktop)]
  pub(crate) handle: AppHandle<R>,
  #[cfg(mobile)]
  pub(crate) handle: PluginHandle<R>,
  pub preferences: Arc<RwLock<Preferences>>,
  pub commits: Arc<RwLock<CommitSearchIndex>>,
  pub send_to_ipc: Mutex<Option<IPCSend>>,
}

impl<R: Runtime> Ahqstore<R> {
  pub fn init(&self, hwnd: AppHandle<R>) {
    let mut lock = self.send_to_ipc.blocking_lock();

    let hwnd2 = hwnd.clone();

    if lock.is_none() {
      *lock = Some(initialize(hwnd, self.commits.clone()));
    }

    let lck = self.commits.clone();

    spawn(async move {
      search_daemon(hwnd2, lck).await;
    });
  }

  #[cfg(desktop)]
  pub async fn refresh(&self) -> crate::Result<()> {
    let mut lock = self.commits.write().await;

    *lock = CommitSearchIndex {
      commit: get_all_commits(None).await?,
      meta: None,
    };

    Ok(())
  }

  #[cfg(mobile)]
  pub async fn refresh(&self) -> crate::Result<()> {
    let mut lock = self.commits.write().await;

    *lock = self.refresh_commit_android().await?;

    Ok(())
  }

  #[cfg(mobile)]
  pub async fn refresh_commit_android(&self) -> crate::Result<Commits> {
    self
      .handle
      .run_mobile_plugin_async("updateCommit", ())
      .await
      .map_err(Into::into)
  }

  #[cfg(mobile)]
  pub fn android_build(&self) -> crate::Result<AndroidBuildOutput> {
    self
      .handle
      .run_mobile_plugin("getAndroidBuild", ())
      .map_err(Into::into)
  }

  #[cfg(mobile)]
  pub async fn show_code(&self, code: String) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin_async("showCode", ShowCodeRequest { value: code })
      .await
      .map_err(Into::into)
  }

  #[cfg(mobile)]
  pub async fn zoom(&self, zoom: f32) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin_async("zoom", ZoomRequest { zoom: zoom * 100.0 })
      .await
      .map_err(Into::into)
  }
}
