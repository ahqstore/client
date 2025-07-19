use std::ffi::OsStr;

use libc::getppid;
use uzers::{get_user_by_uid, get_current_uid};

fn main() {
  unsafe { getppid() };

  let user = get_user_by_uid(get_current_uid()).unwrap();

  let root = user.uid() == 0 || user.groups().unwrap().iter().any(|x| x.name() == OsStr::new("ahqstore"));

  println!("Hello, world {root}!");
}
