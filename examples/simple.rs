use cef_simple::{Cef, WindowOptions};
use simplelog::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always
    )])
    .unwrap();

    let cef = Cef::initialize(None, true)?;

    cef.open_window(WindowOptions {
        url: "file://D:/Rust/filer/public/index.html".to_owned(),
        title: None,//Some("CEF Simple".to_string()),
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
