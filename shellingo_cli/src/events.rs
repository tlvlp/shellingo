use std::error::Error;
use ratatui::crossterm::event::{Event, ModifierKeyCode};
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event;
use crate::app::{AppState, ComponentFocus, Popup};

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let code = key_event.code;
            match code {
                // Switch between components
                KeyCode::Tab => app.switch_component_focus(),
                _ => {
                    match app.focused_component {
                        ComponentFocus::Menu => handle_menu_input(app, code),
                        ComponentFocus::Body => handle_body_input(app, code),
                        ComponentFocus::Popup => handle_popup_input(app, code),
                    }
                }
            }
        } else { Ok(()) }
    } else { Ok(()) }
}

fn handle_menu_input(app: &mut AppState, code: KeyCode ) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Left => app.select_prev_menu(),
        KeyCode::Right => app.select_next_menu(),
        KeyCode::Enter => app.navigate_to_selected_menu(),
        KeyCode::Esc => app.open_popup(Popup::ExitConfirmation),
        _ => { Ok(()) }
    }
}

fn handle_body_input(app: &mut AppState, code: KeyCode ) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Esc => app.open_popup(Popup::ExitConfirmation),
        _ => { Ok(()) }
    }
}

fn handle_popup_input(app: &mut AppState, code: KeyCode ) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Esc => app.close_active_popup(), // TODO: Popup close confirmation instead of exit
        _ => { Ok(()) }
    }
}