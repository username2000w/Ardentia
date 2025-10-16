use color_eyre::{eyre::Ok, Result};
use ardentia::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::default();

    app.run(terminal)?;

    ratatui::restore();

    Ok(())
}
