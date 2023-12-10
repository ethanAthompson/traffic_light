use std::rc::Rc;

use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders},
    Frame,
};

use crate::utils::text::space_text;

use super::{layout::traffic_light_layout, Lights, TrafficLight};

// easier to read when the system is in its own function
pub fn traffic_light_system(
    frame: &mut Frame<'_>,
    layout: Rc<[Rect]>,
    // To place a container around the rest of the layouts
    layout_container: Rc<[Rect]>,
) {
    // Items
    let red_light = TrafficLight::default().new(Lights::Red).construct();
    let yellow_light = TrafficLight::default().new(Lights::Yellow).construct();
    let green_light = TrafficLight::default().new(Lights::Green).construct();
    let container = traffic_light_container();
    // Layout
    let layout = traffic_light_layout(layout);

    // Spawners
    frame.render_widget(container, layout_container[0]);
    frame.render_widget(red_light, layout[0]);
    frame.render_widget(yellow_light, layout[1]);
    frame.render_widget(green_light, layout[2]);
}

pub fn traffic_light_container() -> Block<'static> {
    let fg_container = Color::White;

    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(fg_container))
        .border_set(symbols::border::ROUNDED)
        .title(space_text("Traffic Lights"))
}
