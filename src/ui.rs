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
        footer::systems::{commands_system, copyright_system, footer_system},
        header::systems::header_system,
        inputbox::systems::{inputbox_scroller_system, inputbox_system},
        lights::systems::{light_system, traffic_system},
        // lights::systems::traffic_light_system,
        morsecode::system::connected_pins_system,
        navbar::systems::{navbar_page_system, navbar_system, navbar_tab_system},
    },
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .horizontal_margin(2)
        .split(frame.size());

    let status_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .margin(2)
        .split(div[1]);

    let header_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Constraint::Length(40),
            Constraint::Percentage(100),
        ])
        .vertical_margin(2)
        .split(div[0]);

    let inner_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .margin(2)
        .split(header_div[0]);

    let lights_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .margin(2)
        .split(inner_div[0]);

    let tab_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .vertical_margin(2)
        .split(inner_div[2]);

    let inner_tab_div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70),
        ])
        .margin(2)
        .split(inner_div[2]);

    header_system(frame, header_div.clone(), app);
    footer_system(frame, div.clone(), app);
    connected_pins_system(frame, status_div.clone(), app);
    commands_system(frame, status_div.clone(), app);
    copyright_system(frame, status_div.clone(), app);
    light_system(frame, inner_div.clone(), app);
    traffic_system(frame, lights_div.clone(), app);
    inputbox_system(frame, inner_div.clone(), app);
    navbar_system(frame, inner_div.clone(), app);
    navbar_tab_system(frame, tab_div.clone(), app);
    navbar_page_system(frame, inner_tab_div.clone(), app);
}
