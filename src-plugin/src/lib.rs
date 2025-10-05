use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod structs;

mod commands;
use commands::*;
// Another set of commands
use crate::structs::platform::*;

mod error;
mod models;

pub use error::{Error, Result};

use structs::Ahqstore;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ahqstore APIs.
pub trait AhqstoreExt<R: Runtime> {
  fn ahqstore(&self) -> &Ahqstore<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AhqstoreExt<R> for T {
  fn ahqstore(&self) -> &Ahqstore<R> {
    self.state::<Ahqstore<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init() -> TauriPlugin<tauri::Wry> {
  Builder::new("ahqstore")
    .invoke_handler(tauri::generate_handler![
      #[cfg(desktop)]
      get_windows,
      #[cfg(desktop)]
      get_linux_distro,
      #[cfg(windows)]
      is_windows_11,
      download,
      encrypt,
      decrypt,
      open,
      set_progress,
      is_development,
      show_code,
      rem_code,
      hash_username,
      set_scale,
      refresh_commit,
      get_commit,
      get_all_search,
      get_home,
      get_app,
      get_dev_data,
      get_app_asset,
      get_devs_apps,
      get_arch,
    ])
    .setup(|app, api| {
      let ahqstore = structs::init(app, api)?;

      app.manage(ahqstore);

      app.ahqstore()
        .init(app.clone());

      Ok(())
    })
    .build()
}
