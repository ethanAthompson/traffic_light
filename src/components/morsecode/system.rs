use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use rust_gpiozero::*;
use std::{fmt, rc::Rc, thread::sleep, time::Duration};

use crate::app::App;

pub fn morse_code_text(app: &mut App) -> Paragraph<'static> {
    let morse_code_text = format!("{}", app.history[app.selected]);

    Paragraph::new(morse_code_text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .add_modifier(Modifier::BOLD)
}

pub fn connected_pins_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Connected Pins ")
                .title_alignment(Alignment::Center),
        ),
        layout[0],
    );
}
