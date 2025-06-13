use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder};
use tauri_plugin_opener;
use tauri_plugin_shell;

pub fn run() {

    tauri::Builder::default()

          // a tray
        .setup(|app| {

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        //    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(()) })

        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("failed to run zap-rs");
}