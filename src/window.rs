use tauri::{App, Result, WebviewUrl, WebviewWindowBuilder};

pub fn init_window<R: tauri::Runtime>(app: &App<R>) -> Result<()> {
    WebviewWindowBuilder::new(
        app,
        "main", // window label
        WebviewUrl::External("https://web.whatsapp.com".parse().expect("not found"))
    )
    .title("zap-rs")
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .fullscreen(false)
    .visible(true)
    .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0")
    .build()?;

    Ok(())
}
