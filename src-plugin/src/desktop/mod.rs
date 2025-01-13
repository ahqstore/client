use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

mod platform;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Ahqstore<R>> {
  Ok(Ahqstore(app.clone()))
}

/// Access to the ahqstore APIs.
pub struct Ahqstore<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Ahqstore<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
