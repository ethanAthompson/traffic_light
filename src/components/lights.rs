use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders,
    },
};
use rust_gpiozero::*;
use std::{thread, time::Duration};

use crate::utils::text::space_text;

use super::morsecode::ConnectedPins;

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
    pub fn swap_r(&mut self) {
        self.on = self.off;
    }

    pub fn swap_l(&mut self) {
        self.off = self.on;
    }

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

    pub fn duration(light: Lights) {
        match light {
            Lights::Red => thread::sleep(Duration::from_secs(1000)),
            Lights::Yellow => thread::sleep(Duration::from_secs(3000)),
            Lights::Green => thread::sleep(Duration::from_millis(5000)),
            _ => unimplemented!(),
        }
    }

    // self.off = self.on
    pub fn construct(self) -> Canvas<'static, impl Fn(&mut Context<'_>)> {
        Canvas::default()
            .background_color(self.off)
            .block(
                Block::default()
                    .title(space_text(self.name.as_str()))
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(self.off)),
            )
            .paint(|ctx| {})
    }
}

impl Hardware for ConnectedPins {}

pub trait Hardware {
    fn red_light() -> LED {
        LED::new(ConnectedPins::GPIO17.value())
    }
    fn yellow_light() -> LED {
        LED::new(ConnectedPins::GPIO27.value())
    }
    fn green_light() -> LED {
        LED::new(ConnectedPins::GPIO22.value())
    }
    fn buzzer() -> Buzzer {
        Buzzer::new(ConnectedPins::GPIO23.value())
    }
}

pub fn play_lights() {
    thread::spawn(|| {
        let mut red_light = TrafficLight::default().new(Lights::Red);
        let mut yellow_light = TrafficLight::default().new(Lights::Yellow);
        let mut green_light = TrafficLight::default().new(Lights::Green);

        // Red Light
        ConnectedPins::red_light().on();
        TrafficLight::swap_r(&mut red_light);
        TrafficLight::duration(Lights::Red);
        TrafficLight::swap_l(&mut red_light);
        ConnectedPins::red_light().off();

        // Yellow Light
        ConnectedPins::yellow_light().on();
        TrafficLight::swap_r(&mut yellow_light);
        TrafficLight::duration(Lights::Yellow);
        TrafficLight::swap_l(&mut yellow_light);
        ConnectedPins::yellow_light().off();

        // Green Light
        ConnectedPins::green_light().on();
        TrafficLight::swap_r(&mut green_light);
        TrafficLight::duration(Lights::Green);
        TrafficLight::swap_l(&mut green_light);
        ConnectedPins::green_light().off();
    });
}
