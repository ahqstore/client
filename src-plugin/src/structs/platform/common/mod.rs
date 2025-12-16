use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager, Runtime};

use anyhow::Result;

#[cfg(desktop)]
pub mod desktop;

pub enum ConnectionType {
  Disconnected,
  Metered,
  Unmetered
}

pub fn downloads<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf> {
  let mut dir = handle.path().app_cache_dir()?;

  dir.push("data");

  _ = fs::create_dir_all(&dir);

  Ok(dir)
}

pub fn dwnl_tmp<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf> {
  let mut dir = handle.path().app_cache_dir()?;

  dir.push("tmp");

  _ = fs::create_dir_all(&dir);

  Ok(dir)
}

pub fn clear_dwnl(handle: &AppHandle) {
  if let Some(dwn) = downloads(handle).ok() {
    _ = fs::remove_dir_all(&dwn);
  }

  if let Some(tmp) = dwnl_tmp(handle).ok() {
    _ = fs::remove_dir_all(&tmp);
  }
}