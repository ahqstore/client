use std::sync::LazyLock;
use std::env;

use tauri::fs;
// use windows::{Win32::Security::{PSID, CheckTokenMembership, CreateWellKnownSid, WinBuiltinAdministratorsSid}, core::BOOL};

// TODO Probably in future
// pub fn is_admin() -> Option<bool> {
//   let mut resp = BOOL::from(false);
  
//   unsafe {
//     let mut size = size_of::<PSID>() as u32;

//     // Getting size
//     _ = CreateWellKnownSid(
//       WinBuiltinAdministratorsSid, 
//       None, 
//       None, 
//       &mut size
//     );

//     let mut psid = vec![0u8; size as usize];

//     let psid_ptr = PSID(psid.as_mut_ptr() as *mut _);

//     CreateWellKnownSid(
//       WinBuiltinAdministratorsSid, 
//       None, 
//       Some(psid_ptr), 
//       &mut size
//     ).ok()?;

//     CheckTokenMembership(None, psid_ptr, &mut resp).ok()?;
//   };

//   Some(resp.as_bool())
// }

pub(crate) static AHQSTORE_GLOBAL_DIR: LazyLock<String> = LazyLock::new(|| {
  let root = env::var("SYSTEMDRIVE")
    .expect("Impossible error");

  format!(r"{root}\ProgramData\AHQ Store Applications")
});

pub(crate) static AHQSTORE_USER_DIR: LazyLock<String> = LazyLock::new(|| {
  let useroot = env::var("USERPROFILE")
    .expect("Impossible error");

  format!(r"{useroot}\ahqstore")
});
