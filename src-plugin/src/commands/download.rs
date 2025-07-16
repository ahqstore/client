use std::sync::Arc;

use tauri::{ipc::Channel, AppHandle, Manager};

use crate::{
  structs::platform::downloader::{self, get_size},
  DownloadEvent, Result,
};

use anyhow::Context;
