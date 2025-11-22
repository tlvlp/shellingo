use std::any::Any;
use ratatui::Frame;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::prelude::{Color, Style};
use ratatui_widgets::block::{Block, Padding};
use ratatui_widgets::list::{List, ListItem};
use ratatui_widgets::paragraph::Paragraph;
use strum::{EnumMessage, IntoEnumIterator};
use crate::app::{AppState, PracticeControlOptions, UiComponent};
use crate::ui_shared;

pub(crate) fn get_body_constraints() -> [Constraint; 2] {
    [Constraint::Percentage(10), Constraint::Percentage(90)]
}

pub(crate) fn render_title_with_tooltips(frame: &mut Frame, title_block: Block,  draw_area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "[Tab] switch panes, [↑↓] navigate"
        ).block(title_block),

        draw_area
    );
}

pub(crate) fn render_practice_controls(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    frame.render_stateful_widget(
        get_practice_control_list(app),
        draw_area,
        &mut app.practice_controls_list_state
    );
}

fn get_practice_control_list<'a>(app: &mut AppState) -> List<'a>{
    let controls = PracticeControlOptions::iter()
        .map(|control | {
            let readable_control_name = control.get_message().unwrap().to_string();
            ListItem::new(readable_control_name)
        })
        .collect::<Vec<ListItem>>();
    List::new(controls)
    .block(
        Block::bordered()
            .padding(Padding::horizontal(1))
            .border_type(ui_shared::select_border_for_component(UiComponent::PracticeControls, app)),
    )
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::Black).bg(Color::White))
        .scroll_padding(1)
}

pub(crate) fn render_practice_main(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    let placeholder = Paragraph::new("Placeholder")
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_type(ui_shared::select_border_for_component(UiComponent::PracticeMain, app))
        );
    frame.render_widget(placeholder, draw_area);

}