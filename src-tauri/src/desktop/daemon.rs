use tauri::AppHandle;
use std::sync::mpsc::Receiver;

use super::internal_show_window;

pub fn run_daemon(app: AppHandle, rx: Receiver<()>) {
  for _ in rx.iter() {
    internal_show_window(&app);
  }
}