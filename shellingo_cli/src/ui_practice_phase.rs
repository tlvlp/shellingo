use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui_widgets::block::Block;
use ratatui_widgets::paragraph::Paragraph;
use crate::app::AppState;

pub(crate) fn render_title_with_tooltips(frame: &mut Frame, title_block: Block,  draw_area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "[Tab] switch panes, [↑↓] navigate"
        ).block(title_block),

        draw_area
    );
}

pub(crate) fn render_practice_controls(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    // TODO
}

pub(crate) fn render_practice_main(app: &mut AppState, frame: &mut Frame, draw_area: Rect) {
    // TODO
}