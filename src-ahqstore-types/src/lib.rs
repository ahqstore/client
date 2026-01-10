#![allow(
  dead_code,
  unused_imports,
  non_local_definitions,
  reason = "Conditional compilation"
)]

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::fs::read;

pub type AppId = String;
pub type Str = String;
pub type AppData = (String, String);
pub type RefId = u64;

pub type Success = bool;

pub mod app;
pub use app::*;

pub mod api;
pub use api::*;

pub mod data;
pub use data::*;

pub mod winget;

/// **You should use cli**
/// ```sh
/// cargo install ahqstore_cli_rs
/// ```
/// or visit app / api sub module
///
/// This Module:
/// This module lists the standard commands & types that AHQ Store sends to AHQ Store Service

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize)]
pub struct StatusUpdateData {
  pub queue: &'static [QueuedApp],
  #[serde(rename = "supportsUpdate")]
  pub supports_update: bool,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize)]
pub struct QueuedApp {
  pub id: String,
  pub status: AppUpdateInstallStatus,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum AppUpdateInstallStatus {
  /// { "status": "Pending" }
  Pending,
  /// { "status": "Cancelled" }
  Cancelled,
  /// { "status": "Downloading", "progress": 100.0 }
  Downloading { progress: f64 },
  /// { "status": "Installing" }
  Installing,
  /// { "status": "Updating" }
  Updating,
  /// { "status": "Uninstalling" }
  Uninstalling,
  /// { "status": "Done" }
  Done,
}

/// PREFERENCES

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug)]
pub enum UpdateStrategy {
  Never,
  CheckOnly,
  DownloadInstall_UnmeteredWifi,
  DownloadInstall_Wifi,
  DownloadInstall,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Prefs {
  pub update: UpdateStrategy,
}

impl Prefs {
  pub fn get(path: &str) -> Option<Vec<u8>> {
    read(&path).ok()
  }

  pub fn str_to(s: &str) -> Option<Prefs> {
    from_str(s).ok()
  }

  pub fn convert(&self) -> Option<String> {
    to_string(self).ok()
  }

  pub fn default() -> Prefs {
    Prefs {
      update: UpdateStrategy::CheckOnly,
    }
  }
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Clone)]

pub enum AppStatus {
  Pending,
  Downloading,
  AVScanning,
  Installing,
  Uninstalling,
  InstallSuccessful,
  UninstallSuccessful,
  NotSuccessful,
  AVFlagged,
}

impl Serialize for AppStatus {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(match self {
      AppStatus::Pending => "Pending...",
      AppStatus::Downloading => "Downloading...",
      AppStatus::Installing => "Installing...",
      AppStatus::Uninstalling => "Uninstalling...",
      AppStatus::InstallSuccessful => "Installed",
      AppStatus::UninstallSuccessful => "Uninstalled",
      AppStatus::NotSuccessful => "Error!",
      AppStatus::AVScanning => "Scanning for Viruses!",
      AppStatus::AVFlagged => "Flagged as Malicious!",
    })
  }
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Debug, Clone)]

pub enum UpdateStatusReport {
  Disabled,
  UpToDate,
  Checking,
  Updating,
}

#[cfg(test)]
mod tests {
  #[test]
  #[cfg(feature = "export")]
  fn export() {
    _ = specta::export::ts("./pkg/ahqstore.types.d.ts").unwrap();
    _ = specta::export::ts("./types/ahqstore.types.d.ts").unwrap();
  }
}
