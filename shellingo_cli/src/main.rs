use std::error::Error;
use crate::app::AppState;

mod app;
mod events;
mod question_parser;
mod ui;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = AppState::new();
        loop {
            terminal.draw(|frame| ui::draw_ui(frame, &app))?;
            if let Err(e) = events::handle_input(&mut app) {
                eprintln!("{:?}", e);
                break;
            }
        }
        Ok(())
    })
}
