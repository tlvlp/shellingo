use crate::app::{AppState, Popup, UiBodyItem, UiComponent};
use ratatui::crossterm::event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::{Event};
use std::error::Error;

pub fn handle_input(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let code = key_event.code;
            match code {
                KeyCode::Tab => app.switch_component_focus(),
                _ => match app.focused_component {
                    UiComponent::Menu => handle_menu_input(app, code),
                    UiComponent::Body => handle_body_input(app, code),
                    UiComponent::Popup => handle_popup_input(app, code),
                },
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn handle_menu_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Left => app.select_prev_menu(),
        KeyCode::Right => app.select_next_menu(),
        KeyCode::Enter => app.navigate_to_selected_menu(),
        KeyCode::Esc => app.open_popup(Popup::ExitConfirmation),
        _ => Ok(()),
    }
}

fn handle_body_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match app.active_screen {
        UiBodyItem::QuestionSelector => {
            match code {
                KeyCode::Up => app.previous_group(),
                KeyCode::Down => app.next_group(),
                KeyCode::Enter => app.select_group(),
                KeyCode::Char(' ') => app.select_group(),
                KeyCode::Esc => app.open_popup(Popup::ExitConfirmation),
                _ => Ok(()),
            }
        }
        UiBodyItem::PracticeScreen => { Ok(())}
        UiBodyItem::ExitPopup => {Ok(())}
    }

}

fn handle_popup_input(app: &mut AppState, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Esc => app.close_active_popup(), // TODO: Popup close confirmation instead of exit
        _ => Ok(()),
    }
}
