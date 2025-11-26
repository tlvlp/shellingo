use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui_widgets::block::{Block, Padding};
use ratatui_widgets::borders::BorderType;
use ratatui_widgets::list::{List, ListItem};
use ratatui_widgets::paragraph::Paragraph;
use strum::{EnumMessage, IntoEnumIterator};
use crate::app::{AppState, PracticeControlOptions, UiComponent};
use crate::{ui_shared};

pub(crate) fn get_body_constraints() -> [Constraint; 2] {
    [Constraint::Length(22), Constraint::Fill(1)]
}

pub(crate) fn render_title_with_tooltips(frame: &mut Frame, title_block: Block,  draw_area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "[Tab] switch panes, [↑↓] navigate, [Enter] check answer"
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
    let (border, style) = ui_shared::get_style_for_component(UiComponent::PracticeControls, app);
    let controls = PracticeControlOptions::iter()
        .map(|control | {
            let readable_control_name = control.get_message().unwrap().to_string();
            ListItem::new(readable_control_name).style(style)
        })
        .collect::<Vec<ListItem>>();
    List::new(controls)
        .style(style)
    .block(
        Block::bordered()
            .padding(Padding::horizontal(1))
            .border_type(border),
    )
        .highlight_symbol("> ")
        .highlight_style(style.fg(Color::Black).bg(Color::White))
        .scroll_padding(1)
}

pub(crate) fn render_practice_main(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    let (border, style) = ui_shared::get_style_for_component(UiComponent::PracticeMain, app);
    let practice_main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3)
        ])
        .split(draw_area);
    let practice_main_layout_question = practice_main_layout[0];
    let practice_main_layout_answer = practice_main_layout[1];
    let practice_main_layout_status = practice_main_layout[2];

    let question = app.practice_get_current_question_in_round()
        .borrow_mut()
        .clone();
    let question_text = question.question;

    frame.render_widget(get_question_box(question_text, style, border), practice_main_layout_question);
    frame.render_widget(get_answer_box("Answer placeholder".to_string(), style, border), practice_main_layout_answer);
    frame.render_widget(get_status_box(app.practice_get_round_status_string(), style, border), practice_main_layout_status);

}

fn get_question_box<'a>(question: String, style: Style, border: BorderType) -> Paragraph<'a> {
    Paragraph::new(question)
        .style(style)
        .block(
            Block::bordered()
                .title(" Question ")
                .padding(Padding::horizontal(1))
                .border_type(border)
        )
}

fn get_answer_box<'a>(question: String, style: Style, border: BorderType) -> Paragraph<'a> {
    Paragraph::new(question)
        .style(style)
        .block(
            Block::bordered()
                .title(" Answer ")
                .padding(Padding::horizontal(1))
                .border_type(border)
        )
}

fn get_status_box<'a>(question: String, style: Style, border: BorderType) -> Paragraph<'a> {
    Paragraph::new(question)
        .style(style)
        .block(
            Block::bordered()
                .title(" Practice status ")
                .padding(Padding::horizontal(1))
                .border_type(border)
        )
}