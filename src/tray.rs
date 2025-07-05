use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, Manager, Result};
use tauri::image::Image;

fn show_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show()?;
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

fn quit_app<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) {
    app_handle.exit(0);
}

pub fn init_tray<R: tauri::Runtime>(app: &App<R>) -> Result<()> {
    let toggle_item = MenuItem::with_id(app, "toggle", "Hide", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(Image::from_bytes(include_bytes!("../icons/tray_64x64.png")).unwrap())
        .menu(&menu)
        .on_menu_event({
            let toggle_item = toggle_item.clone(); 
            move |app_handle, event| match event.id.as_ref() { 
                "toggle" => {
                    let _ = toggle_window(app_handle, &toggle_item); 
                },
                "quit" => quit_app(app_handle),
                _ => {
                    eprintln!("menu item {:?} not handled", event.id);
                }
            }
        })
        
        .build(app)?;

    Ok(())
}
