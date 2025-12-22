//! Declared URLS for:
//! AHQ Store F-Droid Repo (f-droid mirror)
//!
//! Repository Mirror : <https://github.com/ahqstore/repo_android>
//! Thanks to F-Droid for providing the data

use std::sync::LazyLock;

pub static FDROID_COMMIT_URL: &'static str =
  "https://api.github.com/repos/ahqstore/repo_android/commits";
pub static FDROID_BASE_URL: &'static str =
  "https://cdn.jsdelivr.net/gh/ahqstore/repo_android@{COMMIT}";

pub static FDROID_APP_URL: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/apps/{{APP_ID}}.json"));
pub static FDROID_APP_ASSET_URL: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/res/{{APP_ID}}/{{ASSET}}"));

pub static FDROID_TOTAL: LazyLock<String> = LazyLock::new(|| format!("{FDROID_BASE_URL}/db/total"));
pub static FDROID_HOME: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/home.json"));

pub static FDROID_SEARCH: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/search/{{ID}}.json"));
pub static FDROID_MAP: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/map/{{ID}}.json"));

pub static FDROID_APPS_DEV: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/db/dev/{{ID}}"));
pub static FDROID_DEV_DATA: LazyLock<String> =
  LazyLock::new(|| format!("{FDROID_BASE_URL}/users/{{ID}}.json"));
