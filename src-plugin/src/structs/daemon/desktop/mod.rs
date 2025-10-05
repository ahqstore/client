use std::{sync::Arc, time::Duration};

use ahqstore_types::Commits;
use tauri::Runtime;
use tokio::{sync::{RwLock, broadcast::Sender, mpsc::UnboundedReceiver}, time::sleep};

use crate::structs::{Ahqstore, daemon::{ReceivedData, SendRequest}};

pub async fn daemon<R: Runtime>(_: &Ahqstore<R>, commits: Arc<RwLock<Commits>>, tx: Sender<Arc<ReceivedData>>, rx: UnboundedReceiver<SendRequest>) {
  loop {
    sleep(Duration::from_millis(100));
  }
}