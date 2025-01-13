use winreg::enums::*;
use winreg::RegKey;

#[cfg(windows)]
pub fn create_association() -> Option<()> {
  let root = RegKey::predef(HKEY_CLASSES_ROOT);

  let (key, _) = root.create_subkey("ahqstore").ok()?;
  key.set_value("", &"AHQ Store").ok()?;

  let (icon, _) = key.create_subkey("DefaultIcon").ok()?;
  icon
    .set_value("", &r"C:\Program Files\AHQ Store\ahq-store-app.exe,0")
    .ok()?;

  let (shell, _) = key.create_subkey("shell").ok()?;
  let (shell, _) = shell.create_subkey("open").ok()?;
  let (shell, _) = shell.create_subkey("command").ok()?;
  shell
    .set_value(
      "",
      &r#""C:\Program Files\AHQ Store\ahq-store-app.exe" protocol %1"#,
    )
    .ok()?;

  Some(())
}

#[cfg(windows)]
pub fn custom_uninstall() -> Option<()> {
  use std::fs;

  let root = RegKey::predef(HKEY_LOCAL_MACHINE);

  let key = root.open_subkey("SOFTWARE").ok()?;
  let key = key.open_subkey("Microsoft").ok()?;
  let key = key.open_subkey("Windows").ok()?;
  let key = key.open_subkey("CurrentVersion").ok()?;
  let key = key
    .open_subkey_with_flags("Uninstall", KEY_ALL_ACCESS)
    .ok()?;

  let mut uninstall_str = String::default();

  let unst = r#""C:\Program Files\AHQ Store\uninstall.exe" uninstall"#;

  key.enum_keys().for_each(|x| {
    if let Ok(x) = x {
      let debug_data = &x;

      if let Ok(key) = key.open_subkey_with_flags(&x, KEY_ALL_ACCESS) {
        let name = key.get_value::<String, &str>("DisplayName");
        let uninstall = key.get_value::<String, &str>("UninstallString");

        if let (Ok(x), Ok(y)) = (name, uninstall) {
          if &x == "AHQ Store" {
            println!("id {debug_data} Name {x}");
            key
              .set_value(
                "DisplayIcon",
                &r"C:\Program Files\AHQ Store\ahq-store-app.exe,0",
              )
              .unwrap();
            key.set_value("WindowsInstaller", &0u32).unwrap();
            key.set_value("UninstallString", &unst).unwrap();

            if &y != unst {
              uninstall_str = y;
            } else {
              uninstall_str = format!("{}", debug_data);
            }
          }
        }
      }
    }
  });

  let uninstall_str = uninstall_str.to_lowercase();
  let uninstall_str = uninstall_str.replace("msiexec.exe /x", "");

  fs::write(r"C:\Program Files\AHQ Store\unst", uninstall_str).ok()
}

pub fn rem_reg(path: &str) -> Option<()> {
  let root = RegKey::predef(HKEY_LOCAL_MACHINE);

  let key = root.open_subkey("SOFTWARE").ok()?;
  let key = key.open_subkey("Microsoft").ok()?;
  let key = key.open_subkey("Windows").ok()?;
  let key = key.open_subkey("CurrentVersion").ok()?;
  let key = key
    .open_subkey_with_flags("Uninstall", KEY_ALL_ACCESS)
    .ok()?;

  /// DANGER: NO NOT REMOVE
  /// IF PATH == "", IT"LL BREAK THE SYSTEM
  if path != "" && path.len() > 10 {
    key.delete_subkey_all(path.to_uppercase()).ok()
  } else {
    None
  }
}
