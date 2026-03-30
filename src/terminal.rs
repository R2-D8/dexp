use std::io::Stdout;

use anyhow::Result;
use crossterm::event::Event;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal: enable raw mode and enter alternate screen.
pub fn init() -> Result<Tui> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(std::io::stdout()))?)
}

/// Restore the terminal: disable raw mode and leave alternate screen.
pub fn restore() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

/// Read the next keyboard or other event from the terminal.
pub fn next_event() -> Result<Event> {
    todo!()
}
