//! Unified API for all officially declared AHQ Store Parsable Repos
//!
//! Currently URL Declared AHQ Store Parsable Repos Officially Used are
//! - 🛍️ AHQ Store Official Community Repository (AHQStore)
//! - 🪟 Microsoft Winget Community Repository (WinGet)
//! - 🫓 AHQ Store Linux AppImage Repository (AHQStore)
//! - 📱 FDroid Android Community Repository (FDroid)

use std::env::consts::OS;

use crate::AHQStoreApplication;

use super::{
  ahqstore::{
    AHQSTORE_APPS_DEV, AHQSTORE_APP_ASSET_URL, AHQSTORE_APP_URL, AHQSTORE_DEV_DATA, AHQSTORE_HOME,
    AHQSTORE_MAP, AHQSTORE_SEARCH, AHQSTORE_TOTAL,
  },
  fdroid::{
    FDROID_APPS_DEV, FDROID_APP_ASSET_URL, FDROID_APP_URL, FDROID_DEV_DATA, FDROID_HOME,
    FDROID_MAP, FDROID_SEARCH, FDROID_TOTAL,
  },
  linux::{
    self, LINUX_APPS_DEV, LINUX_APP_ASSET_URL, LINUX_APP_URL, LINUX_DEV_DATA, LINUX_HOME,
    LINUX_MAP, LINUX_SEARCH, LINUX_TOTAL,
  },
  methods::{self, OfficialManifestSource, Store},
  winget::{
    WINGET_APPS_DEV, WINGET_APP_ASSET_URL, WINGET_APP_URL, WINGET_DEV_DATA, WINGET_HOME,
    WINGET_MAP, WINGET_SEARCH, WINGET_TOTAL,
  },
  Home, SearchEntry,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Commits {
  pub ahqstore: String,
  pub alt: String,
}

pub async fn get_all_commits(token: Option<String>) -> Result<Commits> {
  let ahqstore = methods::get_commit(Store::AHQStore, token.as_ref())
    .await
    .context("http")?;

  #[cfg(windows)]
  let winget = methods::get_commit(Store::WinGet, token.as_ref())
    .await
    .context("http")?;

  #[cfg(target_os = "android")]
  let fdroid = methods::get_commit(Store::FDroid, token.as_ref())
    .await
    .context("http")?;

  #[cfg(target_os = "linux")]
  let l = methods::get_commit(Store::FDroid, token.as_ref())
    .await
    .context("http")?;

  Ok(Commits {
    ahqstore,
    #[cfg(not(any(windows, target_os = "android", target_os = "linux")))]
    alt: "".to_string(),
    #[cfg(windows)]
    alt: winget,
    #[cfg(target_os = "android")]
    alt: fdroid,
    #[cfg(target_os = "linux")]
    alt: l,
  })
}

#[allow(unreachable_patterns)]
pub async fn get_total_maps_by_source(
  source: OfficialManifestSource,
  commit: &str,
) -> Result<usize> {
  let total = match source {
    OfficialManifestSource::AHQStore => &*AHQSTORE_TOTAL,
    OfficialManifestSource::WinGet => &*WINGET_TOTAL,
    OfficialManifestSource::FDroid => &*FDROID_TOTAL,
    OfficialManifestSource::Linux => &*LINUX_TOTAL,
    _ => {
      return Err(anyhow!("source not supported"));
    }
  };
  methods::get_total_maps(total, commit).await.context("")
}

#[allow(unreachable_patterns)]
pub async fn get_home(source: OfficialManifestSource, commit: &str) -> Result<Home> {
  let home = match source {
    OfficialManifestSource::AHQStore => &*AHQSTORE_HOME,
    OfficialManifestSource::WinGet => &*WINGET_HOME,
    OfficialManifestSource::FDroid => &*FDROID_HOME,
    OfficialManifestSource::Linux => &*LINUX_HOME,
    _ => {
      return Err(anyhow!("source not supported"));
    }
  };

  methods::get_home(home, commit).await.context("")
}

pub async fn get_search_by_source(
  source: OfficialManifestSource,
  commit: &str,
  id: &str,
) -> Result<Vec<super::SearchEntry>> {
  let search = match source {
    OfficialManifestSource::AHQStore => &*AHQSTORE_SEARCH,
    OfficialManifestSource::WinGet => &*WINGET_SEARCH,
    OfficialManifestSource::FDroid => &*FDROID_SEARCH,
    OfficialManifestSource::Linux => &*LINUX_SEARCH,
  };

  methods::get_search(search, commit, id).await.context("")
}

pub async fn get_all_maps_by_source(
  source: OfficialManifestSource,
  commit: &str,
) -> Result<super::MapData> {
  let (total, map) = match source {
    OfficialManifestSource::AHQStore => (&*AHQSTORE_TOTAL, &*AHQSTORE_MAP),
    OfficialManifestSource::WinGet => (&*WINGET_TOTAL, &*WINGET_MAP),
    OfficialManifestSource::FDroid => (&*FDROID_TOTAL, &*FDROID_MAP),
    OfficialManifestSource::Linux => (&*LINUX_TOTAL, &*LINUX_MAP),
  };

  let (total, map) = (total.as_str(), map.as_str());

  methods::get_full_map(total, map, commit).await.context("")
}

pub async fn get_all_search(commit: &Commits) -> Result<Vec<SearchEntry>> {
  let total = &*AHQSTORE_TOTAL;
  let search = &*AHQSTORE_SEARCH;

  let mut result: Vec<SearchEntry> = methods::get_full_search(total, search, &commit.ahqstore)
    .await
    .context("")?;

  let (total, search) = match OS {
    "windows" => (&*WINGET_TOTAL, &*WINGET_SEARCH),
    "linux" => (&*LINUX_TOTAL, &*LINUX_SEARCH),
    "android" => (&*FDROID_TOTAL, &*FDROID_SEARCH),
    _ => unreachable!(),
  };

  result.append(
    &mut methods::get_full_search(total, search, &commit.alt)
      .await
      .context("")?,
  );

  Ok(result)
}

pub type RespMapData = super::MapData;

#[allow(unreachable_patterns)]
pub async fn get_map_by_source(
  source: OfficialManifestSource,
  commit: &str,
  id: &str,
) -> Result<RespMapData> {
  let map = match source {
    OfficialManifestSource::AHQStore => &*AHQSTORE_MAP,
    OfficialManifestSource::WinGet => &*WINGET_MAP,
    OfficialManifestSource::FDroid => &*FDROID_MAP,
    OfficialManifestSource::Linux => &*LINUX_MAP,
    _ => {
      return Err(anyhow!("source not supported"));
    }
  };

  methods::get_map(map, commit, id).await.context("")
}

pub async fn get_devs_apps(commit: &Commits, dev_id: &str) -> Result<Vec<String>> {
  let (commit, apps_dev) = match &dev_id[0..2] {
    "a:" => (&commit.ahqstore, &*AHQSTORE_APPS_DEV),
    e => (
      &commit.alt,
      match e {
        "w:" => &*WINGET_APPS_DEV,
        "f:" => &*FDROID_APPS_DEV,
        "l:" => &*LINUX_APPS_DEV,
        _ => unreachable!(),
      },
    ),
  };

  methods::get_devs_apps(apps_dev, commit, &dev_id[2..])
    .await
    .context("")
}

pub async fn get_dev_data(commit: &Commits, id: &str) -> Result<super::DevData> {
  let (commit, dev_data) = match &id[0..2] {
    "a:" => (&commit.ahqstore, &*AHQSTORE_DEV_DATA),
    e => (
      &commit.alt,
      match e {
        "w:" => &*WINGET_DEV_DATA,
        "f:" => &*FDROID_DEV_DATA,
        "l:" => &*LINUX_DEV_DATA,
        _ => unreachable!(),
      },
    ),
  };

  methods::get_dev_data(dev_data, commit, &id[2..])
    .await
    .context("")
}

pub async fn get_app_asset(commit: &Commits, app_id: &str, asset: &str) -> Option<Vec<u8>> {
  let (commit, app_asset_url) = match &app_id[0..2] {
    "a:" => (&commit.ahqstore, &*AHQSTORE_APP_ASSET_URL),
    e => (
      &commit.alt,
      match e {
        "w:" => &*WINGET_APP_ASSET_URL,
        "f:" => &*FDROID_APP_ASSET_URL,
        "l:" => &*LINUX_APP_ASSET_URL,
        _ => unreachable!(),
      },
    ),
  };

  methods::get_app_asset(app_asset_url, commit, &app_id[2..], asset).await
}

pub async fn get_app(commit: &Commits, app_id: &str) -> Result<AHQStoreApplication> {
  let (commit, app_asset_url) = match &app_id[0..2] {
    "a:" => (&commit.ahqstore, &*AHQSTORE_APP_URL),
    e => (
      &commit.alt,
      match e {
        "w:" => &*WINGET_APP_URL,
        "f:" => &*FDROID_APP_URL,
        "l:" => &*LINUX_APP_URL,
        _ => unreachable!(),
      },
    ),
  };

  methods::get_app(app_asset_url, commit, &app_id[2..])
    .await
    .context("HTTP Error")
}
