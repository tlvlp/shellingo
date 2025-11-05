use crate::app::{AppState, UiComponent};
use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::style::{Style};
use ratatui::symbols::border::Set;
use ratatui::text::ToText;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    symbols,
    widgets::{Block, Padding, Tabs},
};
use ratatui_widgets::list::{List};

pub fn draw_ui(frame: &mut Frame, app: &mut AppState) {
    // Split the layout into two areas
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let menu = Tabs::new(app.menu_item_spans.clone())
        .select(app.get_active_menu_position())
        .block(
            Block::bordered()
                .title("[ Shellingo ]")
                .border_set(select_border_for(UiComponent::Menu, app)),
        );
    
    app.file_list.state.select_first();

    let body = List::new(app.file_list.items.to_owned())
        .block(
            Block::bordered()
                .padding(Padding::vertical(1))
                .border_set(select_border_for(UiComponent::Body, app)),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::Black).bg(Color::White));

    frame.render_widget(menu, main_layout[0]);
    frame.render_stateful_widget(body, main_layout[1], &mut app.file_list.state);
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

fn select_border_for<'a>(component: UiComponent, app: &AppState) -> Set<'a> {
    if app.focused_component == component {
        symbols::border::DOUBLE
    } else {
        symbols::border::PLAIN
    }
}
