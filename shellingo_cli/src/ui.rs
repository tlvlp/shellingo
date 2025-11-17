use crate::app::{AppState, UiComponent};
use ratatui::prelude::Color;
use ratatui::style::{Style};
use ratatui::symbols::border::Set;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    symbols,
    widgets::{Block, Padding},
};
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
        Paragraph::new("[Tab] Switch between panes, [↑↓] navigate in lists, [Enter/Space] select or edit items")
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
    match app.active_component {
        UiComponent::GroupSelector | UiComponent::QuestionSelector => {
            frame.render_stateful_widget(get_question_group_list(app), body_layout_left, &mut app.question_group_list_state);
            frame.render_stateful_widget(get_question_table(app), body_layout_right, &mut app.question_table_state);
        }
        _ => {
            frame.render_widget(get_no_items_found(), body_layout_left);
        }
    };
}


fn get_no_items_found<'a>() -> Paragraph<'a> {
    Paragraph::new("No items found")
        .block(Block::bordered()
                   .padding(Padding::horizontal(1))
                   .border_set(symbols::border::PLAIN),
        )
}

fn get_question_group_list<'a>(app: &mut AppState) -> List<'a> {
    List::new(
        app.questions_by_groups
            .iter()
            .map(|(group_name, group_details)| {
                let selection_postfix = if group_details.is_active { " *"} else { "" };
                ListItem::new(format!("{}{}",group_name.clone(), selection_postfix))
                    .style(
                        if group_details.is_active { Style::default().bold().fg(Color::Green) }
                        else { Style::default() }
                    )
            })
    )
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_set(select_border_for(UiComponent::QuestionSelector, app)),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::Black).bg(Color::White))
}

fn get_question_table<'a>(app: &mut AppState) -> Table<'a> {
    // todo
    //  - Implement app.load_questions_for_group --> app.questions_by_groups --> QuestionGroupDetails.questions,
    //    that is triggered when selecting a group.
    //  - Load questions from app.load_questions_for_group in a | question | answer | format to the table.
    //  - Leave the questions loaded in load_questions_for_group,
    //    but load them again on deselect/select of the group.
    //  Test data:
    let rows = [
        Row::new(vec!["Cell1", "Cell2"]),
        Row::new(vec!["Cell3", "Cell4"]),
    ];
    let widths = [Constraint::Length(5), Constraint::Length(5)];
    Table::new(rows, widths)
        .block(
            Block::bordered()
                .padding(Padding::horizontal(1))
                .border_set(select_border_for(UiComponent::QuestionSelector, app))
        )
        .highlight_symbol("> ")
        .row_highlight_style(Style::new().fg(Color::Black).bg(Color::White))
}

/// Create a centered Rect using up certain percentage of the available rect
// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     let vertical_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(get_centered_constraints(percent_x))
//         .split(r);
//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(get_centered_constraints(percent_y))
//         .split(vertical_layout[1])[1]
// }

// fn get_centered_constraints(percent: u16) -> [Constraint; 3] {
//     [
//         Constraint::Percentage((100 - percent) / 2),
//         Constraint::Percentage(percent),
//         Constraint::Percentage((100 - percent) / 2),
//     ]
// }

fn select_border_for<'a>(component: UiComponent, app: &AppState) -> Set<'a> {
    if app.active_component == component {
        symbols::border::DOUBLE
    } else {
        symbols::border::PLAIN
    }
}
