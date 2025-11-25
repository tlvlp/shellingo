use ratatui::crossterm::style::Stylize;
use ratatui::Frame;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::prelude::{Color, Style};
use ratatui_widgets::block::{Block, Padding};
use ratatui_widgets::list::{List, ListItem};
use ratatui_widgets::paragraph::Paragraph;
use ratatui_widgets::table::{Row, Table};
use crate::app::{AppState, UiComponent};
use crate::ui_shared;

pub(crate) fn get_body_constraints() -> [Constraint; 2] {
    [Constraint::Percentage(20), Constraint::Percentage(80)]
}

pub(crate) fn render_title_with_tooltips(frame: &mut Frame, title_block: Block,  draw_area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "[Tab] switch panes, [↑↓←→] navigate, [Enter/Space] select items, [P] start practice"
        ).block(title_block),

        draw_area
    );
}

pub(crate) fn render_group_list_with_scrollbar(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    let (groups_list, groups_list_len) = get_question_group_list(app);
    frame.render_stateful_widget(
        groups_list,
        draw_area,
        &mut app.question_group_list_state
    );
    frame.render_stateful_widget(
        ui_shared::get_new_scrollbar(),
        draw_area.inner(Margin {vertical: 1, horizontal: 0}), // Draw inside the same area
        &mut app.question_group_list_scrollbar_state.content_length(groups_list_len)
            .position(app.question_group_list_state.selected().unwrap_or(0)),
    );
}

fn get_question_group_list<'a>(app: &mut AppState) -> (List<'a>, usize) {
    let (border, style) = ui_shared::get_style_for_component(UiComponent::GroupSelector, app);
    let list = List::new(
        app.questions_by_groups
            .iter()
            .map(| (group_name, group_details)| {
                let selection_postfix = if group_details.is_active { " *"} else { "" };
                ListItem::new(format!("{}{}",group_name.clone(), selection_postfix))
                    .style(
                        if group_details.is_active { style.bold().fg(Color::Green) }
                        else { style }
                    )
            })
    )
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_type(border),
        )
        .highlight_symbol("> ")
        .highlight_style(style.fg(Color::Black).bg(Color::White))
        .scroll_padding(1);

    let list_len = list.len();
    (list, list_len)
}

pub(crate) fn render_question_table_with_scrollbar(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    let (questions_table, questions_table_len) = get_question_table(app);
    frame.render_stateful_widget(
        questions_table,
        draw_area,
        &mut app.question_table_state
    );
    frame.render_stateful_widget(
        ui_shared::get_new_scrollbar(),
        draw_area.inner(Margin {vertical: 1, horizontal: 0}), // Draw inside the same area
        &mut app.question_table_scrollbar_state.content_length(questions_table_len)
            .position(app.question_table_state.selected().unwrap_or(0)),
    );
}

fn get_question_table<'a>(app: &mut AppState) -> (Table<'a>, usize) {
    let (border, style) = ui_shared::get_style_for_component(UiComponent::QuestionSelector, app);
    let rows = app.setup_get_questions_for_selected_group()
        .into_iter()
        .map(|q| Row::new([
            q.borrow().question.clone(), {
                format!("➔ {:?}", q.borrow().answers)
                    .replace("{", "")
                    .replace("}", "")
            }
        ]));
    let question_count = rows.len();
    let widths = [Constraint::Fill(1), Constraint::Fill(1)];
    let table = Table::new(rows, widths)
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_type(border)
        )
        .row_highlight_style(style.fg(Color::Black).bg(Color::White)).style(style);

    (table, question_count)
}
