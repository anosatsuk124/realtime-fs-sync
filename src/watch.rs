use std::path::PathBuf;

use notify::{RecursiveMode, Watcher};

/// Watch a directory for changes with provided mode and handler for events.
pub fn watch_dir<F>(path: PathBuf, mode: RecursiveMode, handler: F) -> anyhow::Result<()>
where
    F: Fn(notify::Event) -> () + std::marker::Send + 'static,
{
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            log::info!("watch event: {:?}", event);
            handler(event);
        }
        Err(e) => log::error!("watch error: {:?}", e),
    })?;

    watcher.watch(&path, mode)?;

    Ok(())
}
