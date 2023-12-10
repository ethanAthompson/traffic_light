use std::rc::Rc;

use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::{app::App, utils::text::space_text};
