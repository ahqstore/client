use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::structs::platform::os::install::{AHQSTORE_USER_DIR, AHQSTORE_GLOBAL_DIR};

pub async fn list_user_apps() -> Option<Vec<AppListing>> {
  let dir = &AHQSTORE_USER_DIR.as_str();

  _inner_list_apps(dir, true).await
}

pub async fn list_global_apps() -> Option<Vec<AppListing>> {
  let dir = &AHQSTORE_GLOBAL_DIR.as_str();

  _inner_list_apps(dir, false).await
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum AppListing {
  Full {
    app_id: String
  },
  Unknown {
    app_id: String
  }
}

pub async fn _inner_list_apps(dir: &str, create_if_needed: bool) -> Option<Vec<AppListing>> {
  // Check if it exists
  if fs::read_dir(&dir)
    .await
    .is_err() {
    fs::create_dir_all(&dir).await.ok()?;
  }

  let mut tasks = vec![];

  let mut dir  =fs::read_dir(&dir)
      .await
      .ok()?;

  while let Some(entry) = dir.next_entry().await.ok()? {
    // Fetching & Parsing
    tasks.push(async move {
      // TODO: Implement

      Result::<(), String>::Ok(())
    });
  }

  let val = join_all(tasks).await;

  Some(
    val
      .into_iter()
      // Guaranteed unwrap
      .map(|x| match x {
        Ok(()) => AppListing::Full { app_id: "()".into() },
        Err(app_id) => AppListing::Unknown { app_id }
      })
      .collect::<Vec<_>>()
  )
}