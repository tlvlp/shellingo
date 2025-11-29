use std::env;
use crate::app::AppState;

mod app;
mod events;
mod question_parser;
mod ui;
mod ui_setup_phase;
mod ui_practice_phase;
mod ui_shared;
mod question;
mod practice;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut args: Vec<String> = env::args().collect();
        // Clear the default argument containing the full executable path
        args.remove(0);
        let mut app = AppState::new(args);
        loop {
            terminal.draw(|frame| ui::draw_ui(frame, &mut app))?;
            if let Err(e) = events::handle_input(&mut app) {
                eprintln!("{:?}", e);
                break;
            }
        }
        Ok(())
    })
}
