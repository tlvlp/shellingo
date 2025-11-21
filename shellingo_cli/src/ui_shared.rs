use ratatui_widgets::borders::BorderType;
use ratatui_widgets::scrollbar::{Scrollbar, ScrollbarOrientation};
use crate::app::{AppState, UiComponent};

pub(crate) fn select_border_for_component(component: UiComponent, app: &mut AppState) -> BorderType {
    if app.get_active_component() == component {
        BorderType::Thick
    } else {
        BorderType::Plain
    }
}

pub(crate) fn get_new_scrollbar<'a>() -> Scrollbar<'a>  {
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None)
}