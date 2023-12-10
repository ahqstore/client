use std::{
  sync::mpsc::{channel, Sender},
  time::Duration,
};

use ahqstore_types::{Command, Response};
use futures_util::SinkExt;
use tokio::{spawn, time::sleep};

use crate::windows::utils::{get_ws, write_log};

use super::{
  send_term,
  service::{download_app, get_app, install_app},
};

pub fn get_install_daemon() -> Sender<Command> {
  let (tx, rx) = channel();

  spawn(async move {
    let mut pending: Vec<Command> = vec![];

    let between = || sleep(Duration::from_millis(50));

    loop {
      'r: loop {
        if let Ok(data) = rx.try_recv() {
          pending.push(data);
        } else {
          break 'r;
        }
      }

      if let Some(ws) = get_ws() {
        for cmd in pending {
          match cmd {
            Command::GetApp(ref_id, app_id) => {
              let app_data = get_app(ref_id, app_id).await;
              let x = Response::as_msg(app_data);
              let _ = ws.send(x).await;

              send_term(ref_id).await;
            }
            Command::InstallApp(ref_id, app_id) => {
              if let Some(x) = download_app(ref_id, &app_id).await {
                let _ = ws
                  .send(Response::as_msg(Response::Installing(
                    ref_id,
                    app_id.clone(),
                  )))
                  .await;
                install_app(app_id.clone(), x);
                let _ = ws
                  .send(Response::as_msg(Response::Installed(ref_id, app_id)))
                  .await;
              } else {
                write_log("Error downloading");
              }
              send_term(ref_id).await;
            }
            _ => {}
          }

          between().await;
        }
        pending = vec![];
      }

      sleep(Duration::from_nanos(1)).await;
    }
  });

  tx
}
