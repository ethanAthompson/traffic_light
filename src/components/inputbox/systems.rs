use std::rc::Rc;

use ratatui::{
    layout::Alignment,
    prelude::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols::{self, scrollbar},
    text::{Line, Span, Text},
    widgets::{
        canvas::*, Bar, BarChart, BarGroup, Block, BorderType, Borders, LineGauge, Padding,
        Paragraph, RenderDirection, Scrollbar, ScrollbarOrientation, ScrollbarState, Sparkline,
        Tabs, Wrap,
    },
    Frame,
};

use crate::{app::App, utils::text::space_text};

use super::{layout::inputbox_layout, InputColor, InputMode};

pub fn inputbox_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(app.input.len());

    frame.render_widget(
        Paragraph::new(format!("-> {}", app.input.as_str()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Morsecode Generator ")
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .scroll((app.vertical_scroll as u16, 0))
            .style(match app.input_mode {
                InputMode::Normal => InputMode::normal(),
                InputMode::Editing => InputMode::edit(),
            }),
        layout[1],
    );
}

pub fn inputbox_scroller_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let widget = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .symbols(scrollbar::VERTICAL)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .track_symbol(None);

    frame.render_stateful_widget(
        widget,
        layout[1].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.vertical_scroll_state,
    );
}
