#![allow(
  dead_code,
  unused_imports,
  non_local_definitions,
  reason = "Conditional compilation"
)]

//! **You should use cli**
//! ```sh
//! cargo install ahqstore_cli_rs
//! ```
//! or visit app / api sub module
//!
//! This Module:
//! - This module lists the standard commands & types that AHQ Store sends to AHQ Store Service
//! - Defines schemas for the AHQ Store File Formats

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::{borrow::Cow, fs::read, sync::Arc};
use tokio::task::JoinHandle;

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

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize)]
pub struct StatusUpdateData {
  #[cfg(not(feature = "export"))]
  pub queue: Box<[QueuedAppData]>,
  #[cfg(feature = "export")]
  pub queue: Vec<QueuedAppData>,
  #[serde(rename = "supportsUpdate")]
  pub disable_update: bool,
  #[serde(rename = "queueOverflow")]
  pub overflow: bool,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize, Clone)]
pub enum AppActionIntent {
  Install,
  Uninstall,
  Update,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize, Clone)]
pub struct QueuedAppData {
  #[cfg(not(feature = "export"))]
  pub id: Arc<str>,
  #[cfg(feature = "export")]
  pub id: String,
  #[cfg_attr(feature = "export", specta(type = u32))]
  pub transaction: u64,
  pub status: AppUpdateInstallStatus,
  pub intent: AppActionIntent,
}

impl QueuedAppData {
  pub fn from(data: &QueuedApp) -> Self {
    Self {
      id: data.id.clone(),
      transaction: data.transaction,
      intent: data.intent.clone(),
      status: data.status.clone(),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct QueuedAppUpdate {
  pub transaction: u64,
  pub status: AppUpdateInstallStatus,
}

#[derive(Debug, Serialize)]
pub struct QueuedApp {
  #[cfg(not(feature = "export"))]
  pub id: Arc<str>,
  #[cfg(feature = "export")]
  pub id: String,
  pub transaction: u64,
  pub status: AppUpdateInstallStatus,
  pub intent: AppActionIntent,
  #[serde(skip)]
  pub task: Option<JoinHandle<()>>,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "status")]
pub enum AppUpdateInstallStatus {
  /// { "status": "Pending" }
  Pending,
  /// { "status": "PendingUserAction" }
  PendingUserAction,
  /// { "status": "Cancelled" }
  Cancelled {
    // Shows for 5seconds
    #[serde(skip)]
    time: u64,
  },
  /// { "status": "Downloading", "progress": 100.0 }
  Downloading { progress: f64 },
  /// { "status": "AVSCanning" }
  AVScanning,
  /// { "status": "PendingInstall" }
  PendingInstall,
  /// { "status": "Installing", "progress": null }
  ///
  /// OR
  ///
  /// { "status": "Installing", "progress": 30.0 }
  Installing { progress: Option<f64> },
  /// { "status": "MoreDwnlNeeded" }
  MoreDwnlNeeded {
    // Total progress
    progress: f64,
    current: usize,
    items: usize,
  },
  /// { "status": "CopyingFiles", "percentage": 67, "total": 100 }
  CopyingFiles { percentage: f64, total: usize },
  /// { "status": "Finalizing" }
  Finalizing,
  /// { "status": "Updating" }
  Updating,
  /// { "status": "Uninstalling" }
  Uninstalling,
  /// { "status": "Successful" }
  Successful {
    // This is a time delta used by us to auto prune >2s entries
    #[serde(skip)]
    time: u64,
  },
  /// { "status": "Error", "err": "ERROR DESC" }
  Error {
    err: Cow<'static, str>,
    // >10s time delta
    #[serde(skip)]
    time: u64,
  },
}

#[cfg(test)]
mod tests {
  #[test]
  #[cfg(feature = "export")]
  fn export() {
    // Since the `u64`s represent time deltas, and counters. We can safely treat as JS Number.
    let export =
      specta::ts::ExportConfiguration::new().bigint(specta::ts::BigIntExportBehavior::Number);

    _ = specta::export::ts_with_cfg("./pkg/ahqstore.types.d.ts", &export).unwrap();
    _ = specta::export::ts_with_cfg("./types/ahqstore.types.d.ts", &export).unwrap();
  }
}
