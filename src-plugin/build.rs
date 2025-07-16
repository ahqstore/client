const COMMANDS: &[&str] = &[
  "get_windows",
  "get_linux_distro",
  "is_windows_11",
  "to_hash_uid",
  "open",
  "set_progress",
  "is_development",
  "show_code",
  "rem_code",
  "hash_username",
  "refresh_commit",
  "get_commit",
  "get_all_search",
  "get_home",
  "get_app",
  "get_dev_data",
  "get_app_asset",
  "get_devs_apps",
  "get_arch",
  "set_scale",
  "encrypt",
  "decrypt",
  "download",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .build();
}
