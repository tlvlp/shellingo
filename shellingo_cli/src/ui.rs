use crate::app::{AppPhase, AppState, UiComponent};
use crate::{ ui_setup_phase, ui_practice_phase};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Padding},
    Frame,
};
use ratatui::layout::{Alignment, Flex, Rect};
use ratatui_widgets::borders::BorderType;
use ratatui_widgets::clear::Clear;
use ratatui_widgets::paragraph::Paragraph;

pub fn draw_ui(frame: &mut Frame, app: &mut AppState) {
    let app_phase = app.get_app_phase_for_active_component();

    // Split the main layout
    let [help_area, body_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
            .areas(frame.area());

    // Title
    let title_block = Block::bordered()
        .title("[ Shellingo ]")
        .border_type(BorderType::Plain)
        .border_style(Style::new().dim())
        .padding(Padding::horizontal(1));

    // Body layout
    let [body_left_area, body_right_area]  = Layout::horizontal(
        match app_phase {
            AppPhase::Setup => ui_setup_phase::get_body_constraints(),
            AppPhase::Practice => ui_practice_phase::get_body_constraints(),
        }
    ).areas(body_area);

    // Render contents
    match app_phase {
        AppPhase::Setup => {
            ui_setup_phase::render_title_with_help_text(frame, title_block, help_area);
            ui_setup_phase::render_group_list_with_scrollbar(app, frame, body_left_area);
            ui_setup_phase::render_question_table_with_scrollbar(app, frame, body_right_area);
        },
        AppPhase::Practice => {
            ui_practice_phase::render_title_with_help_text(frame, title_block, help_area);
            ui_practice_phase::render_practice_controls(app, frame, body_left_area);
            ui_practice_phase::render_practice_main(app, frame, body_right_area);
        }
    };

    // Exit popup
    if app.get_active_component() == UiComponent::ExitPopup {
        let popup_area = popup_area(frame.area(), 37, 6);
        let popup = get_exit_popup();
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup, popup_area);

    }
}

fn get_exit_popup<'a>() -> Paragraph<'a> {
    Paragraph::new("Do you want to exit Shellingo?\n\
                        [Enter] Yes, [Esc] No")
        .block(Block::bordered()
            .title("[ Exit ]")
            .padding(Padding::horizontal(1))
            .padding(Padding::vertical(1))
            .border_type(BorderType::Thick)
            .style(Style::default().fg(Color::Red))
        ).alignment(Alignment::Center)
}

fn popup_area(area: Rect, x_len: u16, y_len: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(y_len)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(x_len)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}