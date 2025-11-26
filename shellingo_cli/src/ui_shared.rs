use ratatui::style::Style;
use ratatui_widgets::borders::BorderType;
use ratatui_widgets::scrollbar::{Scrollbar, ScrollbarOrientation};
use crate::app::{AppState, UiComponent};

pub(crate) fn get_style_for_component(component: UiComponent, app: &mut AppState) -> (BorderType, Style) {
    if app.get_active_component() == component {
        (BorderType::Plain, Style::default())
    } else {
        (BorderType::Plain, Style::default().dim())
    }
}

pub(crate) fn get_new_scrollbar<'a>() -> Scrollbar<'a>  {
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None)
}