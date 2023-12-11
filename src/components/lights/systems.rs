use std::rc::Rc;

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, utils::text::space_text};

use super::{Lights, TrafficLight};

pub fn traffic_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    let red_light = TrafficLight::default().new(Lights::Red).construct();
    let yellow_light = TrafficLight::default().new(Lights::Yellow).construct();
    let green_light = TrafficLight::default().new(Lights::Green).construct();

    frame.render_widget(red_light, layout[0]);
    frame.render_widget(yellow_light, layout[1]);
    frame.render_widget(green_light, layout[2]);
}

pub fn light_system(frame: &mut Frame<'_>, layout: Rc<[Rect]>, app: &mut App) {
    frame.render_widget(
        Paragraph::default().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lights ")
                .title_alignment(Alignment::Center),
        ),
        layout[0],
    );
}
