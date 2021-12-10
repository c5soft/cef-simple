use cef_simple::{Cef, WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cef = Cef::initialize(None, true)?;

    cef.open_window(WindowOptions {
        url: "D:/Rust/filer/public/index.html".to_owned(),
        title: Some("CEF Simple".to_string()),
        window_icon: Some(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/icon.png"
        ))),
        window_app_icon: Some(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/icon.png"
        ))),
        ..WindowOptions::default()
    })?;

    cef.run()?;

    Ok(())
}
