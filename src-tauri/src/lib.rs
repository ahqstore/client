use tauri::webview::WebviewWindowBuilder;

#[cfg(desktop)]
use tauri::utils::config::WindowEffectsConfig;
#[cfg(desktop)]
use tauri::window::Effect;

use tauri::Manager;

#[cfg(desktop)]
mod desktop;

pub static SHOULD_EXIT: i32 = 20;

#[cfg(windows)]
fn get_accent() -> Option<String> {
  use windows::core::w;
  use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};

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
      Some(&mut pcbdata as *mut u32),
    )
    .ok()
    .ok()?;

    println!("[INFO] Color: {color:?}");

    let [r, g, b, _] = color;

    Some(format!("window.accent = \"rgb({r},{g},{b})\""))
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(dead_code, unused)]
pub fn run() {
  #[cfg(windows)]
  let accent: &'static str = get_accent()
    .unwrap_or("window.accent = \"rgb(53,126,199)\"".into())
    .leak();

  #[cfg(not(windows))]
  let accent = "window.accent = \"rgb(53,126,199)\"";

  let app = tauri::Builder::default()
    .on_page_load(move |c, _| c.eval(accent).expect("Unable to evaluate script"));

  #[cfg(desktop)]
  let app = app.plugin(tauri_plugin_single_instance::init(|app, _, _| {
    let app_c = app.clone();
    std::thread::spawn(move || {
      use crate::desktop::show_window;

      let ready: bool = match crate::desktop::STARTED.lock() {
        Ok(x) => *x,
        Err(x) => **x.get_ref(),
      };

      // Dont create a second window if the app is already working on it
      if ready {
        show_window(&app_c);
      }
    });
  }));

  let app = app
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_ahqstore::init())
    .plugin(tauri_plugin_deep_link::init())
    .setup(|_app| {
      #[cfg(desktop)]
      println!("[INFO] Running Desktop Setup");
      #[cfg(desktop)]
      desktop::setup(_app)?;

      #[cfg(desktop)]
      _app
        .handle()
        .plugin(tauri_plugin_window_state::Builder::default().build())?;

      #[cfg(mobile)]
      create_window(_app);

      println!("[INFO] Running");
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  app.run(|_handle, event| match event {
    tauri::RunEvent::ExitRequested { api, code, .. } =>
    {
      #[cfg(desktop)]
      if code.unwrap_or(0) != SHOULD_EXIT {
        api.prevent_exit();
      }
    }
    _ => {}
  });
}

pub(crate) fn create_window<T: Manager<R>, R: tauri::Runtime>(app: &T) {
  if app.get_webview_window("main").is_some() {
    return;
  }

  let builder = WebviewWindowBuilder::new(app, "main", {
    #[cfg(not(debug_assertions))]
    let url = tauri::WebviewUrl::App("/".into());

    #[cfg(debug_assertions)]
    let url = tauri::WebviewUrl::External(app.config().build.dev_url.as_ref().unwrap().clone());

    url
  });

  #[cfg(desktop)]
  let builder = builder.center()
    .min_inner_size(348.0, 700.0)
    .inner_size(1024.0, 760.0)
    .resizable(true)
    .decorations(false)
    .visible(false)
    .prevent_overflow()
    .title("AHQ Store Neo")
    .effects(
      WindowEffectsConfig {
        effects: vec![Effect::Mica],
        state: None,
        radius: None,
        color: None,
      }
    )
    .additional_browser_args("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --autoplay-policy=no-user-gesture-required");

  builder.transparent(true).build().unwrap();
}
