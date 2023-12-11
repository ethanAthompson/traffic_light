use std::rc::Rc;

use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub fn footer_layout(layout: Rc<[Rect]>) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(layout[0])
}
