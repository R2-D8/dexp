use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

/// Load directory entries from the given path, sorted (directories first, then alphabetically).
pub fn load_directory(path: &Path) -> Result<Vec<FileEntry>> {
    let mut res_entries = Vec::new();
    for entry in std::fs::read_dir(path).context("failed to read directory")? {
        let entry = entry.context("failed to read directory entry")?;
        let metadata = entry.metadata().context("failed to read metadata")?;
        res_entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
        });
    }

    Ok(res_entries)
}
