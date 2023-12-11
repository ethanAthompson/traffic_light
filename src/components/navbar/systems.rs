//
use std::{collections::HashSet, rc::Rc};

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::{app::App, components::morsecode::system::morse_code_text, utils::text::space_text};

/// Every app atleast needs a navbar right?
pub fn navbar_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Navbar ")
                .title_alignment(Alignment::Center),
        ),
        layout[2],
    );
}

pub fn navbar_tab_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let widget = Tabs::new(app.tabs.clone())
        .block(
            Block::default()
                .title(space_text("Morse Code"))
                .style(Style::default().fg(Color::White))
                .title_style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::DarkGray))
        .select(app.selected);

    frame.render_widget(widget, layout[0]);
}

/// The page system supposed to be generic if i what that means
pub fn navbar_page_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let page_on = match app.selected {
        0 => format!(" Page: Home "),
        _ => format!(" Page: {} ", app.selected),
    };

    let pages = match app.selected {
        _ => Block::default()
            .title(page_on)
            .borders(Borders::TOP)
            .title_alignment(ratatui::prelude::Alignment::Right)
            .style(Style::default().fg(Color::White))
            .border_set(symbols::border::ROUNDED),
    };

    let codes = match app.selected {
        _ => morse_code_text(app),
    };

    frame.render_widget(pages, layout[1]);
    frame.render_widget(codes, layout[1]);
}
