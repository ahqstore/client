pub mod av;
mod msi;

use ahqstore_types::InstallerFormat;
use mslnk::ShellLink;
use serde_json::to_string_pretty;
use std::{
  fs,
  io::Error,
  os::windows::process::CommandExt,
  process::{Child, Command},
  thread::{self, sleep, JoinHandle},
  time::Duration,
};

use crate::{
  handlers::install_node,
  utils::{
    get_installer_file, get_program_folder, get_programs, get_target_lnk,
    structs::{AHQStoreApplication, AppData},
  },
};

use super::UninstallResult;

pub fn run(path: &str, args: &[&str]) -> Result<Child, Error> {
  Command::new(path)
    .creation_flags(0x08000000)
    .args(args)
    .spawn()
}

pub fn unzip(path: &str, dest: &str) -> Result<Child, Error> {
  Command::new("powershell")
    .creation_flags(0x08000000)
    .args([
      "-Command",
      &format!("Expand-Archive -Path '{path}' -DestinationPath '{dest}' -Force"),
    ])
    .spawn()
}

pub async fn install_app(app: &AHQStoreApplication) -> Option<Child> {
  let file = get_installer_file(app);

  let Some(win32) = app.get_win_download() else {
    return None;
  };

  match win32.installerType {
    InstallerFormat::WindowsZip => load_zip(&file, app),
    InstallerFormat::WindowsInstallerMsi => install_msi(&file, app),
    _ => None,
  }
}

pub fn install_msi(msi: &str, app: &AHQStoreApplication) -> Option<Child> {
  let install_folder = get_program_folder(&app.appId);

  fs::create_dir_all(&install_folder).ok()?;

  let to_exec_msi = format!("{}\\installer.msi", &install_folder);

  fs::copy(&msi, &to_exec_msi).ok()?;
  fs::write(
    format!("{}\\app.json", &install_folder),
    to_string_pretty(&app).ok()?,
  )
  .ok()?;
  fs::write(
    format!("{}\\ahqStoreVersion", &install_folder),
    &app.version,
  )
  .ok()?;
  fs::remove_file(&msi).ok()?;

  run("msiexec", &["/norestart", "/qn", "/i", &to_exec_msi]).ok()
}

pub fn load_zip(zip: &str, app: &AHQStoreApplication) -> Option<Child> {
  let install_folder = get_program_folder(&app.appId);
  let version_file = format!("{}\\ahqStoreVersion", install_folder);

  let _ = fs::remove_dir_all(&install_folder);
  fs::create_dir_all(&install_folder).ok()?;

  sleep(Duration::from_millis(200));
  let cmd = unzip(&zip, &install_folder);

  println!("Unzipped");

  let cleanup = |err| {
    let _ = fs::remove_file(&zip);

    if err {
      let _ = fs::remove_dir_all(&install_folder);
    } else {
      if let Some(val) = app.get_win_options() {
        if let Some(exec) = &val.exec {
          if let Ok(link) = ShellLink::new(format!("{}\\{}", &install_folder, &exec)) {
            let _ = link.create_lnk(get_target_lnk(&app.appShortcutName));
          }
        }
      }
    }
  };

  let val = (|| {
    let mut child = cmd.ok()?;
    let status = child.wait().ok()?;

    if !status.success() {
      return None;
    }
    let _ = fs::write(&version_file, &app.version).ok()?;

    let _ = fs::write(
      format!("{}\\app.json", &install_folder),
      to_string_pretty(&app).ok()?,
    )
    .ok()?;

    return Command::new("whoami")
      .creation_flags(0x08000000)
      .spawn()
      .ok();
  })();

  cleanup(val.is_none());

  val
}

pub fn uninstall_app(app: &AHQStoreApplication) -> UninstallResult {
  let link = get_target_lnk(&app.appShortcutName);
  let program = get_program_folder(&app.appId);

  if msi::is_msi(&app.appId) {
    UninstallResult::Thread(msi::uninstall_msi(app.appId.clone()))
  } else {
    UninstallResult::Thread(thread::spawn(move || {
      let _ = fs::remove_file(&link);

      if !fs::remove_dir_all(&program).is_ok() {
        return false;
      }

      // Successful
      true
    }))
  }
}

pub fn list_apps() -> Option<Vec<AppData>> {
  let folder = get_programs();

  let dirs = fs::read_dir(&folder).ok()?;

  let mut vec = vec![];

  for dir in dirs {
    let dir = dir.ok()?.file_name();
    let dir = dir.to_str().unwrap_or("unknown");

    let version = fs::read_to_string(format!(
      "{}\\{}",
      &get_program_folder(&dir),
      "ahqStoreVersion"
    ))
    .unwrap_or("unknown".into());

    if version == "unknown" {
      let _ = fs::remove_dir_all(get_program_folder(&dir));
    } else if version != "custom" {
      if msi::is_msi(dir) {
        if msi::exists(dir).unwrap_or(false) {
          vec.push((dir.to_owned(), version));
        }
      } else {
        vec.push((dir.to_owned(), version));
      }
    }
  }

  Some(vec)
}
