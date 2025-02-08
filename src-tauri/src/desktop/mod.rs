use tauri::image::Image;
use tauri::menu::IconMenuItem;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::menu::PredefinedMenuItem;
use tauri::tray::TrayIconBuilder;
use tauri::tray::TrayIconEvent;
use tauri::{App, Emitter};
use tauri::{Listener, Manager};

use tauri_plugin_autostart::MacosLauncher;

use tauri_plugin_updater::UpdaterExt;

#[cfg(windows)]
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};

pub fn setup(app: &mut App) -> tauri::Result<()> {
  #[cfg(windows)]
  {
    let hwnd = app
      .get_webview_window("main")
      .expect("Impossible error")
      .hwnd()
      .unwrap();

    unsafe {
      //2: Mica, 3: Acrylic, 4: Mica Alt
      let attr = 2;
      let _ = DwmSetWindowAttribute(
        hwnd,
        DWMWINDOWATTRIBUTE(38),
        &attr as *const _ as _,
        std::mem::size_of_val(&attr) as u32,
      );
    }
  }

  let handle = app.handle();

  handle.plugin(tauri_plugin_autostart::init(
    MacosLauncher::LaunchAgent,
    Some(vec!["--hidden"]),
  ))?;

  handle.plugin(tauri_plugin_updater::Builder::new().build())?;

  handle.plugin(tauri_plugin_single_instance::init(|app, _, _| {
    let _ = app.get_webview_window("main").expect("Impossible").show();
  }))?;

  let handle = app.handle().clone();

  tauri::async_runtime::spawn(async move {
    if let Err(_) = update(handle).await {
      println!("Couldn't check update");
    }
  });

  if !should_be_hidden() {
    let handle = app.handle().clone();

    app.listen("loaded", move |_| {
      let _ = handle
        .get_webview_window("main")
        .expect("Impossible")
        .show();
    });
  }

  #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
  {
    use tauri_plugin_deep_link::DeepLinkExt;
    app.deep_link().register_all().expect("Unable to register");
  }

  TrayIconBuilder::with_id("main")
    .tooltip("AHQ Store is running")
    .icon(Image::from_bytes(include_bytes!("../../icons/icon.png"))?)
    .show_menu_on_left_click(false)
    .on_tray_icon_event(move |app, event| match event {
      TrayIconEvent::Click { .. } => {
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
