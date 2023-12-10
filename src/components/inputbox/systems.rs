use std::rc::Rc;

use ratatui::{
    layout::Alignment,
    prelude::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        canvas::*, Bar, BarChart, BarGroup, Block, BorderType, Borders, LineGauge, Padding,
        Paragraph, RenderDirection, Scrollbar, ScrollbarOrientation, ScrollbarState, Sparkline,
        Tabs,
    },
    Frame,
};

use crate::{app::App, utils::text::space_text};

use super::{InputColor, InputMode};

pub fn inputbox_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let text = inputbox_text(app);

    frame.render_widget(text, layout[1]);
}

pub fn inputbox_text(app: &mut App) -> Paragraph<'_> {
    Paragraph::new(space_text(app.input.as_str()))
        .block(
            Block::default()
                .title(space_text("Morse Code Generator"))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match app.input_mode {
            InputMode::Normal => InputMode::normal(),
            InputMode::Editing => InputMode::edit(),
        })
        .alignment(Alignment::Left)
}

// Messages append a line in a page in the tab, like how helix does with files
// When you press space, add a line \n
