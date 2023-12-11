use crossterm::Result;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use std::rc::Rc;

use crate::{app::App, components::inputbox::InputMode};

/// Footer for the application
pub fn footer_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
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
