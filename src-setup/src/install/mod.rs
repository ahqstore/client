#[cfg(unix)]
use notify_rust::Notification;
use slint::SharedString;
use std::{fs, thread, time::Duration};

use self::fetch::fetch;
use self::fetch::ReleaseData;
use crate::{utils::get_install, AppWindow, InstallMode};
use reqwest::Client;

mod download;
mod fetch;

#[cfg(not(windows))]
mod deb;
#[cfg(windows)]
mod msi;
#[cfg(windows)]
mod regedit;

use download::download;

static mut WIN: Option<AppWindow> = None;

pub fn start_install(win: AppWindow, install: InstallMode) {
  unsafe { WIN = Some(win) };

  thread::spawn(move || {
    let win = unsafe { WIN.as_mut().unwrap() };

    win.set_msg(SharedString::from("Getting files ready..."));

    thread::sleep(Duration::from_secs(3));

    let mut runtime = tokio::runtime::Builder::new_current_thread();
    let runtime = runtime.enable_all().build().unwrap();

    runtime.block_on(async {
      let (mut client, files) = fetch(&install).await;

      println!("{:?}", &files);

      plt_install(&win, &mut client, &files).await;
    });
  });
}

#[cfg(not(windows))]
async fn plt_install(win: &AppWindow, client: &mut Client, files: &ReleaseData) {
  use std::process;

  use crate::{
    install::deb::{exit, get_sudo, install_daemon, install_deb},
    utils::get_service_dir,
  };

  if &files.deb == "" {
    Notification::new()
      .summary("Uh Oh!")
      .body("We were unable to find the linux build files, try toggling the Pre-Release Option")
      .show()
      .unwrap();

    win.set_counter(-1.0);
    win.set_msg("Install".into());
    return;
  }

  win.set_msg("Downloading...".into());

  let mut sudo = get_sudo();
  let installer = get_install();
  let service = get_service_dir();

  let _ = fs::remove_file(&installer);

  download(client, &files.deb, &installer, |perc| {
    win.set_counter(perc);
  })
  .await;

  thread::sleep(Duration::from_secs(1));
  win.set_counter(0.0);
  thread::sleep(Duration::from_secs(3));

  download(client, &files.linux_daemon, &service, |perc| {
    win.set_counter(perc);
  })
  .await;

  thread::sleep(Duration::from_secs(2));
  win.set_indet(true);

  win.set_msg("Installing...".into());

  install_deb(&mut sudo, &installer);
  install_daemon(&mut sudo, service);
  exit(sudo);

  thread::sleep(Duration::from_secs(1));

  let _ = fs::remove_file(&installer);
  win.set_msg("Installed 🎉".into());
  win.set_indet(false);

  thread::sleep(Duration::from_secs(5));
  process::exit(0);
}

#[cfg(windows)]
async fn plt_install(win: &AppWindow, client: &mut Client, files: &ReleaseData) {
  use std::process;

  use crate::{
    install::msi::{install_msi, install_service},
    utils::get_service_dir,
  };

  win.set_msg("Downloading...".into());

  let installer = get_install();
  let service = get_service_dir();

  let _ = fs::remove_file(&installer);

  download(client, &files.msi, &installer, |perc| {
    win.set_counter(perc);
  })
  .await;

  thread::sleep(Duration::from_secs(1));
  win.set_counter(0.0);
  thread::sleep(Duration::from_secs(3));

  download(client, &files.service, &service, |perc| {
    win.set_counter(perc);
  })
  .await;

  win.set_indet(true);

  win.set_msg("Installing...".into());

  regedit::create_association();

  thread::sleep(Duration::from_secs(2));

  install_msi(&installer);

  thread::sleep(Duration::from_secs(3));

  install_service(&service);

  thread::sleep(Duration::from_secs(1));

  let _ = fs::remove_file(&installer);

  win.set_msg("Installed 🎉".into());
  win.set_indet(false);

  thread::sleep(Duration::from_secs(5));
  process::exit(0);
}
