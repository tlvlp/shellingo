use crate::app::AppState;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{Terminal, crossterm};
use std::error::Error;
use std::io;

mod app;
mod events;
mod question_parser;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    //Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state
    let mut app = AppState::new();

    // Main loop
    loop {
        terminal.draw(|frame| ui::draw_ui(frame, &app))?;
        if let Err(e) = events::handle_input(&mut app) {
            eprintln!("{:?}", e);
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}
