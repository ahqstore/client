use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Ahqstore;

#[cfg(mobile)]
use mobile::Ahqstore;

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
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("ahqstore")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let ahqstore = mobile::init(app, api)?;

      #[cfg(desktop)]
      let ahqstore = desktop::init(app, api)?;
      app.manage(ahqstore);
      
      Ok(())
    })
    .build()
}
