use std::rc::Rc;

use ratatui::prelude::{Constraint, Direction, Layout, Rect};

/// Has its own seperate layout which becomes a child of whatever parent
pub fn inputbox_layout(layout: Rc<[Rect]>) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(0)
        .vertical_margin(1)
        .split(layout[1])
}
