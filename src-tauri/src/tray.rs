use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri::{App, Result};

fn show_window(app_handle: &tauri::AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

fn hide_window(app_handle: &tauri::AppHandle) {
  let window = app_handle.get_webview_window("main").unwrap();
  window.hide().unwrap();
}

fn quit_app(app_handle: &tauri::AppHandle) {
    app_handle.exit(0);
}

pub fn init_tray(app: &App) -> Result<()> {
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app_handle, event| match event.id.as_ref() {
            "show" => show_window(app_handle),
            "hide" => hide_window(app_handle),
            "quit" => quit_app(app_handle),
            other => eprintln!("Unimplemented menu id: {:?}", other),
        })
        .build(app)?;

    Ok(())
}
