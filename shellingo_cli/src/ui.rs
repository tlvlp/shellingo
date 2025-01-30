use crate::app::AppState;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Stylize;
use ratatui::text::ToText;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Tabs},
    Frame,
};
use ratatui::widgets::{Padding, Paragraph};

pub fn draw_ui(frame: &mut Frame, app: &AppState) {

    // Split the layout into two areas
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let menu = Tabs::new(app.menu_item_spans.clone())
        .select(app.get_active_menu_position())
        .block(Block::default().title("[ Shellingo ]").borders(Borders::all()));

    let body = Paragraph::new(
        app.active_screen.to_text().light_yellow()
    ).block(Block::bordered().padding(Padding::vertical(1)));

    frame.render_widget(menu, main_layout[0]);
    frame.render_widget(body, main_layout[1]);
}

/// Create a centered Rect using up certain percentage of the available rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(get_centered_constraints(percent_x))
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(get_centered_constraints(percent_y))
        .split(vertical_layout[1])[1]
}

fn get_centered_constraints(percent: u16) -> [Constraint; 3] {
    [
        Constraint::Percentage((100 - percent) / 2),
        Constraint::Percentage(percent),
        Constraint::Percentage((100 - percent) / 2),
    ]
}
