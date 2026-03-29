use std::path::PathBuf;

use anyhow::Result;

use crate::explorer::FileEntry;
use crate::terminal::Tui;

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected: usize,
}

impl App {
    /// Create a new App instance with the current working directory.
    pub fn new() -> Result<Self> {
        todo!()
    }

    /// Run the main event loop.
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        todo!()
    }

    /// Handle a keyboard event and update app state.
    fn handle_event(&mut self) -> Result<()> {
        todo!()
    }

    /// Move selection down in the file list.
    fn select_next(&mut self) {
        todo!()
    }

    /// Move selection up in the file list.
    fn select_previous(&mut self) {
        todo!()
    }

    /// Enter the selected directory if it's a directory.
    fn enter_selected_directory(&mut self) -> Result<()> {
        todo!()
    }

    /// Navigate to the parent directory.
    fn go_to_parent(&mut self) -> Result<()> {
        todo!()
    }

    /// Reload directory entries from disk.
    fn reload_entries(&mut self) -> Result<()> {
        todo!()
    }
}
