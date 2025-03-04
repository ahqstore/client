//! Declared URLS for:
//! AHQ Store AppImage Repository (ahqstore/repo_linux)
//!
//! Repository Mirror : <https://github.com/ahqstore/repo_linux>

use std::sync::LazyLock;

pub static LINUX_COMMIT_URL: &'static str =
  "https://api.github.com/repos/ahqstore/repo_linux/commits";
pub static LINUX_BASE_URL: &'static str = "https://rawcdn.githack.com/ahqstore/repo_linux/{COMMIT}";

pub static LINUX_APP_URL: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/apps/{{APP_ID}}.json"));
pub static LINUX_APP_ASSET_URL: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/res/{{APP_ID}}/{{ASSET}}"));

pub static LINUX_TOTAL: LazyLock<String> = LazyLock::new(|| format!("{LINUX_BASE_URL}/db/total"));
pub static LINUX_HOME: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/home.json"));

pub static LINUX_SEARCH: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/search/{{ID}}.json"));
pub static LINUX_MAP: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/map/{{ID}}.json"));

pub static LINUX_APPS_DEV: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/db/dev/{{ID}}"));
pub static LINUX_DEV_DATA: LazyLock<String> =
  LazyLock::new(|| format!("{LINUX_BASE_URL}/users/{{ID}}.json"));
