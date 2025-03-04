#[cfg(feature = "js")]
use wasm_bindgen::prelude::wasm_bindgen;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct DownloadUrl {
  pub installerType: InstallerFormat,
  pub asset: String,

  /// This will be based on asset and releaseId
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum InstallerFormat {
  #[doc = "🎯 Stable as of v1"]
  WindowsZip,

  #[doc = "🎯 Stable as of v2\n\n"]
  WindowsInstallerMsi,

  #[doc = "🎯 Stable after v2\n\n"]
  WindowsInstallerExe,

  #[doc = "🔬 Planned in AHQ Store NEO\n\n"]
  WindowsUWPMsix,

  #[doc = "🎯 Stable as of v2\n\n"]
  LinuxAppImage,

  #[doc = "🔬 Planned in AHQ Store NEO\n\n"]
  AndroidApkZip,
}

impl Display for InstallerFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match &self {
        InstallerFormat::WindowsZip => "Windows Zip",
        InstallerFormat::WindowsInstallerExe => "Windows Installer Exe",
        InstallerFormat::WindowsInstallerMsi => "Windows Installer Msi",
        InstallerFormat::WindowsUWPMsix => "UWP Windows Msix Package",
        InstallerFormat::LinuxAppImage => "Linux App Image",
        InstallerFormat::AndroidApkZip => "Universal Android Apk Zip Package",
      }
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct AppRepo {
  /// author must be your GitHub username or username of an org where you're a "visible" member
  pub author: String,
  pub repo: String,
}
