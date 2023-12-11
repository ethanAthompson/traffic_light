use ratatui::{
    prelude::{Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::{
    app::App,
    components::{
        footer::systems::*, header::systems::*, inputbox::systems::*, lights::systems::*,
        morsecode::system::*, navbar::systems::*,
    },
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .horizontal_margin(2)
        .split(frame.size());

    let status_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .margin(0)
        .split(div[1]);

    let header_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Constraint::Length(40),
            Constraint::Percentage(100),
        ])
        .vertical_margin(1)
        .split(div[0]);

    let inner_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .margin(0)
        .split(header_div[0]);

    let lights_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(32),
            Constraint::Percentage(32),
            Constraint::Percentage(32),
            Constraint::Percentage(6),
        ])
        .margin(1)
        .split(inner_div[0]);

    let tab_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(inner_div[2]);

    let inner_tab_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(40)])
        .margin(2)
        .split(inner_div[2]);

    let copyright_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .vertical_margin(2)
        .split(status_div[2]);

    let command_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .horizontal_margin(0)
        .vertical_margin(1)
        .split(status_div[1]);

    let pins_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(status_div[0]);

    header_system(frame, header_div.clone(), app);
    footer_system(frame, div.clone(), app);
    connected_pins_system(frame, status_div.clone(), app);
    connected_pins_status_system(frame, pins_div.clone(), app);
    commands_system(frame, status_div.clone(), app);
    command_text_system(frame, command_div.clone(), app);
    copyright_system(frame, status_div.clone(), app);
    copyright_text_system(frame, copyright_div.clone(), app);
    light_system(frame, inner_div.clone(), app);
    traffic_system(frame, lights_div.clone(), app);
    play_light_system(frame, lights_div.clone(), app);
    inputbox_system(frame, inner_div.clone(), app);
    navbar_system(frame, inner_div.clone(), app);
    navbar_tab_system(frame, tab_div.clone(), app);
    navbar_page_system(frame, inner_tab_div.clone(), app);
}
