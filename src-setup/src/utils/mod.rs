#[cfg(windows)]
use std::{os::windows::process::CommandExt, process::Command};

use dirs::home_dir;
use lazy_static::lazy_static;

#[cfg(windows)]
lazy_static! {
  pub static ref ROOT_DIR: String = std::env::var("SystemDrive").unwrap();
  pub static ref AHQSTORE_ROOT: String = format!(
    "{}{}ProgramData{}AHQ Store Applications",
    &*ROOT_DIR, &SEP, &SEP
  );
}

#[cfg(unix)]
lazy_static! {
  pub static ref ROOT_DIR: String = "/".into();
  pub static ref AHQSTORE_ROOT: String = "/ahqstore".into();
}

#[cfg(windows)]
static SEP: &'static str = "\\";

#[cfg(unix)]
static SEP: &'static str = "/";

lazy_static! {
  pub static ref PROGRAMS: String = format!("{}{}Programs", &*AHQSTORE_ROOT, &SEP,);
  pub static ref UPDATERS: String = format!("{}{}Updaters", &*AHQSTORE_ROOT, &SEP);
  pub static ref INSTALLERS: String = format!("{}{}Installers", &*AHQSTORE_ROOT, &SEP);
}

pub fn get_install() -> String {
  let mut path = home_dir().unwrap();
  #[cfg(windows)]
  path.push("ahqstore.msi");

  #[cfg(not(windows))]
  path.push("ahqstore.deb");

  path.to_str().unwrap().to_string()
}