#[cfg(desktop)]
mod desktop;

#[cfg(windows)]
fn get_accent() -> Option<String> {
  use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};
  use windows::core::w;

  unsafe {
    let mut color = [0u8; 4];
    let mut pcbdata: u32 = 4;

    RegGetValueW(
      HKEY_CURRENT_USER,
      w!(r"Software\Microsoft\Windows\DWM"),
      w!("AccentColor"),
      RRF_RT_REG_DWORD,
      None,
      Some(&mut color as *mut _ as _),
      Some(&mut pcbdata as *mut u32)
    ).ok().ok()?;

    println!("[INFO] Color: {color:?}");

    let [r, g, b, _] = color;

    Some(format!("window.accent = \"rgb({r},{g},{b})\""))
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(windows)]
  let accent = get_accent().unwrap_or("window.accent = \"rgb(53,126,199)\"".into()).leak();

  #[cfg(not(windows))]
  let accent = "window.accent = \"rgb(53,126,199)\"";

  tauri::Builder::default()
    .on_page_load(|c, _| c.eval(accent).expect("Unable to evaluate script"))
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_ahqstore::init())
    .setup(|app| {
      println!("[INFO] Running Desktop Setup");
      #[cfg(desktop)]
      desktop::setup(app)?;

      #[cfg(desktop)]
      app.handle().plugin(tauri_plugin_window_state::Builder::default().build())?;

      println!("[INFO] Running");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
