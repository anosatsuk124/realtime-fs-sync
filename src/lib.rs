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

pub const DEFAULT_LOCALE: &str = "en-US";

pub fn init_locale() {
    let locale = if let Ok(locale) = env::var("LOCALE") {
        locale
    } else if let Some(locale) = sys_locale::get_locale() {
        available_locales()
            .iter()
            .find(|l| l.to_string() == locale)
            .map(|l| l.to_string())
            .unwrap_or(DEFAULT_LOCALE.to_string())
    } else {
        DEFAULT_LOCALE.to_string()
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
