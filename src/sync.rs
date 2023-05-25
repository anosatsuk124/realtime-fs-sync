use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use anyhow::Context;

static CURRENT_HASH: Mutex<String> = Mutex::new(String::new());

pub fn sync_with_event(event: notify::Event) {
    let kind = event.kind;
    let paths = event.paths;

    use notify::event::EventKind;
    match kind {
        EventKind::Modify(_) => sync_with_content_changed(paths),
        EventKind::Create(_) => sync_with_content_changed(paths),
        EventKind::Remove(_) => sync_with_content_changed(paths),
        _ => {}
    }
}

pub fn sync_with_content_changed(paths: Vec<PathBuf>) {
    log::info!("Content(s) changed, syncing...");

    for path in paths {
        log::info!("Syncing {}", path.display());
        store_current_hash(&path).unwrap_or_else(|e| {
            log::error!("Failed to store current hash of {}: {}", path.display(), e);
        });
    }
}

pub fn store_current_hash(file: &Path) -> anyhow::Result<()> {
    let digest = sha256::try_digest(file)
        .with_context(|| format!("Failed to calculate SHA256 digest of {}", file.display()))?;
    let mut current_hash = CURRENT_HASH.lock().unwrap(); // FIXME: error handling
    *current_hash = digest;
    Ok(())
}
