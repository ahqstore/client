#[cfg(windows)]
use std::{
  env::current_exe,
  process::{self, Command},
};

#[cfg(windows)]
use check_elevation::is_elevated;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::InstallMode;

#[cfg(windows)]
fn place_bin_in_temp() -> String {
  let mut temp_file = std::env::temp_dir();

  std::fs::create_dir_all(&temp_file).unwrap();

  temp_file.push("uninstall.exe");

  let exe = current_exe().unwrap();
  let exe = exe.to_string_lossy();
  let exe = exe.to_string();

  std::fs::copy(&exe, &temp_file).unwrap();

  temp_file.to_string_lossy().to_string()
}

#[cfg(not(windows))]
pub fn relaunch_if_needed(_: &InstallMode) {}

#[cfg(windows)]
pub fn relaunch_if_needed(update: &InstallMode) {
  let exe = current_exe().unwrap();
  let exe = exe.to_string_lossy();

  let (wait, exe): (bool, String) =
    if matches!(update, &InstallMode::Uninstall) && !exe.ends_with("uninstall.exe") {
      (false, place_bin_in_temp())
    } else {
      (true, format!("{exe}"))
    };

  if !is_elevated().unwrap_or(false) {
    let mut cmd = Command::new("powershell");
    let cmd = cmd.creation_flags(0x08000000);
    let cmd = cmd.args(["start-process", "-FilePath"]);
    let cmd = cmd.arg(format!("\"{exe}\""));

    if wait {
      cmd.arg("-wait");
    }

    if matches!(update, &InstallMode::Install) {
      cmd.arg("-args 'update'");
    } else if matches!(update, &InstallMode::InstallPR) {
      cmd.arg("-args 'update-pr'");
    } else if matches!(update, &InstallMode::Uninstall) {
      cmd.arg("-args 'uninstall'");
    }
    cmd.arg("-verb runas");

    cmd.spawn().unwrap().wait().unwrap();

    process::exit(0);
  }
}
