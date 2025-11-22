use crate::app::{AppState, UiComponent};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::crossterm::event::{Event};
use std::error::Error;

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {

            Event::Key(KeyEvent { code: key, .. }) => {
                match app.get_active_component() {
                    // Setup phase
                    UiComponent::GroupSelector => handle_group_selector_input(app, key),
                    UiComponent::QuestionSelector => handle_question_selector_input(app, key),

                    // Practice phase
                    UiComponent::PracticeControls => handle_practice_controls_input(app, key),
                    UiComponent::PracticeMain => handle_practice_main_input(app, key),

                    // Exit
                    UiComponent::ExitPopup => handle_exit_popup_input(app, key),
                }
            }

            // FIXME: Mouse events are not reported.
            // Event::Mouse(MouseEvent { kind, column, row, modifiers }) => {
            //     println!("{:?}, {:?}, {:?}, {:?}", column, row, kind, column);
            //     Ok(())
            // },

            _ => Ok(())
        }
    } else {
        Ok(())
    }
}

fn handle_group_selector_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.previous_group(),
        KeyCode::Down | KeyCode::Char('j') => app.next_group(),
        KeyCode::Enter | KeyCode::Char(' ') => app.toggle_group_active_status_and_load_questions(),
        KeyCode::Char('p') => app.navigate_to_practice_main(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.toggle_setup_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_question_selector_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.previous_question(),
        KeyCode::Down | KeyCode::Char('j') => app.next_question(),
        KeyCode::Char('p') => app.navigate_to_practice_main(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.toggle_setup_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_practice_controls_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        // TODO: Handle events
        KeyCode::Up | KeyCode::Char('k') => app.select_previous_practice_control_menu_item(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next_practice_control_menu_item(),
        KeyCode::Enter => app.activate_selected_practice_control(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.toggle_practice_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_practice_main_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        // TODO: Handle events
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.toggle_practice_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}


fn handle_exit_popup_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Enter => app.exit_app(),
        KeyCode::Esc => app.close_exit_popup(),
        _ => Ok(()),
    }
}

