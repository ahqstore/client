use tauri::image::Image;
use tauri::menu::IconMenuItem;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::menu::PredefinedMenuItem;
use tauri::tray::TrayIconBuilder;
use tauri::tray::TrayIconEvent;
use tauri::utils::config::WindowEffectsConfig;
use tauri::webview::WebviewWindowBuilder;
use tauri::window::Effect;
use tauri::{App, Emitter};
use tauri::{Listener, Manager};

use tauri_plugin_autostart::MacosLauncher;

use tauri_plugin_updater::UpdaterExt;

#[cfg(windows)]
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};

pub fn setup(app: &mut App) -> tauri::Result<()> {
//   #[cfg(windows)]
//   {
//     let hwnd = app
//       .get_webview_window("main")
//       .expect("Impossible error")
//       .hwnd()
//       .unwrap();

//     unsafe {
//       //2: Mica, 3: Acrylic, 4: Mica Alt
//       let attr = 2;
//       let _ = DwmSetWindowAttribute(
//         hwnd,
//         DWMWINDOWATTRIBUTE(38),
//         &attr as *const _ as _,
//         std::mem::size_of_val(&attr) as u32,
//       );
//     }
//   }

  let handle = app.handle();

  println!("Autostart");
  handle.plugin(tauri_plugin_autostart::init(
    MacosLauncher::LaunchAgent,
    Some(vec!["--hidden"]),
  ))?;

  println!("Updater");
  handle.plugin(tauri_plugin_updater::Builder::new().build())?;

  println!("Single Instance");
  handle.plugin(tauri_plugin_single_instance::init(|app, _, _| {
    // TODO: fix
    let _ = app.get_webview_window("main").expect("Impossible").show();
  }))?;

  let handle = app.handle().clone();

  println!("Checking for update");
  tauri::async_runtime::spawn(async move {
    if let Err(_) = update(handle).await {
      println!("Couldn't check update");
    }
  });

  let handle = app.handle().clone();

  // Show everytime
  app.listen("loaded", move |_| {
    let _ = handle
      .get_webview_window("main")
      .expect("Impossible")
      .show();
  });

  println!("Creating Window");
  if !should_be_hidden() {
    create_window(&mut app.handle().clone());
  }

  println!("Deep Linking");
  #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
  {
    use tauri_plugin_deep_link::DeepLinkExt;
    app.deep_link().register_all().expect("Unable to register");
  }

  println!("Building Tray Icon");
  TrayIconBuilder::with_id("main")
    .tooltip("AHQ Store is running")
    .icon(Image::from_bytes(include_bytes!("../../icons/icon.png"))?)
    .show_menu_on_left_click(false)
    .on_tray_icon_event(move |app, event| match event {
      TrayIconEvent::Click { .. } => {
        // TODO: Fix show
        let _ = app
          .app_handle()
          .get_webview_window("main")
          .expect("Impossible")
          .show();
      }
      _ => {}
    })
    .menu(
      &Menu::with_id_and_items(
        app,
        "main_menu",
        &[
          &IconMenuItem::new(
            app,
            "AHQ Store",
            false,
            Some(Image::from_bytes(include_bytes!("../../icons/icon.png"))?),
            None::<String>,
          )
          .expect("Unable to construct"),
          &PredefinedMenuItem::separator(app).expect("Unable to construct"),
          &MenuItem::with_id(app, "show", "Open Application", true, None::<String>)
            .expect("Unable to construct"),
          &MenuItem::with_id(app, "check", "Check for updates", true, None::<String>)
            .expect("Unable to construct"),
          &PredefinedMenuItem::separator(app).expect("Unable to construct"),
          &MenuItem::with_id(app, "quit", "Quit", true, None::<String>)
            .expect("Unable to construct"),
        ],
      )
      .expect("Unable to construct"),
    )
    .on_menu_event(|app, event| {
      let id = event.id.0;

      match id.as_str() {
        "quit" => {
          app.exit(0);
        }
        "check" => {
          let _ = app
            .get_webview_window("main")
            .expect("Impossible")
            .emit("update", "");
        }
        "show" => {
          let _ = app.get_webview_window("main").expect("Impossible").show();
        }
        _ => {}
      }
    })
    .build(app)
    .expect("Failed to build tray icon");

  Ok(())
}

pub(crate) fn remove_window(app: &tauri::AppHandle) {
  if let Some(x) = app.get_webview_window("main") {
    _ = x.close();
  }
}

pub(crate) fn create_window(app: &mut tauri::AppHandle) {
  if app.get_webview_window("main").is_some() {
    return;
  }

  _ = WebviewWindowBuilder::new(
    app,
    "main",
    tauri::WebviewUrl::App("/".into())
  )
    .center()
    .min_inner_size(348.0, 700.0)
    .inner_size(1024.0, 760.0)
    .resizable(true)
    .prevent_overflow()
    .title("AHQ Store Neo")
    .transparent(true)
    .effects(
      WindowEffectsConfig {
        effects: vec![Effect::Mica],
        state: None,
        radius: None,
        color: None,
      }
    )
    .additional_browser_args("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --autoplay-policy=no-user-gesture-required")
    .build()
    .unwrap();
}

fn should_be_hidden() -> bool {
  if let Some(args) = std::env::args().last() {
    if args == "--hidden" {
      return true;
    }
  }

  false
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
        },
      )
      .await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}
