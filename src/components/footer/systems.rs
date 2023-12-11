use crate::{app::App, components::inputbox::InputMode};
use crossterm::Result;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};
use std::rc::Rc;

/// Footer for the application
pub fn footer_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::TOP)
                .title(" Status ")
                .title_alignment(Alignment::Center),
        ),
        layout[1],
    );
}

/// Footer for the application
pub fn commands_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Commands ")
                .title_alignment(Alignment::Center),
        ),
        layout[1],
    );
}

pub fn command_text_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let style = Style::default().fg(Color::White);
    let header_style = Style::default().fg(Color::LightCyan);
    let t1 = vec![
        Line::from(Span::styled(format!("-- Game --"), header_style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Ctrl-Z/C to quit app"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Ctrl-P to play game!"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("j/k to move tabs ▲ ▼"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Right/Left to move tabs ◄ ►"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("x to remove a tab [Esc 1st]"), style)),
    ];

    let t2 = vec![
        Line::from(Span::styled(format!("-- Input --"), header_style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("e to enter edit mode"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Esc to exit edit mode"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Enter to submit input"), style)),
        Line::from(Span::styled(format!(""), style)),
        Line::from(Span::styled(format!("Ctrl-F to hear the code"), style)),
    ];

    frame.render_widget(
        Paragraph::new(t1)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(t2)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
        layout[1],
    );
}

pub fn copyright_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Copyright ")
                .title_alignment(Alignment::Center),
        ),
        layout[2],
    );
}

pub fn copyright_text_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let license = env!("CARGO_PKG_LICENSE_FILE");
    let user = env!("CARGO_PKG_AUTHORS");

    let style = Style::default().fg(Color::White);

    let cc = vec![
        Line::from(Span::styled(format!(" By {} ", user), style)),
        Line::from(Span::styled(format!(" {} ", license), style)),
        Line::from(Span::styled(
            format!(" {} ", "Your data is saved at traffic_light_data.toml 🚀🛰️"),
            style,
        )),
    ];

    frame.render_widget(
        Paragraph::new(cc)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
        layout[0],
    );
}
