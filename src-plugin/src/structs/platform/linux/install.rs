use std::sync::LazyLock;
use std::env;

pub(crate) static AHQSTORE_GLOBAL_DIR: LazyLock<String> = LazyLock::new(|| {
  format!("/ahqstore")
});

pub(crate) static AHQSTORE_USER_DIR: LazyLock<String> = LazyLock::new(|| {
  let useroot = env::var("HOME")
    .expect("Impossible error");

  format!(r"{useroot}/ahqstore")
});
