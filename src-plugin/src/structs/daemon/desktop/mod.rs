use std::{
  sync::Arc,
  time::{Duration, SystemTime, UNIX_EPOCH},
};

use ahqstore_types::{Commits, StatusUpdateData};
use tauri::Runtime;
use tokio::{
  sync::{broadcast::Sender, mpsc::UnboundedReceiver, RwLock},
  task::JoinHandle,
  time::sleep,
};

use crate::structs::{daemon::SendRequest, platform, search::CommitSearchIndex, Ahqstore};

const TEN_MINS: u64 = 10 * 60 * 1000;

pub async fn daemon<R: Runtime>(
  a: &Ahqstore<R>,
  commits: Arc<RwLock<CommitSearchIndex>>,
  tx: Sender<Arc<StatusUpdateData>>,
  mut rx: UnboundedReceiver<SendRequest>,
) {
  let mut next_check = 0u64;
  let mut user_initiated = false;

  let mut update_task: Option<JoinHandle<()>> = None;

  loop {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time is running in reverse")
      .as_secs();

    while let Some(x) = rx.try_recv().ok() {}

    if user_initiated {}

    sleep(Duration::from_millis(100)).await;
  }
}

pub fn should_run() -> bool {
  false
}
