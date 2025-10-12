use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use ahqstore_types::Commits;
use tauri::Runtime;
use tokio::{sync::{RwLock, broadcast::Sender, mpsc::UnboundedReceiver}, time::sleep};

use crate::structs::{Ahqstore, daemon::{ReceivedData, SendRequest}};

pub async fn daemon<R: Runtime>(_: &Ahqstore<R>, commits: Arc<RwLock<Commits>>, tx: Sender<Arc<ReceivedData>>, rx: UnboundedReceiver<SendRequest>) {
  let mut next_check = 0u64;

  loop {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time is running in reverse").as_secs();



    sleep(Duration::from_millis(100));
  }
}