use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{collections::HashMap, env::consts::ARCH, time::{SystemTime, UNIX_EPOCH}};

pub mod install;
mod other_fields;

#[cfg(feature = "js")]
use tsify::*;

#[cfg(feature = "js")]
use wasm_bindgen::JsValue;

pub use install::*;
pub use other_fields::*;

use crate::api::Commits;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "Use the official ahqstore (<https://crates.io/crates/ahqstore_cli_rs>) cli\n🎯 Introduced in v1, Revamped in v2"]
#[cfg_attr(feature = "js", derive(Tsify))]
#[cfg_attr(feature = "js", tsify(into_wasm_abi, from_wasm_abi))]
pub struct AHQStoreApplication {
  /// The ID of the application
  pub appId: String,

  /// The name of the shortcut of the app
  pub appShortcutName: String,

  /// The name that'll be displayed in the app
  pub appDisplayName: String,

  /// Unique ID of the author
  pub authorId: String,

  /// The TagName of the release, MUST NOT BE LATEST
  pub releaseTagName: String,

  /// Download URLs that the app will address
  pub downloadUrls: HashMap<u8, DownloadUrl>,

  /// Install options
  pub install: InstallerOptions,

  /// App display images referencing /resources, let the cli do it
  pub displayImages: Vec<u8>,

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
  pub license_or_tos: Option<String>,

  /// These Resources will be passed to the installer, the size of all the `Vec<u8>` must not be more than 5 * 1024 * 1024 bytes (~5MiB)
  pub resources: Option<HashMap<u8, Vec<u8>>>,

  /// This is set to true when the app is verified by the AHQ Store Team
  pub verified: bool,
}

impl AHQStoreApplication {
  pub const RESOURCE_ID_ICON: u8 = 0;
  pub const RESOURCE_IMAGE: fn(u8) -> u8 = |x| x + 1;

  pub const AHQSTORE_OFFICIAL_AUTHOR_ID: &str = "1";

  #[cfg(feature = "apps_repo")]
  pub fn validate(&self) -> Result<String, String> {
    let mut result = String::new();

    result.push_str(&self.validate_resource());

    if let Some(ver) = &self.usrVersion {
      if !ver.is_ascii() {
        result.push_str("❌ Version of not plain ascii\n");
      }

      if !ver.chars().all(|x| x.is_alphanumeric()) {
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
      if &self.releaseTagName == "latest" {
        result.push_str("❌ ReleaseTagName can't be latest\n");
      }

      if let Some(_) = self.source {
        result
          .push_str("❌ Source can't be present, your application must not reference a source\n");
      }
    }

    if result.contains("❌") {
      Err(result)
    } else {
      Ok(result)
    }
  }

  #[cfg(feature = "apps_repo")]
  pub fn validate_resource(&self) -> String {
    let Some(x) = &self.resources else {
      return "❌ No Resources Present".into();
    };

    let mut total = 0;

    x.iter().for_each(|(_, v)| total += v.len());

    let mut resp = String::new();

    if total > 5 * 1024 * 1024 {
      resp.push_str("❌ Total size of all resources combined must not be more than 5MiB\n");
    } else if self.displayImages.len() > 6 {
      resp.push_str(
        "❌ A maximum of 6 images (1 icon + 5 display images) can be set in displayImages\n",
      );
    } else if x.get(&0).is_none() {
      resp.push_str("❌ Resource with id 0 must be present as it represents icon\n");
    } else if !self
      .displayImages
      .iter()
      .all(|id| x.get(&(*id + 1)).is_some())
    {
      resp.push_str("❌ Every display images should have their resource id\n");
    } else {
      resp.push_str("✅ Resources are valid\n");
    }

    resp
  }

  pub fn export(&self) -> (String, Vec<(u8, Vec<u8>)>) {
    let mut obj = self.clone();
    obj.verified = false;
    obj.version = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time is somehow running in reverse").as_secs();

    for val in obj.downloadUrls.values_mut() {
      if &obj.authorId == Self::AHQSTORE_OFFICIAL_AUTHOR_ID && &val.asset == "url" {
        continue;
      }

      let path = format!(
        "https://github.com/{}/{}/releases/download/{}/{}",
        self.repo.author, self.repo.repo, self.releaseTagName, val.asset
      );

      val.url = path;
    }
    let resources: Vec<(u8, Vec<u8>)> = std::mem::replace(&mut obj.resources, None)
      .map_or(vec![], |map| map.into_iter().collect::<Vec<_>>());

    (to_string(&obj).unwrap(), resources)
  }

  pub fn list_os_arch(&self) -> Vec<&'static str> {
    self.install.list_os_arch()
  }

  pub fn is_supported(&self) -> bool {
    self.install.is_supported()
  }

  pub fn has_platform(&self) -> bool {
    self.install.has_platform()
  }

  #[doc = "🎯 Introduced in v3"]
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

  #[doc = "🎯 Introduced in v2"]
  pub fn get_win_download(&self) -> Option<&DownloadUrl> {
    let win32 = self.get_win_options()?;
    let url = self.downloadUrls.get(&win32.assetId)?;

    match &url.installerType {
      InstallerFormat::WindowsZip
      | InstallerFormat::WindowsInstallerExe
      | InstallerFormat::WindowsInstallerMsi
      | InstallerFormat::WindowsAHQDB
      | InstallerFormat::WindowsUWPMsix => Some(&url),
      _ => None,
    }
  }

  #[doc = "🎯 Introduced in v2"]
  /// Just a clone of get_win_download for backwards compatibility
  pub fn get_win32_download(&self) -> Option<&DownloadUrl> {
    self.get_win_download()
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn get_win_extension(&self) -> Option<&'static str> {
    match self.get_win_download()?.installerType {
      InstallerFormat::WindowsZip => Some(".zip"),
      InstallerFormat::WindowsInstallerExe => Some(".exe"),
      InstallerFormat::WindowsInstallerMsi => Some(".msi"),
      InstallerFormat::WindowsAHQDB => Some(".ahqdb"),
      InstallerFormat::WindowsUWPMsix => Some(".msix"),
      _ => None,
    }
  }

  #[doc = "🎯 Introduced in v2"]
  /// Just a clone of get_win_extention for backwards compatibility
  pub fn get_win32_extension(&self) -> Option<&'static str> {
    self.get_win_extension()
  }

  #[doc = "🎯 Introduced in v3"]
  pub fn get_linux_options(&self) -> Option<&InstallerOptionsLinux> {
    match ARCH {
      "x86_64" => self.install.linux.as_ref(),
      "aarch64" => self.install.linuxArm64.as_ref(),
      "arm" => self.install.linuxArm7.as_ref(),
      _ => {
        return None;
      }
    }
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn get_linux_download(&self) -> Option<&DownloadUrl> {
    let linux = self.get_linux_options()?;

    let url = self.downloadUrls.get(&linux.assetId)?;

    match &url.installerType {
      InstallerFormat::LinuxAppImage => Some(&url),
      _ => None,
    }
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn get_linux_extension(&self) -> Option<&'static str> {
    match self.get_linux_download()?.installerType {
      InstallerFormat::LinuxAppImage => Some(".AppImage"),
      _ => None,
    }
  }

  #[doc = "🎯 Introduced in v3"]
  pub fn is_supported_android(&self, sdk: u32) -> bool {
    self.install.is_supported_android(sdk)
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn get_android_download(&self) -> Option<&DownloadUrl> {
    let Some(android) = &self.install.android else {
      return None;
    };

    let url = self.downloadUrls.get(&android.assetId)?;

    match &url.installerType {
      InstallerFormat::AndroidApkZip => Some(&url),
      _ => None,
    }
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn get_android_extension(&self) -> Option<&'static str> {
    match self.get_android_download()?.installerType {
      InstallerFormat::AndroidApkZip => Some(".apk"),
      _ => None,
    }
  }

  #[cfg(feature = "internet")]
  #[doc = "🎯 Introduced in v3"]
  #[deprecated(since = "3.14.3", note = "Use `get_resource` instead")]
  pub async fn get_resource(&self, resource: u8) -> Option<Vec<u8>> {
    use crate::api::internet::{get_all_commits, get_app_asset};

    let commit = get_all_commits(None).await.ok()?;

    get_app_asset(&commit, &self.appId, &resource.to_string()).await
  }

  #[cfg(feature = "internet")]
  #[doc = "🎯 Introduced in v3.14.3"]
  pub async fn get_resource_commit(&self, commit: &Commits, resource: u8) -> Option<Vec<u8>> {
    use crate::{api::internet::get_all_commits, get_app_asset};

    get_app_asset(&commit, &self.appId, &resource.to_string()).await
  }
}
