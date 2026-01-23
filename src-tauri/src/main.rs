// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  #[cfg(target_os = "windows")]
  #[cfg(debug_assertions)]
  configure_dev_webview_runtime();

  ahqstore_new_lib::run()
}

#[cfg(target_os = "windows")]
#[cfg(debug_assertions)]
fn configure_dev_webview_runtime() {
  let _ = std::env::current_exe()
    .ok()
    .and_then(|path| path.parent().map(|p| p.join("../../WebView.143.x64")))
    .filter(|path| path.is_dir())
    .map(|path| {
      println!("Using founded local webview runtime: {}", path.display());
      unsafe {
        std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", path);
      }
    });
}
