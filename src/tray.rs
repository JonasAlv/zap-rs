use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::{App, Manager, Result};
use tauri::image::Image;

fn show_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show()?;
        window.set_focus()?;
    } else {
        eprintln!("Window [main] not found when trying [show_window()].");
    }
    Ok(())
}

fn hide_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.hide()?;
    } else {
        eprintln!("Window [main] not found when trying [hide_window()].");
    }
    Ok(())
}

fn toggle_window<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
    toggle_item: &MenuItem<R>,
) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        if window.is_visible()? {
            hide_window(app_handle)?;
            toggle_item.set_text("Show")?;
        } else {
            show_window(app_handle)?;
            toggle_item.set_text("Hide")?;
        }
    } else {
        eprintln!("Window [main] not found when trying [toggle_window()]");
    }
    Ok(())
}

fn quit_app<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
    app_handle.exit(0);
    Ok(())
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
                let _ = quit_app(app_handle);
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