use std::io::Stdout;

use anyhow::Result;
use crossterm::event::Event;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal: enable raw mode and enter alternate screen.
pub fn init() -> Result<Tui> {
    todo!()
}

/// Restore the terminal: disable raw mode and leave alternate screen.
pub fn restore() -> Result<()> {
    todo!()
}

/// Read the next keyboard or other event from the terminal.
pub fn next_event() -> Result<Event> {
    todo!()
}
