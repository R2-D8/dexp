use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// Render the UI given the current app state.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let title = format!("dexp | {}", app.cwd.display());
    let block = Block::default().title(title).borders(Borders::ALL);
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let selected = if app.entries.is_empty() {
        0
    } else {
        app.selected.saturating_add(1)
    };

    let body = format!("entries: {}\nselected: {}", app.entries.len(), selected);
    frame.render_widget(Paragraph::new(body), inner);
}
