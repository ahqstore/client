use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
  borrow::Cow,
  collections::HashMap,
  env::consts::ARCH,
  time::{SystemTime, UNIX_EPOCH},
};

mod attestations;
mod install;
mod other_fields;

pub use attestations::*;
pub use install::*;
pub use other_fields::*;

use crate::api::Commits;

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Use the official ahqstore (<https://crates.io/crates/ahqstore_cli_rs>) cli\n🎯 Introduced in v1
///
/// Please note that this entire file (encoded as JSON), must be supplemented with
/// OIDC Rekor Transparency bundle log
pub struct AHQStoreApplication {
  /// The ID of the application
  pub appId: String,

  /// The name of the shortcut of the app
  pub appShortcutName: String,

  /// The name that'll be displayed in the AHQ Store
  pub appDisplayName: String,

  /// Unique ID of the author
  pub authorId: String,

  /// URLs to files the app might use
  ///
  /// Maximum 256 resources as `u8` suggests
  pub resources: HashMap<u8, Resource>,

  /// Path to ahqstore.tarball file
  pub ahqtarball: String,

  /// Install options
  pub install: InstallerOptions,

  /// App description
  pub description: String,

  /// The Github Repo associated
  pub repo: AppRepo,

  /// A version provided by the user, like AHQ Store CLI
  pub usrVersion: Option<String>,

  /// Will be automatically overriden
  pub version: u64,

  /// The Site to your app
  pub site: Option<String>,

  /// This'll be ignored unless you're ahq_verified tag which no one except AHQ Store Team has
  ///
  /// The general dev isn't meant to redistribute others' apps unless they own right to do so
  pub source: Option<String>,

  /// License type or Terms of Service Page
  pub licenseOrTos: Option<String>,

  /// These are the total number of images
  ///
  /// The minimum value must is 1 and maximum is actually 10
  /// The resources must be in the order 0,1,2,3,4,....9
  ///
  /// image id `0` signifies the icon
  /// Others signify wallpaper icons
  pub totalImages: u8,

  /// This is set to true when the app is verified by the AHQ Store Team
  ///
  /// This is a heuristic, not the proof, its only work is to show a supplementary checkmark
  /// in the web app client.
  pub verified: bool,

  /// Attestations, Provenance and security reports
  ///
  /// These are included for accountability
  pub attestations: Attestations,
}

impl AHQStoreApplication {
  pub const RESOURCE_ID_ICON: u8 = 0;
  pub const RESOURCE_IMAGE: fn(u8) -> u8 = |x| x + 1;

  pub const AHQSTORE_OFFICIAL_AUTHOR_ID: &str = "1";

  #[cfg(feature = "apps_repo")]
  /// Performs a shallow validation of the schema
  ///
  /// # WARNING
  /// This is a heuristic, not lacks vital scans
  /// Refrain from using this method solely in production
  pub fn validate_schema(&self) -> Result<String, String> {
    let mut result = String::new();

    if let Some(ver) = &self.usrVersion {
      if !ver.is_ascii() {
        result.push_str("❌ Version of not plain ascii\n");
      }

      if !ver
        .chars()
        .all(|x| x.is_alphanumeric() || x == '.' || x == '-' || x == '_')
      {
        result.push_str("❌ Version is not alphanumeric\n");
      }

      if ver.len() > 12 {
        result.push_str("❌ Version is longer than 12 characters\n");
      }
    }

    if self.appId.starts_with("l:")
      || self.appId.starts_with("f:")
      || self.appId.starts_with("w:")
      || !self.appId.chars().all(|x| x.is_alphanumeric())
    {
      result.push_str("❌ AppId must not start with l:, f:, w: and must be alphanumeric\n");
    }

    if &self.authorId != Self::AHQSTORE_OFFICIAL_AUTHOR_ID {
      if let Some(_) = self.source {
        result
          .push_str("❌ Source can't be present, your application must not reference a source\n");
      }
    }

    for val in self.resources.values() {
      if &self.authorId == Self::AHQSTORE_OFFICIAL_AUTHOR_ID {
        break;
      }

      if let AssetData::ArbitraryUrl(_) = val.asset {
        result.push_str("❌ Found instance of ArbitraryUrl!\n");
      }
    }

    if self.totalImages < 1 || self.totalImages > 10 {
      result.push_str("❌ Source specifies an invalid number of images.\n");
    }

    let mut check_id = |id: u8, platform: &str| {
      match self.resources.get(&id) {
        None => result.push_str(&format!(
          "❌ {} installer points to missing assetId {}\n",
          platform, id
        )),
        Some(res) => {
          // Logic Guard: Ensure the file intent matches the platform
          let is_win = matches!(
            res.intent,
            FileIntent::WindowsZip
              | FileIntent::WindowsInstallerExe
              | FileIntent::WindowsInstallerMsi
              | FileIntent::WindowsAHQDB
              | FileIntent::WindowsUWPMsix
          );
          let is_lin = matches!(res.intent, FileIntent::LinuxAppImage);
          let is_andy = matches!(res.intent, FileIntent::AndroidApkZip);

          if platform.contains("win") && !is_win {
            result.push_str(&format!(
              "❌ win32 installer points to a non-Windows resource (ID {})\n",
              id
            ));
          }
          if platform.contains("linux") && !is_lin {
            result.push_str(&format!(
              "❌ linux installer points to a non-Linux resource (ID {})\n",
              id
            ));
          }
          if platform.contains("android") && !is_andy {
            result.push_str(&format!(
              "❌ android installer points to a non-Android resource (ID {})\n",
              id
            ));
          }
        }
      }
    };

    if let Some(w) = &self.install.win32 {
      check_id(w.assetId, "win32");
    }
    if let Some(wa) = &self.install.winarm {
      check_id(wa.assetId, "winarm");
    }
    if let Some(l) = &self.install.linux {
      check_id(l.assetId, "linux");
    }
    if let Some(la) = &self.install.linuxArm64 {
      check_id(la.assetId, "linuxArm64");
    }
    if let Some(a) = &self.install.android {
      match a.asset {
        AndroidAssetId::Universal { assetId } => check_id(assetId, "android"),
        AndroidAssetId::AbiBased {
          aarch64,
          armv7,
          x86,
          x86_64,
        } => {
          if let Some(aarch64) = aarch64 {
            check_id(aarch64, "android-arm64");
          }
          if let Some(armv7) = armv7 {
            check_id(armv7, "android-armv7");
          }
          if let Some(x86) = x86 {
            check_id(x86, "android-x86");
          }
          if let Some(x86_64) = x86_64 {
            check_id(x86_64, "android-x86_64");
          }
        }
      }
    }

    if result.contains("❌") {
      Err(result)
    } else {
      Ok(result)
    }
  }

  pub fn export(&self) -> Option<String> {
    let mut obj = self.clone();
    obj.verified = false;
    obj.version = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time is somehow running in reverse")
      .as_secs();

    for val in obj.resources.values_mut() {
      if &obj.authorId == Self::AHQSTORE_OFFICIAL_AUTHOR_ID {
        continue;
      }

      if let AssetData::ArbitraryUrl(_) = val.asset {
        return None;
      }
    }

    if obj.totalImages < 1 || obj.totalImages > 10 {
      return None;
    }

    to_string(&obj).ok()
  }

  pub fn list_os_arch(&self) -> Vec<Platform> {
    self.install.list_os_arch()
  }

  pub fn is_supported(&self) -> bool {
    self.install.is_supported()
  }

  pub fn has_platform(&self) -> bool {
    self.install.has_platform()
  }

  /// 🎯 Introduced in v3
  pub fn get_win_options(&self) -> Option<&InstallerOptionsWindows> {
    let get_w32 = || {
      let Some(x) = &self.install.win32 else {
        return None;
      };

      Some(x)
    };

    // If we are on aarch64, we prefer to use native arm build
    let win32 = if ARCH == "aarch64" {
      if let Some(arm) = &self.install.winarm {
        arm
      } else {
        get_w32()?
      }
    } else {
      get_w32()?
    };

    Some(win32)
  }

  /// 🎯 Introduced in v2
  pub fn get_win_download(&self) -> Option<&Resource> {
    let win32 = self.get_win_options()?;
    let url = self.resources.get(&win32.assetId)?;

    match &url.intent {
      FileIntent::WindowsZip
      | FileIntent::WindowsInstallerExe
      | FileIntent::WindowsInstallerMsi
      | FileIntent::WindowsAHQDB
      | FileIntent::WindowsUWPMsix => Some(&url),
      _ => None,
    }
  }

  /// 🎯 Introduced in v2
  /// Just a clone of get_win_download for backwards compatibility
  pub fn get_win32_download(&self) -> Option<&Resource> {
    self.get_win_download()
  }

  /// 🎯 Introduced in v2
  pub fn get_win_extension<'a>(&'a self) -> Option<&'a str> {
    match &self.get_win_download()?.intent {
      FileIntent::WindowsZip => Some(".zip"),
      FileIntent::WindowsInstallerExe => Some(".exe"),
      FileIntent::WindowsInstallerMsi => Some(".msi"),
      FileIntent::WindowsAHQDB => Some(".ahqdb"),
      FileIntent::WindowsUWPMsix => Some(".msix"),
      FileIntent::Artifact { extension } => Some(extension),
      _ => None,
    }
  }

  /// 🎯 Introduced in v3
  pub fn get_linux_options(&self) -> Option<&InstallerOptionsLinux> {
    match ARCH {
      "x86_64" => self.install.linux.as_ref(),
      "aarch64" => self.install.linuxArm64.as_ref(),
      _ => {
        return None;
      }
    }
  }

  /// 🎯 Introduced in v2
  pub fn get_linux_download(&self) -> Option<&Resource> {
    let linux = self.get_linux_options()?;

    let url = self.resources.get(&linux.assetId)?;

    match &url.intent {
      FileIntent::LinuxAppImage => Some(&url),
      _ => None,
    }
  }

  /// 🎯 Introduced in v2
  pub fn get_linux_extension<'a>(&'a self) -> Option<&'a str> {
    match &self.get_linux_download()?.intent {
      FileIntent::LinuxAppImage => Some(".AppImage"),
      FileIntent::Artifact { extension } => Some(extension),
      _ => None,
    }
  }

  /// 🎯 Introduced in v3
  pub fn is_supported_android(&self, sdk: u32) -> bool {
    self.install.is_supported_android(sdk)
  }

  /// 🎯 Introduced in v2
  pub fn get_android_download(&self, sdk: u32) -> Option<&Resource> {
    let Some(android) = &self.install.android else {
      return None;
    };

    if !self.install.is_supported_android(sdk) {
      return None;
    }

    let url = self.resources.get(&match android.asset {
      AndroidAssetId::Universal { assetId } => Some(assetId),
      AndroidAssetId::AbiBased {
        aarch64,
        armv7,
        x86,
        x86_64,
      } => match ARCH {
        "aarch64" => aarch64,
        "arm" => armv7,
        "x86" => x86,
        "x86_64" => x86_64,
        _ => return None,
      },
    }?)?;

    match &url.intent {
      FileIntent::AndroidApkZip => Some(&url),
      _ => None,
    }
  }

  /// 🎯 Introduced in v2
  pub fn get_android_extension<'a>(&'a self, sdk: u32) -> Option<&'a str> {
    match self.get_android_download(sdk)?.intent {
      FileIntent::AndroidApkZip => Some(".apk"),
      _ => None,
    }
  }

  #[cfg(feature = "internet")]
  /// 🎯 Introduced in v3.14.3
  pub async fn get_resource_commit(&self, commit: &Commits, resource: u8) -> Option<Vec<u8>> {
    use crate::{api::internet::get_all_commits, get_app_asset};

    get_app_asset(&commit, &self.appId, &resource.to_string()).await
  }
}
