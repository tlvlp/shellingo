use crate::app::{AppPhase, AppState, UiComponent};
use ratatui::prelude::Color;
use ratatui::style::{Style};
use ratatui::symbols::border::Set;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    symbols,
    widgets::{Block, Padding},
};
use ratatui::layout::{Alignment, Flex, Rect};
use ratatui_widgets::clear::Clear;
use ratatui_widgets::list::{List, ListItem};
use ratatui_widgets::paragraph::Paragraph;
use ratatui_widgets::table::{Row, Table};

pub fn draw_ui(frame: &mut Frame, app: &mut AppState) {
    // Split the main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());
    let main_layout_header = main_layout[0];
    let main_layout_body = main_layout[1];

    // Header
    frame.render_widget(
        Paragraph::new("[Tab] Switch between panes, [↑↓] navigate inside lists, [Enter/Space] select items")
            .block(Block::bordered().title("[ Shellingo ]"))
        , main_layout_header
    );

    // Body
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_layout_body);
    let body_layout_left = body_layout[0];
    let body_layout_right = body_layout[1];
    match app.get_app_phase_for_active_component() {
        AppPhase::Setup => {
            frame.render_stateful_widget(get_question_group_list(app), body_layout_left, &mut app.question_group_list_state);
            frame.render_stateful_widget(get_question_table(app), body_layout_right, &mut app.question_table_state);
        }
        // _ => {
        //     frame.render_widget(get_no_items_found(), body_layout_left);
        // }
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
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(Color::Red))
        ).alignment(Alignment::Center)
}

// fn get_no_items_found<'a>() -> Paragraph<'a> {
//     Paragraph::new("No items found")
//         .block(Block::bordered()
//                    .padding(Padding::horizontal(1))
//                    .border_set(symbols::border::PLAIN),
//         )
// }

fn get_question_group_list<'a>(app: &mut AppState) -> List<'a> {
    List::new(
        app.question_groups
            .iter()
            .map(| group_details| {
                let selection_postfix = if group_details.is_active { " *"} else { "" };
                ListItem::new(format!("{}{}",group_details.group_name.clone(), selection_postfix))
                    .style(
                        if group_details.is_active { Style::default().bold().fg(Color::Green) }
                        else { Style::default() }
                    )
            })
    )
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_set(select_border_for_component(UiComponent::GroupSelector, app)),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::Black).bg(Color::White))
}

fn get_question_table<'a>(app: &mut AppState) -> Table<'a> {
    let rows = app.get_questions_for_selected_group()
        .into_iter()
        .map(|q| Row::new([q.question, format!("{:?}", q.solutions)]));
    let widths = [Constraint::Fill(2), Constraint::Fill(8)];
    Table::new(rows, widths)
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_set(select_border_for_component(UiComponent::QuestionSelector, app))
        )
        .highlight_symbol("> ")
        .row_highlight_style(Style::new().fg(Color::Black).bg(Color::White))
}

fn select_border_for_component<'a>(component: UiComponent, app: &mut AppState) -> Set<'a> {
    if app.get_active_component() == component {
        symbols::border::DOUBLE
    } else {
        symbols::border::PLAIN
    }
}

fn popup_area(area: Rect, x_len: u16, y_len: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(y_len)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(x_len)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}