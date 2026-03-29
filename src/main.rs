mod app;
mod explorer;
mod terminal;
mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::new()?;

    let run_result = app.run(&mut terminal);
    let restore_result = terminal::restore();

    run_result.and(restore_result)
}
