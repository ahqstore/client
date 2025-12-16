use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use ahqstore_types::Commits;
use tauri::Runtime;
use tokio::{sync::{RwLock, broadcast::Sender, mpsc::UnboundedReceiver}, task::JoinHandle, time::sleep};

use crate::structs::{Ahqstore, daemon::{ReceivedData, SendRequest}, platform};

const TEN_MINS: u64 = 10 * 60 * 1000;

pub async fn daemon<R: Runtime>(a: &Ahqstore<R>, commits: Arc<RwLock<Commits>>, tx: Sender<Arc<ReceivedData>>, mut rx: UnboundedReceiver<SendRequest>) {
  let mut next_check = 0u64;
  let mut user_initiated = false;

  let mut update_task: Option<JoinHandle<()>> = None;

  loop {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time is running in reverse").as_secs();
    
    while let Some(x) = rx.try_recv().ok() {

    }

    if user_initiated {

    }

    sleep(Duration::from_millis(100)).await;
  }
}

pub fn should_run() -> bool {
  false
}