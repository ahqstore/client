use std::{
  borrow::Cow,
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use ahqstore_types::{get_app, AppUpdateInstallStatus, Commits, QueuedAppUpdate};
use tokio::sync::mpsc::Sender;

mod win32;

fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time is going in reverse")
    .as_secs()
}

pub async fn install_app(
  app_id: Arc<str>,
  sha: Commits,
  transaction: u64,
  sender: Sender<QueuedAppUpdate>,
) {
  let Ok(app) = get_app(&sha, &app_id).await else {
    _ = sender
      .send(QueuedAppUpdate {
        transaction,
        status: AppUpdateInstallStatus::Error {
          err: Cow::Borrowed("Unable to fetch app, is all correct?"),
          time: now(),
        },
      })
      .await;
    return;
  };

  let (Some(url), Some(extension)) = (app.get_win_download(), app.get_win_extension()) else {
    _ = sender
      .send(QueuedAppUpdate {
        transaction,
        status: AppUpdateInstallStatus::Error {
          err: Cow::Borrowed("Platform not supported"),
          time: now(),
        },
      })
      .await;

    return;
  };
}
