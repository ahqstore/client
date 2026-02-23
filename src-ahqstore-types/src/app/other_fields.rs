use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Resource {
  pub intent: FileIntent,
  pub asset: AssetData,
  pub sha: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum AssetData {
  AssetName(String),
  /// Not allowed unless you are `AHQ Store Account`
  ArbitraryUrl(String),
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FileIntent {
  // Zip File that AHQDB Installers might require
  ArtifactZip,
  // A custom single file that AHQDB Installer might require
  Artifact {
    extension: String,
  },

  /// 🎯 Stable as of v1
  WindowsZip {
    exec: String,
    scope: WindowsInstallScope,
  },

  /// 🎯 Stable as of v2
  ///
  ///
  WindowsInstallerMsi {
    /// This is an optional field that can be useful for applications
    /// whose GUID cannot be reliably fetched from the `.msi` file
    ///
    /// Some examples include applications like Firefox where the .msi
    /// file effectively calls an `.exe` setup, making our GUID search return
    /// invalid outcome.
    ///
    /// The GUID entry should have a `ahqstore` key with SZ field that has the appId
    /// for verification
    guid: Option<String>,
  },

  /// 🎯 Stable after v2
  ///
  ///
  WindowsInstallerExe {
    args: Option<Vec<String>>,
  },

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  WindowsUWPMsix {
    /// This is used incase, we are unable to correctly identify required AUMID from the AppxManifest
    ///
    /// Recommended to be present
    aumid: Option<String>,
  },

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  WindowsAHQDB {
    scope: WindowsInstallScope,
  },

  /// 🎯 Stable as of v2
  ///
  ///
  LinuxAppImage,

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  AndroidApkZip {
    min_sdk: u32,
  },
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum WindowsInstallScope {
  User,
  Machine,
  Both,
}

impl Display for FileIntent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match &self {
        FileIntent::ArtifactZip => "Zip Artifact",
        FileIntent::Artifact { .. } => "Artifact",
        FileIntent::WindowsZip { .. } => "Windows Zip",
        FileIntent::WindowsInstallerExe { .. } => "Windows Installer Exe",
        FileIntent::WindowsInstallerMsi { .. } => "Windows Installer Msi",
        FileIntent::WindowsAHQDB { .. } => "Windows AHQDB Installer",
        FileIntent::WindowsUWPMsix { .. } => "UWP Windows Msix Package",
        FileIntent::LinuxAppImage => "Linux App Image",
        FileIntent::AndroidApkZip { .. } => "Universal Android Apk Zip Package",
      }
    )
  }
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct AppRepo {
  pub provider: RepositoryProvider,

  /// Your Author username
  ///
  /// For GitHub, its username
  pub author: String,
  pub repo: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepositoryProvider {
  GitHub,
}
