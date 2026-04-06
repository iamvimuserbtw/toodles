mod app;
mod todo;
mod tui;
mod ui;

use color_eyre::eyre::Result;

use crate::{
    app::{App, RunningState},
    ui::render,
};

fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = App::new();
    app.seed_todo();

    while app.running_state != RunningState::Done {
        // Render the tui
        terminal.draw(|frame| render(frame, &mut app))?;

        // Handle events and map to a Message
        let mut current_message = App::handle_event()?;

        // Process updates as long as they return a non-None message
        while current_message.is_some() {
            current_message = app.update(current_message.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
