use std::sync::mpsc::Receiver;
use tauri::AppHandle;

use super::internal_show_window;

pub fn run_daemon(app: AppHandle, rx: Receiver<()>) {
  for _ in rx.iter() {
    internal_show_window(&app);
  }
}
