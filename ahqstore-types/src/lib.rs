use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::fs::read;

pub type AppId = String;
pub type Str = String;
pub type AppData = (String, String);
pub type RefId = u64;

pub mod app;
pub use app::*;

pub mod api;
pub use api::*;

/// **You should use cli**
/// ```sh
/// cargo install ahqstore_cli_rs
/// ```

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
  pub sha: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prefs {
  launch_app: bool,
  install_apps: bool,
}

impl Prefs {
  pub fn get(path: &str) -> Option<Vec<u8>> {
    read(&path).ok()
  }

  pub fn str_to(s: &str) -> Option<Self> {
    from_str(s).ok()
  }

  pub fn convert(&self) -> Option<String> {
    to_string(self).ok()
  }

  pub fn default() -> Self {
    Self {
      launch_app: true,
      install_apps: true,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
  GetSha(RefId),

  GetApp(RefId, AppId),
  InstallApp(RefId, AppId),
  UninstallApp(RefId, AppId),

  ListApps(RefId),

  RunUpdate(RefId),
  UpdateStatus(RefId),

  GetPrefs(RefId),
  SetPrefs(RefId, Prefs),

  AddPkg(RefId, String),
}

impl Command {
  pub fn try_from<T: AsRef<str>>(value: T) -> Option<Self> {
    serde_json::from_str(value.as_ref()).ok()
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reason {
  UnknownData(RefId),

  Unauthenticated,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorType {
  GetAppFailed(RefId, AppId),
  AppPlatformNoSupport(RefId, AppId),
  AppInstallError(RefId, AppId),
  AppUninstallError(RefId, AppId),
  PrefsError(RefId),
  PkgError(RefId),
  GetSHAFailed(RefId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
  Ready,

  Error(ErrorType),

  SHAId(RefId, String),

  Disconnect(Reason),

  AppData(RefId, AppId, AHQStoreApplication),
  AppDataUrl(RefId, AppId, String),

  ListApps(RefId, Vec<AppData>),

  DownloadStarted(RefId, AppId),
  DownloadProgress(RefId, AppId, [u64; 2]),
  Installing(RefId, AppId),
  Installed(RefId, AppId),

  UninstallStarting(RefId, AppId),
  Uninstalled(RefId, AppId),

  Prefs(RefId, Prefs),
  PrefsSet(RefId),

  DownloadPkgProg(RefId, [u64; 2]),
  InstallPkg(RefId),
  InstalledPkg(RefId),

  TerminateBlock(RefId),
}

impl Response {
  pub fn as_msg(msg: Self) -> Vec<u8> {
    to_string_pretty(&msg)
      .unwrap_or("ERRR".to_string())
      .into_bytes()
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthPing {
  pub process: usize,
}

impl AuthPing {
  pub fn from<T: AsRef<str>>(value: T) -> Option<Self> {
    let string = value.as_ref();

    serde_json::from_str(string).ok()
  }
}
