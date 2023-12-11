use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use rust_gpiozero::*;
use std::{fmt, rc::Rc, thread::sleep, time::Duration};

use crate::app::App;

use super::ConnectedPins;

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

pub fn connected_pins_status_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let style = Style::default().fg(Color::White);
    let header_style = Style::default().fg(Color::LightCyan);

    let pins = [
        ConnectedPins::GPIO17.value(),
        ConnectedPins::GPIO27.value(),
        ConnectedPins::GPIO22.value(),
        ConnectedPins::GPIO23.value(),
    ];

    let t1 = vec![
        Line::from(Span::styled(format!("-- Game --"), header_style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(
            format!("GPIO 17: {}", pins[0]),
            app.check_connection(pins[0]),
        )),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(
            format!("GPIO 27: {}", pins[1]),
            app.check_connection(pins[1]),
        )),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(
            format!("GPIO 22: {}", pins[2]),
            app.check_connection(pins[2]),
        )),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(
            format!("GPIO 23: {}", pins[3]),
            app.check_connection(pins[3]),
        )),
    ];

    frame.render_widget(
        Paragraph::new(t1)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" General Purpose I/O ")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        layout[0],
    );
}
