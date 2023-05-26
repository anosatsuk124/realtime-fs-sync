rust_i18n::i18n!("locales", fallback = "en-US");

pub mod app;
pub mod sync;
pub mod watch;

use std::env;

pub fn init_logger(mode: &RunMode) -> anyhow::Result<()> {
    if mode != &RunMode::Gui {
        drop(env_logger::try_init()?);
        log::info!("Logger initialized with env_logger.");
    } else if env::var("RUST_LOG").is_ok() {
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
        locale
    } else {
        DEFAULT_LOCALE.to_string()
    };

    let locale = available_locales()
        .iter()
        .find(|l| l.to_string() == locale)
        .map(|l| l.to_string())
        .unwrap_or(DEFAULT_LOCALE.to_string());

    rust_i18n::set_locale(&locale);
    log::info!("Locale set to {}.", locale);
}

pub fn gui_run() -> anyhow::Result<()> {
    init_logger(&RunMode::Gui)?;
    init_locale();

    app::app()?;

    Ok(())
}

/// NOTE: Not implemented yet.
pub fn cli_run(mode: RunMode) -> anyhow::Result<()> {
    init_logger(&mode)?;
    init_locale();

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum RunMode {
    Gui,
    Deamon,
    /// NOTE: Not implemented yet.
    Interactive,
}

const RUN_MODE_ENV: &str = "RUN_MODE";
pub fn run() -> anyhow::Result<()> {
    match env::var(RUN_MODE_ENV) {
        Ok(mode) => {
            let mode = mode.as_str();
            match mode {
                "deamon" => cli_run(RunMode::Deamon),
                "interactive" => cli_run(RunMode::Interactive),
                "gui" => gui_run(),
                _ => {
                    anyhow::bail!("Invalid RUN_MODE: {}", mode)
                }
            }
        }
        Err(_) => gui_run(),
    }
}
