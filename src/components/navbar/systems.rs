//
use std::rc::Rc;

use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::{app::App, utils::text::space_text};

/// Every app atleast needs a navbar right?
pub fn navbar_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let fg_text = Color::White;
    let fg_highlight = Color::Cyan;
    let fg_block = Color::Green;

    let navbar_container = Tabs::new(app.tabs.clone())
        .block(
            Block::default()
                .title(space_text("Morse Code"))
                .style(Style::default().fg(fg_text))
                .title_style(Style::default().fg(fg_block))
                .border_style(Style::default().fg(fg_block))
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED),
        )
        .style(Style::default().fg(fg_text))
        .highlight_style(Style::default().fg(fg_highlight))
        .select(app.selected);

    frame.render_widget(navbar_container, layout[0]);
}

/// The page system supposed to be generic if i what that means
pub fn navbar_page_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let fg_page = Color::White;

    // Page cannot start at 0 if it starts at tab 1 ???
    let page_on = format!(" Page: {} ", app.selected + 1);

    let pages = match app.selected {
        _ => Block::default()
            .title(page_on)
            .borders(Borders::ALL)
            .style(Style::default().fg(fg_page))
            .border_set(symbols::border::ROUNDED),
    };

    frame.render_widget(pages, layout[1]);
}
