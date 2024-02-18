use std::{
  env::current_exe,
  process::{self, Command},
};

#[cfg(windows)]
use check_elevation::is_elevated;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::InstallMode;

pub fn relaunch_if_needed(update: &InstallMode) {
  let exe = current_exe().unwrap();
  let exe = exe.to_string_lossy();
  let exe: &str = &format!("{exe}");

  #[cfg(windows)]
  if !is_elevated().unwrap_or(false) {
    let mut cmd = Command::new("powershell");
    let cmd = cmd.creation_flags(0x08000000);
    let cmd = cmd.args(["start-process", exe]);

    if matches!(update, &InstallMode::Install) {
      cmd.arg("-args 'update'");
    } else if matches!(update, &InstallMode::InstallPR) {
      cmd.arg("-args 'update-pr'");
    }
    cmd.arg("-verb runas");

    cmd.spawn().unwrap().wait().unwrap();

    process::exit(0);
  }

  // #[cfg(not(windows))]
  // {
  //   let val = check();
  //   println!("{:?}", &val);
  //   if let RunningAs::User = val {
  //     let mut cmd = Command::new("pkexec");
  //     cmd.arg(exe);

  //     if matches!(update, &InstallMode::Install) {
  //       cmd.arg("update");
  //     } else if matches!(update, &InstallMode::InstallPR) {
  //       cmd.arg("update-pr");
  //     }

  //     cmd.spawn().unwrap();

  //     process::exit(0);
  //   }
  // }
}
