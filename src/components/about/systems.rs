use std::rc::Rc;

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::utils::text::space_text;

/// The user needs these instructions to leave or start...
pub fn instruction_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>) {
    let fg = Color::Green;
    let bg = Color::Reset;

    let widget = instruction_text()
        .block(
            Block::default()
                .title(space_text("Information"))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(fg).bg(bg))
        .alignment(Alignment::Center);

    frame.render_widget(widget, layout[1]);
}

/// Decided to break it up into a format like this; more easier
pub fn instruction_text() -> Paragraph<'static> {
    let key_color = Style::default().fg(Color::LightMagenta);
    let text_content = Style::default().fg(Color::White);

    Paragraph::new(vec![
        Line::from(vec![Span::styled("Welcome to my app!", text_content)]),
        Line::from(vec![Span::raw("\n")]),
        Line::from(vec![
            Span::styled("Press ", text_content),
            Span::styled("Ctrl-Z ", key_color),
            Span::styled("or ", text_content),
            Span::styled("Ctrl-C ", key_color),
            Span::styled("to exit ", text_content),
        ]),
        Line::from(vec![Span::raw(space_text("\n"))]),
        Line::from(vec![
            Span::styled("Press ", text_content),
            Span::styled("e ", key_color),
            Span::styled("to edit, ", text_content),
            Span::styled("ESC ", key_color),
            Span::styled("to stop editing ", text_content),
        ]),
        Line::from(vec![Span::raw(space_text("\n"))]),
        Line::from(vec![
            Span::styled("Press ", text_content),
            Span::styled("Enter ", key_color),
            Span::styled("to submit ", text_content),
        ]),
    ])
}
