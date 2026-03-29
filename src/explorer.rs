use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

/// Load directory entries from the given path, sorted (directories first, then alphabetically).
pub fn load_directory(path: &Path) -> Result<Vec<FileEntry>> {
    todo!()
}
