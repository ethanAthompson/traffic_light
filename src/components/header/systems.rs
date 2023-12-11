use std::rc::Rc;

use anyhow::Result;
use tui_big_text::BigTextBuilder;

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Header for the application
pub fn header_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let version = env!("CARGO_PKG_VERSION");

    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Traffic Light Game v{} ", version))
                .title_alignment(Alignment::Center),
        ),
        layout[0],
    );
}
