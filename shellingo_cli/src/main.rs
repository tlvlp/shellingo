use std::env;
use crate::app::AppState;

mod app;
mod events;
mod question_parser;
mod ui;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let args: Vec<String> = env::args().collect();
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
