use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders,
    },
};

use crate::utils::text::space_text;

pub mod layout;
pub mod queries;
pub mod systems;

/// these are the lights, custom for blue maybe..
pub enum Lights {
    Red,
    Yellow,
    Green,
    Custom(String),
}

/// each light is a traffic light obviously

#[derive(Debug)]
pub struct TrafficLight {
    pub name: String,
    pub on: Color,
    pub off: Color,
    pub status: bool,
}

impl Default for TrafficLight {
    /// just here to use [self] in implementation
    fn default() -> Self {
        Self {
            name: format!(""),
            on: Color::Reset,
            off: Color::Reset,
            status: false,
        }
    }
}

impl TrafficLight {
    pub fn new(self, light: Lights) -> Self {
        match light {
            Lights::Red => Self {
                name: format!("Red"),
                on: Color::Rgb(255, 0, 0),
                off: Color::Rgb(51, 0, 0),
                status: self.status,
            },
            Lights::Yellow => Self {
                name: format!("Yellow"),
                on: Color::Rgb(255, 255, 0),
                off: Color::Rgb(51, 51, 0),
                status: self.status,
            },
            Lights::Green => Self {
                name: format!("Green"),
                on: Color::Rgb(0, 255, 0),
                off: Color::Rgb(0, 51, 0),
                status: self.status,
            },
            Lights::Custom(name) => Self {
                name: format!("{}", name),
                on: self.on,
                off: self.off,
                status: self.status,
            },
        }
    }

    pub fn construct(self) -> Canvas<'static, impl Fn(&mut Context<'_>)> {
        Canvas::default()
            .background_color(self.on)
            .block(
                Block::default()
                    .title(space_text(self.name.as_str()))
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(self.on)),
            )
            .paint(|ctx| {})
    }
}
