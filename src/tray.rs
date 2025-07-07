use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::{App, Manager, Result};
use tauri::image::Image;

fn select_main_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<tauri::WebviewWindow<R>> {
    app_handle
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WebviewNotFound) 
}

//saved for future use
// fn show_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
//     let window = select_main_window(app_handle)?;
//     window.show()?;
//     window.set_focus()?;
//     Ok(())
// }

// fn hide_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
//     let window = select_main_window(app_handle)?;
//     window.hide()?;
//     Ok(())
// }

fn toggle_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>, toggle_item: &MenuItem<R>) -> Result<()> {
    let window = select_main_window(app_handle)?;
    if window.is_visible()? {
        window.hide()?;
        toggle_item.set_text("Show")?;
    } else {
        window.show()?;
        window.set_focus()?;
        toggle_item.set_text("Hide")?;
    }
    Ok(())
}

fn quit_app<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) {
    app_handle.exit(0);
}

pub fn init_tray<R: tauri::Runtime>(app: &App<R>) -> Result<()> {
    let toggle_item = MenuItem::with_id(app, "toggle", "Hide", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_item, &quit_item])?;

    let toggle_item_clone_for_menu = toggle_item.clone();
    let toggle_item_clone_for_click = toggle_item.clone();

    TrayIconBuilder::new()
        .icon(Image::from_bytes(include_bytes!("../icons/tray_64x64.png")).expect("Failed to load tray icon"))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| match event.id.as_ref() {
            "toggle" => {
                let _ = toggle_window(app_handle, &toggle_item_clone_for_menu);
            }
            "quit" => {
                quit_app(app_handle);
            }
            _ => {
                eprintln!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(move |tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                let _ = toggle_window(app, &toggle_item_clone_for_click);
            }
        })
        .build(app)?;

    Ok(())
}