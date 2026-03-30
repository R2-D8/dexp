use std::path::PathBuf;

use anyhow::Result;
use crossterm::event;

use crate::explorer::{FileEntry, load_directory};
use crate::terminal::Tui;
use crate::ui::render;

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    exit: bool,
}

impl App {
    /// Create a new App instance with the current working directory.
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let entries = load_directory(&cwd)?;

        Ok(Self {
            cwd,
            entries,
            selected: 0,
            exit: false,
        })
    }

    /// Run the main event loop.
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| render(f, &self))?;
            self.handle_event()?;
        }
        Ok(())
    }

    /// Handle a keyboard event and update app state.
    fn handle_event(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => self.exit = true,
                    // crossterm::event::KeyCode::Down => self.select_next(),
                    // crossterm::event::KeyCode::Up => self.select_previous(),
                    // crossterm::event::KeyCode::Enter => self.enter_selected_directory()?,
                    // crossterm::event::KeyCode::Backspace => self.go_to_parent()?,
                    // crossterm::event::KeyCode::Char('r') => self.reload_entries()?,
                    _ => {}
                }
            }
        }
        Ok(())
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
