use std::{sync::Arc, time::Duration};

use ahqstore_types::Commits;
use tauri::Runtime;
use tokio::{
  sync::{broadcast::Sender, mpsc::UnboundedReceiver, RwLock},
  time::sleep,
};

use crate::structs::{
  daemon::{SendRequest, StatusUpdateData},
  Ahqstore,
};

// It'll just be an api mapper
pub async fn daemon<R: Runtime>(
  _: &Ahqstore<R>,
  _: Arc<RwLock<CommitSearchIndex>>,
  tx: Sender<Arc<StatusUpdateData>>,
  rx: UnboundedReceiver<SendRequest>,
) {
  // Ahh, android queue is in Java World of things!
}
