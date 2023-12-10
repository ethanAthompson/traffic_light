use std::rc::Rc;

use ratatui::{
    prelude::{Alignment, Rect},
    symbols,
    widgets::{Block, Borders},
    Frame,
};

use crate::utils::text::space_text;

/// every app needs a title right?
pub fn header_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>) {
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .border_set(symbols::border::ROUNDED)
            .title(space_text("Traffic Light Application"))
            .title_alignment(Alignment::Center),
        layout[0],
    );
}
