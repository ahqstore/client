use std::sync::LazyLock;
use tauri::async_runtime::Mutex;
use tauri::webview::{WebviewWindow, WebviewWindowBuilder};
use tauri::{Emitter, EventTarget};

#[cfg(desktop)]
use tauri::utils::config::WindowEffectsConfig;
#[cfg(desktop)]
use tauri::window::Effect;

use tauri::Manager;

#[cfg(desktop)]
mod desktop;

#[cfg(desktop)]
mod plugin;

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
  let app = app.plugin(tauri_plugin_single_instance::init(|_, _, _| {
    std::thread::spawn(move || {
      use crate::desktop::show_window;

      // Lazy Working function
      show_window();
    });
  }));

  let app = app
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_ahqstore::init())
    .plugin(tauri_plugin_deep_link::init())
    .setup(|app| {
      #[cfg(desktop)]
      println!("[INFO] Running Desktop Setup");
      #[cfg(desktop)]
      desktop::setup(app)?;

      #[cfg(desktop)]
      app
        .handle()
        .plugin(tauri_plugin_window_state::Builder::default().build())?;

      #[cfg(mobile)]
      create_window(app);

      println!("[INFO] Running");
      Ok(())
    })
    .register_uri_scheme_protocol("ahqstoreplugin", |_ctx, request| {
      use tauri::http::Response;

      let _hwnd = _ctx.app_handle();

      let uri = request.uri();

      let path = uri.path();
      let path = percent_encoding::percent_decode_str(path).decode_utf8_lossy();
      let path: &str = path.as_ref();

      let _pt1 = path.get(1..=5).unwrap_or("");
      let _plugin_id = path.get(6..).unwrap_or("");


      let mut html = "".to_string();

      #[cfg(desktop)]
      match _pt1 {
        "conf/" => {
          html = plugin::get_html(_hwnd, _plugin_id, true);
        }
        "view/" => {
          html = plugin::get_html(_hwnd, _plugin_id, false);
        }
        perm => {
          if _ctx.webview_label() == "main" {
            match perm {
              "meta/" => {
                html = plugin::get_meta(_hwnd, _plugin_id);
              }
              "stat/" => {
                let (id, state) = _plugin_id.split_once("}::{").unwrap_or(("", ""));
                
                html = plugin::get_state(_hwnd, id, state);
              }
              "asst/" => {
                // Trusted process
                let (id, path) = _plugin_id.split_once("}::{").unwrap_or(("", ""));

                #[cfg(debug_assertions)]
                println!("[INFO] Getting {path} (parsed {_plugin_id:?})");

                html = plugin::get_script(_hwnd, id, path);
              }
              "plug/" => {
                html = plugin::get_plugin_names(_hwnd);
              }
              "inst/" => {
                let (id, path) = _plugin_id.split_once("}::{").unwrap_or(("", ""));

                #[cfg(debug_assertions)]
                println!("[INFO] Installing {path} (parsed {_plugin_id:?})");

                if let Some(_) = plugin::install_plugin(_hwnd, id, path) {
                  html = "OK".to_string();
                } else {
                  html = "NOK".to_string();
                }
              }
              _ => {}
            }
          }
        }
      }

      if &html == "" {
        Response::builder()
          .status(404)
          .header("Access-Control-Allow-Origin", "*")
          .body("Not Found".to_string().into_bytes())
          .unwrap()
      } else {
        Response::builder()
          .status(200)
          .header("Content-Type", "text/html")
          .header("Access-Control-Allow-Origin", "*")
          .body(html.into_bytes())
          .unwrap()
      }
    })
    .invoke_handler(tauri::generate_handler![get_state, set_state, open_plugin])
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

static PLOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

fn get_plugin_name(input: &str) -> &str {
  if let Some(rest) = input.strip_prefix("settings-plugin-") {
    rest
  } else if let Some(rest) = input.strip_prefix("plugin-") {
    rest
  } else {
    "None"
  }
}

fn is_alphanumeric(s: &str) -> bool {
  s.chars().all(|c| c.is_ascii_alphanumeric())
}

#[tauri::command]
async fn get_state(app: WebviewWindow, state: String) -> String {
  if app.label() == "main" {
    return "".into();
  }
  if !is_alphanumeric(&state) {
    return "".into();
  }

  let plugin = get_plugin_name(app.label());

  #[cfg(desktop)]
  return plugin::get_state(app.app_handle(), plugin, &state);

  #[cfg(not(desktop))]
  return "".into();
}

#[tauri::command]
async fn set_state(app: WebviewWindow, state: String, data: String) {
  if app.label() == "main" {
    return;
  }
  if !is_alphanumeric(&state) {
    return;
  }

  let plugin = get_plugin_name(app.label());

  #[cfg(desktop)]
  _ = app.emit_to(EventTarget::webview_window("main"), "state-update", plugin);

  #[cfg(desktop)]
  plugin::set_state(app.app_handle(), plugin, &state, data);
}

#[tauri::command]
async fn open_plugin(app: WebviewWindow, plugin: String, title: String, settings: bool) {
  let lock = PLOCK.lock().await;

  if app.label() != "main" {
    return;
  }

  #[cfg(desktop)]
  {
    use tauri::Url;

    #[cfg(not(debug_assertions))]
    let init_script = include_str!("../plugin-script/dist/bundle.js");

    #[cfg(debug_assertions)]
    let init_script = {
      use std::fs;

      fs::read_to_string(env!("PLUGIN_SCRIPT_PATH")).unwrap()
    };

    let label = if settings {
      format!("settings-plugin-{plugin}")
    } else {
      format!("plugin-{plugin}")
    };

    let url = if settings {
      format!("conf/{plugin}")
    } else {
      format!("view/{plugin}")
    };

    // Ignore errors
    _ = WebviewWindowBuilder::new(
      &app,
      label,
      (||{
        #[cfg(windows)]
        return tauri::WebviewUrl::CustomProtocol(Url::parse(
          format!("http://ahqstoreplugin.localhost/{url}").as_str()
        ).unwrap());

        #[cfg(not(windows))]
        return tauri::WebviewUrl::CustomProtocol(Url::parse(
          format!("ahqstoreplugin://{url}").as_str()
        ).unwrap());
      })()
    )
    .initialization_script(init_script)
    .min_inner_size(348.0, 700.0)
    .inner_size(1024.0, 760.0)
    .resizable(true)
    .title(title)
    .additional_browser_args("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --autoplay-policy=no-user-gesture-required")
    .build();
  }

  drop(lock);
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
