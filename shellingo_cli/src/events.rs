use crate::app::{AppState, UiComponent};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::crossterm::event::{Event};
use std::error::Error;
use ratatui::crossterm;
use tui_input::backend::crossterm::EventHandler;

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        let input_event = crossterm::event::read()?;
        match input_event {
            Event::Key(KeyEvent { code: key, .. }) => {
                match app.get_active_component() {
                    // Setup phase
                    UiComponent::GroupSelector => handle_group_selector_input(app, key),
                    UiComponent::QuestionSelector => handle_question_selector_input(app, key),

                    // Practice phase
                    UiComponent::PracticeControls => handle_practice_controls_input(app, key),
                    UiComponent::PracticeMain => handle_practice_main_input(app, input_event),

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
        KeyCode::Up | KeyCode::Char('k') => app.setup_previous_group(),
        KeyCode::Down | KeyCode::Char('j') => app.setup_next_group(),
        KeyCode::Enter | KeyCode::Char(' ') => app.setup_toggle_group_active_status_and_load_questions(),
        KeyCode::Char('p') => app.setup_navigate_to_practice(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.setup_toggle_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_question_selector_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.setup_previous_question(),
        KeyCode::Down | KeyCode::Char('j') => app.setup_next_question(),
        KeyCode::Char('p') => app.setup_navigate_to_practice(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.setup_toggle_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_practice_controls_input(app: &mut AppState, key: KeyCode) -> Result<(), Box<dyn Error>> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.practice_select_previous_menu_item(),
        KeyCode::Down | KeyCode::Char('j') => app.practice_select_next_menu_item(),
        KeyCode::Enter => app.practice_activate_selected_control(),
        KeyCode::Tab | KeyCode::Left | KeyCode::Right => app.practice_toggle_panes(),
        KeyCode::Esc => app.open_exit_popup(),
        _ => Ok(()),
    }
}

fn handle_practice_main_input(app: &mut AppState, event: Event) -> Result<(), Box<dyn Error>> {
    match event.as_key_event()
        .expect("Event expected to be a key event at this point")
        .code {
            KeyCode::Tab => app.practice_toggle_panes(),
            KeyCode::Enter => app.practice_set_next_question_in_round(),
            KeyCode::Esc => app.open_exit_popup(),
            _ => {
                app.answer_input.handle_event(&event);
                Ok(())
            }
    }
}


fn handle_exit_popup_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Enter => app.exit_app(),
        KeyCode::Esc => app.close_exit_popup(),
        _ => Ok(()),
    }
}

