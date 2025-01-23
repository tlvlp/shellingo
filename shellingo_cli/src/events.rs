use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event;
use crate::app::{AppState, MenuItem};

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Left => app.navigate_to_prev_menu(),
                KeyCode::Right => app.navigate_to_next_menu(),
                KeyCode::Esc => {
                    app.navigate_to(MenuItem::Exit);
                    return Err(Box::from("Exiting application."))
                },
                _ => {}
            }
        }
    }
    Ok(())
}