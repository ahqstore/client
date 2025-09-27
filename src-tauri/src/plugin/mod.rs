use std::fs::{self, File};

use tauri::{AppHandle, Manager};
use zip::ZipArchive;

pub fn get_html(hwnd: &AppHandle, plugin: &str, settings: bool) -> String {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  _ = fs::create_dir_all(&dir);

  dir.push(plugin);

  if settings {
    dir.push("settings.html");
  } else {
    dir.push("pluginUI.html");
  }

  fs::read_to_string(&dir).unwrap_or_default()
}

pub fn get_meta(hwnd: &AppHandle, plugin: &str) -> String {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  _ = fs::create_dir_all(&dir);

  dir.push(plugin);

  dir.push("metadata.json");

  fs::read_to_string(&dir).unwrap_or_default()
}

pub fn get_script(hwnd: &AppHandle, plugin: &str, script: &str) -> String {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  _ = fs::create_dir_all(&dir);

  dir.push(plugin);

  dir.push(script);

  let path = dir.canonicalize().unwrap_or_default();

  fs::read_to_string(&path).unwrap_or_default()
}

pub fn get_plugin_names(hwnd: &AppHandle) -> String {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  _ = fs::create_dir_all(&dir);
  
  let vect = fs::read_dir(&dir)
    .unwrap()
    .into_iter()
    .map(|x| x.unwrap().file_name())
    .map(|x| x.into_string().unwrap())
    .collect::<Vec<_>>();

  serde_json::to_string(&vect).unwrap_or_default()
}

pub fn get_state(hwnd: &AppHandle, plugin: &str, state_name: &str) -> String {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  dir.push(plugin);
  dir.push("state");
  _ = fs::create_dir_all(&dir);

  dir.push(state_name);

  fs::read_to_string(&dir).unwrap_or_default()
}

pub fn set_state(hwnd: &AppHandle, plugin: &str, state_name: &str, state_data: String) {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  dir.push(plugin);
  dir.push("state");
  _ = fs::create_dir_all(&dir);

  dir.push(state_name);

  _ = fs::write(&dir, &state_data);
}

pub fn install_plugin(hwnd: &AppHandle, plugin: &str, path: &str) -> Option<()> {
  let resolver = hwnd.path();

  let mut dir = resolver.app_local_data_dir().unwrap();

  dir.push("plugins");
  dir.push(plugin);

  _ = fs::create_dir_all(&dir);

  ZipArchive::new(File::open(path).ok()?)
    .ok()?
    .extract(&dir)
    .ok()
}