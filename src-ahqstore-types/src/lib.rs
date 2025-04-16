#![allow(
  dead_code,
  unused_imports,
  non_local_definitions,
  reason = "Conditional compilation"
)]

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::fs::read;

#[cfg(feature = "js")]
use tsify::*;
#[cfg(feature = "js")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg_attr(feature = "js", declare)]
pub type AppId = String;
#[cfg_attr(feature = "js", declare)]
pub type Str = String;
#[cfg_attr(feature = "js", declare)]
pub type AppData = (String, String);
#[cfg_attr(feature = "js", declare)]
pub type RefId = u64;

#[cfg_attr(feature = "js", declare)]
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

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct Prefs {
  pub launch_app: bool,
  pub install_apps: bool,
  pub auto_update_apps: bool,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
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
      launch_app: true,
      install_apps: true,
      auto_update_apps: true,
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "js", derive(Tsify))]
#[cfg_attr(feature = "js", tsify(into_wasm_abi, from_wasm_abi))]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", derive(Tsify))]
#[cfg_attr(feature = "js", tsify(into_wasm_abi, from_wasm_abi))]
pub enum UpdateStatusReport {
  Disabled,
  UpToDate,
  Checking,
  Updating,
}

impl Clone for Commits {
  fn clone(&self) -> Self {
    Self {
      ahqstore: self.ahqstore.clone(),
      alt: self.alt.clone(),
    }
  }
}

impl From<&Commits> for Commits {
  fn from(value: &Commits) -> Self {
    value.clone()
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
