rust_i18n::i18n!("locales", fallback = "en-US");

pub mod app;
pub mod sync;
pub mod watch;

use std::env;

pub fn init_logger() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_ok() {
        drop(env_logger::try_init()?);
        log::info!("Logger initialized with env_logger.");
    } else {
        drop(egui_logger::init());
        log::info!("Logger initialized with egui_logger.");
    }

    Ok(())
}

pub fn init_locale() {
    let locale = if let Ok(locale) = env::var("LOCALE") {
        locale
    } else if let Some(locale) = sys_locale::get_locale() {
        locale
    } else {
        "en-US".to_string()
    };

    rust_i18n::set_locale(&locale);
    log::info!("Locale set to {}.", locale);
}

pub fn run() -> anyhow::Result<()> {
    init_logger()?;
    init_locale();

    app::app()?;

    Ok(())
}
