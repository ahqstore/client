use std::{
  mem::replace,
  sync::Arc,
  time::{Duration, SystemTime, UNIX_EPOCH},
};

use ahqstore_types::{
  AppActionIntent, AppUpdateInstallStatus, Commits, QueuedApp, QueuedAppData, QueuedAppUpdate,
  StatusUpdateData,
};
use tauri::Runtime;
use tokio::{
  spawn,
  sync::mpsc::channel,
  sync::{broadcast::Sender, mpsc::UnboundedReceiver, Notify, RwLock},
  task::JoinHandle,
  time::{interval, sleep, MissedTickBehavior},
};

use crate::structs::{daemon::SendRequest, platform, search::CommitSearchIndex, Ahqstore};

const TEN_MINS: u64 = 10 * 60 * 1000;

mod installation;
mod lock;

pub async fn daemon<R: Runtime>(
  ahqstore: &Ahqstore<R>,
  commits: Arc<RwLock<CommitSearchIndex>>,
  tx: Sender<Arc<StatusUpdateData>>,
  mut rx: UnboundedReceiver<SendRequest>,
) {
  let mut user_initiated = false;

  // Queues
  let mut queue: Vec<QueuedApp> = Vec::with_capacity(50);

  // Timers
  let mut intl = interval(Duration::from_mins(10));
  intl.tick().await;

  intl.set_missed_tick_behavior(MissedTickBehavior::Burst);

  let mut notify = Notify::new();

  // States
  let mut changed = false;
  let mut transaction = 0;

  let (for_apps, mut updates) = channel::<QueuedAppUpdate>(50);

  // Main Loop
  loop {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time is running in reverse")
      .as_secs();

    tokio::select! {
      // Queue Pruning
      _ = sleep(Duration::from_millis(100)) => {
        let old = queue.len();
        queue.retain(|x| match &x.status {
          AppUpdateInstallStatus::Successful { time } => now < (*time + 2),
          AppUpdateInstallStatus::Cancelled { time } => now < (*time + 5),
          AppUpdateInstallStatus::Error { time, .. } => now < (*time + 10),
          _ => true,
        });
        changed = old != queue.len();
      }

      Some(msg) = updates.recv() => {
        if let Some(x) = queue.iter_mut().find(|x| x.transaction == msg.transaction) {
          x.status = msg.status;
        }
      }

      // Sending update inteval tick
      //
      // Updating ticking
      _ = intl.tick() => {
        notify.notify_one();
      }

      // Run Update
      _ = notify.notified() => {
        // TODO
        changed = true;
      }

      // Handle user commands
      Some(msg) = rx.recv() => {
        handle_msg(ahqstore, &mut user_initiated, &notify, msg, &mut queue, &mut transaction, now).await;

        while let Ok(extra_msg) = rx.try_recv() {
          handle_msg(ahqstore, &mut user_initiated, &notify, extra_msg, &mut queue, &mut transaction, now).await;
        }

        changed = true;
      }
    }

    if changed {
      let queue_data = queue.iter().map(QueuedAppData::from).collect::<Box<[_]>>();
      _ = tx.send(Arc::new(StatusUpdateData {
        disable_update: false,
        overflow: queue.len() >= 100,
        queue: queue_data,
      }));

      changed = false;
    }
  }
}

#[inline(always)]
async fn handle_msg<R: Runtime>(
  ahqstore: &Ahqstore<R>,
  user_initiated: &mut bool,
  notify: &Notify,
  msg: SendRequest,
  queue: &mut Vec<QueuedApp>,
  transaction: &mut u64,
  now: u64,
) {
  *transaction += 1;

  let commit = ahqstore.commits.read().await.commit.clone();

  if queue.len() < 100 {
    match msg {
      SendRequest::CheckForUpdate => {
        if ahqstore.can_update_commit().await {
          *user_initiated = true;
          notify.notify_one();
        }
      }

      SendRequest::CancelTransaction { transaction } => {
        if let Some(x) = queue.iter_mut().find(|x| x.transaction == transaction) {
          match x.status {
            AppUpdateInstallStatus::Pending
            | AppUpdateInstallStatus::PendingUserAction
            | AppUpdateInstallStatus::Downloading { .. }
            | AppUpdateInstallStatus::AVScanning => {
              x.status = AppUpdateInstallStatus::Cancelled { time: now };

              if let Some(data) = x.task.take() {
                data.abort();
              }

              // Run cleanup
            }
            // Ignore whatever is requested
            _ => {}
          }
        }
      }

      SendRequest::PerformTransaction { transaction } => {
        if let Some(x) = queue.iter_mut().find(|x| x.transaction == transaction) {
          x.status = AppUpdateInstallStatus::Pending;

          x.task = Some(spawn(async {}));
        }
      }

      SendRequest::PerformAllTransactions => {
        queue.iter_mut().for_each(|x| {
          if let AppUpdateInstallStatus::PendingUserAction = &x.status {
            x.status = AppUpdateInstallStatus::Pending;

            // Now spawn the task
            x.task = Some(spawn(async {}));
          }
        });
      }

      SendRequest::InstallUSERAPP { app_id } => {
        let app_id: Arc<str> = Arc::from(app_id);
        queue.push(QueuedApp {
          status: AppUpdateInstallStatus::Pending,
          intent: AppActionIntent::Install,
          transaction: *transaction,
          id: app_id,
          task: Some(spawn(async {})),
        });
      }

      SendRequest::RemoveUSERAPP { app_id } => {
        let app_id: Arc<str> = Arc::from(app_id);
        queue.push(QueuedApp {
          status: AppUpdateInstallStatus::Pending,
          intent: AppActionIntent::Uninstall,
          transaction: *transaction,
          id: app_id,
          task: Some(spawn(async {})),
        });
      }
    }
  }
}
