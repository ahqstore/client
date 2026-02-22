use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Resource {
  pub intent: FileIntent,
  pub asset: AssetData,
  pub sha: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum AssetData {
  AssetName(String),
  /// Not allowed unless you are `AHQ Store Account`
  ArbitraryUrl(String),
}

#[cfg_attr(feature = "export", derive(specta::Type))]
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
  WindowsZip,

  /// 🎯 Stable as of v2
  ///
  ///
  WindowsInstallerMsi,

  /// 🎯 Stable after v2
  ///
  ///
  WindowsInstallerExe,

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  WindowsUWPMsix,

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  WindowsAHQDB,

  /// 🎯 Stable as of v2
  ///
  ///
  LinuxAppImage,

  /// 🔬 Planned in AHQ Store NEO
  ///
  ///
  AndroidApkZip,
}

impl Display for FileIntent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match &self {
        FileIntent::ArtifactZip => "Zip Artifact",
        FileIntent::Artifact { .. } => "Artifact",
        FileIntent::WindowsZip => "Windows Zip",
        FileIntent::WindowsInstallerExe => "Windows Installer Exe",
        FileIntent::WindowsInstallerMsi => "Windows Installer Msi",
        FileIntent::WindowsAHQDB => "Windows AHQDB Installer",
        FileIntent::WindowsUWPMsix => "UWP Windows Msix Package",
        FileIntent::LinuxAppImage => "Linux App Image",
        FileIntent::AndroidApkZip => "Universal Android Apk Zip Package",
      }
    )
  }
}

#[cfg_attr(feature = "export", derive(specta::Type))]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepositoryProvider {
  GitHub
}