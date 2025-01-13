use tauri::tray::TrayIconBuilder;
use tauri_plugin_autostart::MacosLauncher;
use tauri::{App, Emitter};
use tauri::{Listener, Manager};
use tauri::image::Image;
use tauri::tray::TrayIconEvent;
use tauri::menu::Menu;
use tauri::menu::IconMenuItem;
use tauri::menu::PredefinedMenuItem;
use tauri::menu::MenuItem;

pub fn setup(app: &mut App) -> tauri::Result<()> {
  app.handle().plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))?;
  app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;

  if !should_be_hidden() {
    let handle = app.handle().clone();

    app.listen("loaded", move |_| {
      let _ = handle.get_webview_window("main").expect("Impossible").show();
    });
  }

  TrayIconBuilder::with_id("main")
    .tooltip("AHQ Store is running")
    .icon(Image::from_bytes(include_bytes!("../../icons/icon.png"))?)
    .show_menu_on_left_click(false)
    .on_tray_icon_event(move |app, event| match event {
      TrayIconEvent::Click { .. } => {
        let _ = app.app_handle().get_webview_window("main").expect("Impossible").show();
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
            None::<String>
          ).expect("Unable to construct"),
          &PredefinedMenuItem::separator(app).expect("Unable to construct"),
          &MenuItem::with_id(
            app, 
            "show",
            "Open Application", 
            true,
            None::<String>
          ).expect("Unable to construct"),
          &MenuItem::with_id(
            app, 
            "check",
            "Check for updates", 
            true,
            None::<String>
          ).expect("Unable to construct"),
          &PredefinedMenuItem::separator(app).expect("Unable to construct"),
          &MenuItem::with_id(
            app, 
            "quit",
            "Quit", 
            true,
            None::<String>
          ).expect("Unable to construct"),
        ]
      )
      .expect("Unable to construct")
    )
    .on_menu_event(|app, event| {
      let id = event.id.0;

      match id.as_str() {
        "quit" => {
          app.exit(0);
        }
        "check" => {
          let _ = app.get_webview_window("main").expect("Impossible").emit("update", "");
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