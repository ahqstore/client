use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct DownloadUrl {
  pub installerType: InstallerFormat,
  pub asset: String,

  /// This will be based on asset and releaseId
  pub url: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum InstallerFormat {
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

impl Display for InstallerFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match &self {
        InstallerFormat::WindowsZip => "Windows Zip",
        InstallerFormat::WindowsInstallerExe => "Windows Installer Exe",
        InstallerFormat::WindowsInstallerMsi => "Windows Installer Msi",
        InstallerFormat::WindowsAHQDB => "Windows AHQDB Installer",
        InstallerFormat::WindowsUWPMsix => "UWP Windows Msix Package",
        InstallerFormat::LinuxAppImage => "Linux App Image",
        InstallerFormat::AndroidApkZip => "Universal Android Apk Zip Package",
      }
    )
  }
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct AppRepo {
  /// author must be your GitHub username or username of an org where you're a "visible" member
  pub author: String,
  pub repo: String,
}
