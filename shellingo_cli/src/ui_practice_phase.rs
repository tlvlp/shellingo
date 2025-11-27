use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
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

pub(crate) fn render_title_with_help_text(frame: &mut Frame, title_block: Block, draw_area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "[Tab] switch panes, [↑↓] navigate menu, [Enter] check answer, [Esc] "
        ).block(title_block).style(Style::new().dim()),

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
            .border_type(border)
            .border_style(Style::new().dim())
    )
        .highlight_symbol("> ")
        .highlight_style(style.fg(Color::Black).bg(Color::White))
        .scroll_padding(1)
}

pub(crate) fn render_practice_main(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    let (border, style) = ui_shared::get_style_for_component(UiComponent::PracticeMain, app);
    let [
        main_question_area,
        main_answer_area,
        main_status_area
    ] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3)
    ])
        .areas(draw_area);


    let question = app.practice_get_current_question_in_round()
        .borrow_mut()
        .clone();
    let question_text = question.question;

    frame.render_widget(get_generic_block(" Question: ", question_text, style, border), main_question_area);
    render_input(app, frame, style, border, main_answer_area);
    frame.render_widget(get_generic_block(" Practice status: ", app.practice_get_round_status_string(), style, border), main_status_area);

}

fn render_input(app: &mut AppState, frame: &mut Frame, style: Style, border: BorderType, area: Rect) {
    let width = area.width.max(3) - 3;
    let scroll = app.answer_input.visual_scroll(width as usize);
    let input = Paragraph::new(app.answer_input.value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(Block::bordered()
            .title(" Answer: ")
            .border_type(border)
            .padding(Padding::horizontal(1))
            .border_style(Style::new().dim())
        );

    let cursor = app.answer_input.visual_cursor().max(scroll) - scroll + 1;

    frame.set_cursor_position((area.x + 1 + cursor as u16, area.y + 1));
    frame.render_widget(input, area);
}

fn get_generic_block(title: &'_ str, contents: String, style: Style, border: BorderType) -> Paragraph<'_> {
    Paragraph::new(contents)
        .style(style)
        .block(
            Block::bordered()
                .title(title)
                .padding(Padding::horizontal(1))
                .border_type(border)
                .border_style(Style::new().dim())
        )
}