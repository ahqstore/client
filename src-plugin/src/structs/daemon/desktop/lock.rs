use tokio::sync::Mutex;

pub static INSTALLLOCK: Mutex<()> = Mutex::const_new(());

pub fn is_installing() -> bool {
  INSTALLLOCK.try_lock().is_err()
}
