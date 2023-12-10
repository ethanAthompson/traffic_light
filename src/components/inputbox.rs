pub mod layout;
pub mod queries;
pub mod systems;

use ratatui::style::{Color, Style};

pub trait InputColor {
    fn normal() -> Style {
        Style::default().fg(Color::White)
    }

    fn edit() -> Style {
        Style::default().fg(Color::Rgb(204, 102, 0))
    }
}

/// Ok, modes for input for user needs
#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Now inputmode has input colors (:
impl InputColor for InputMode {}
