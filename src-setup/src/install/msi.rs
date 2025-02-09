use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn install_msi(path: &str) {
  Command::new("powershell")
    .args([
      "start-process",
      "-FilePath",
      &format!("\"{}\"", &path),
      "-Wait",
      "-ArgumentList",
      "/quiet, /passive, /norestart",
    ])
    .creation_flags(0x08000000)
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
}