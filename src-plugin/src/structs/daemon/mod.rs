use std::{
  sync::{Arc, LazyLock, OnceLock},
  thread,
};

use ahqstore_types::Commits;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tokio::{
  runtime::Builder,
  sync::{
    broadcast::{channel, Receiver, Sender as S2},
    mpsc::{unbounded_channel, UnboundedSender as S1},
    RwLock,
  },
};

pub enum SendRequest {}

pub enum ReceivedData {}

pub type IPCSend = S1<SendRequest>;
pub type Broadcast = S2<Arc<ReceivedData>>;

pub static BOXED_TX_REF: OnceLock<Broadcast> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateInstallStatus {
  Unknown,
  // The UpdateInstall process is checking for updates
  CheckingForUpdates,
  // The UpdateInstall process is in progress
  InProgress,
  // Everything is alright
  // Nothing Pending
  NoPending,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum AppUpdateInstallStatus {
  /// { "status": "Pending" }
  Pending,
  /// { "status": "Downloading", "progress": 100.0 }
  Downloading { progress: f64 },
  /// { "status": "Installing" }
  Installing,
  /// { "status": "Updating" }
  Updating,
  /// { "status": "Uninstalling" }
  Uninstalling,
  /// { "status": "Done" }
  Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUpdateInstall {
  pub id: String,
  pub status: AppUpdateInstallStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInstallState {
  pub update: UpdateInstallStatus,
  pub list: Vec<AppUpdateInstall>,
}

pub static UPDATE_INSTALL_STATUS: LazyLock<RwLock<UpdateInstallState>> = LazyLock::new(|| {
  RwLock::new(UpdateInstallState {
    update: UpdateInstallStatus::Unknown,
    list: vec![],
  })
});

#[cfg(desktop)]
mod desktop;

#[cfg(desktop)]
pub use desktop::*;

#[cfg(mobile)]
mod android;

#[cfg(mobile)]
pub use android::*;

use crate::{structs::search::CommitSearchIndex, AhqstoreExt};

pub fn get_daemon_listener() -> Receiver<Arc<ReceivedData>> {
  BOXED_TX_REF.get().expect("Cannot error out").subscribe()
}

// Initializes update-installer-worker
// This is responsible for handling app installs, updates
pub fn initialize<R: Runtime>(
  hwnd: AppHandle<R>,
  commits: Arc<RwLock<CommitSearchIndex>>,
) -> IPCSend {
  let (ipc_send, rx) = unbounded_channel();
  let (tx, _) = channel(100);

  BOXED_TX_REF.set(tx.clone()).expect("No error, don't worry");

  thread::spawn(move || {
    Builder::new_current_thread()
      .enable_all()
      .build()
      .expect("Unable to create thread")
      .block_on(async move {
        let astore = hwnd.ahqstore();

        daemon(astore, commits, tx, rx).await;
      });
  });

  ipc_send
}
