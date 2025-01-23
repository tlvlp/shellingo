use crate::app::{AppState, MenuItem};
use ratatui::text::Span;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Tabs},
    Frame,
};
use std::borrow::Cow;
use strum::IntoEnumIterator;

pub fn draw_ui(frame: &mut Frame, app: &AppState) {
    // Draw menu bar
    let menu = Tabs::new(
        MenuItem::iter()
            .map(|mi| Cow::from(mi.to_string()))
            .map(Span::from)
            .collect::<Vec<_>>(),
    )
    .select(app.get_active_menu_position())
    .block(Block::default().title("Menu").borders(Borders::all()));
    
    // Split the layout into two areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());
    
    frame.render_widget(menu, chunks[0]);
}
