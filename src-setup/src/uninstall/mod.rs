use slint::SharedString;
use std::{
  fs, mem,
  process::{self, Command},
  thread,
  time::Duration,
};

#[cfg(windows)]
use super::install::regedit::rem_reg;
#[cfg(windows)]
use crate::{install::regedit, AppWindow};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
static mut WIN: Option<AppWindow> = None;

#[cfg(not(windows))]
pub fn uninstall<T>(_: T) {}

#[cfg(windows)]
pub fn uninstall(win: AppWindow) {
  use crate::utils;

  unsafe {
    WIN = Some(win);
  };

  thread::spawn(move || {
    let win = unsafe { mem::replace(&mut WIN, None).unwrap() };

    win.set_msg(SharedString::from("Getting files ready..."));

    thread::sleep(Duration::from_secs(3));

    win.set_msg(SharedString::from("Uninstalling..."));

    let id = fs::read_to_string(r"C:\Program Files\AHQ Store\unst").unwrap();

    let success = Command::new("msiexec.exe")
      .arg("/qb+")
      .arg(format!("/x{}", &id))
      .spawn()
      .unwrap()
      .wait()
      .unwrap()
      .success();

    utils::kill_daemon();

    fs::remove_dir_all(r"C:\Program Files\AHQ Store");
    let rem = fs::remove_file(
      r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup\ahqstore_user_daemon.exe",
    );
    println!("Err {:?}", rem.err());

    println!("Success: {success}");
    regedit::rem_reg(&id);

    win.set_msg("Uninstalled 🎉".into());

    thread::sleep(Duration::from_secs(5));
    process::exit(0);
  });
}
