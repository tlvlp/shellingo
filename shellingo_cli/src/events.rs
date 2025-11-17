use crate::app::{AppState, UiComponent};
use ratatui::crossterm::event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::{Event};
use std::error::Error;

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let code = key_event.code;
            match app.active_component {
                UiComponent::GroupSelector => handle_group_selector_input(app, code),
                UiComponent::QuestionSelector => handle_question_selector_input(app, code),
                UiComponent::ExitPopup => handle_exit_popup_input(code),
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn handle_group_selector_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Up => app.previous_group(),
        KeyCode::Down => app.next_group(),
        KeyCode::Enter => app.toggle_group_active_and_load_questions(),
        KeyCode::Char(' ') => app.toggle_group_active_and_load_questions(),
        KeyCode::Tab => Ok(app.active_component = UiComponent::QuestionSelector),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_question_selector_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        // KeyCode::Up => app.previous_question(),
        // KeyCode::Down => app.next_question(),
        // KeyCode::Enter => app.edit_selected_question(),
        KeyCode::Tab => Ok(app.active_component = UiComponent::GroupSelector),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_exit_popup_input(code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        _ => Ok(()),
    }
}
