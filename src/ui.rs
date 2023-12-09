use crossterm::style::Stylize;
use ratatui::{
    layout::Alignment,
    prelude::{Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        canvas::*, Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding, Paragraph,
        Scrollbar, ScrollbarOrientation, ScrollbarState, Tabs,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let text = |name: &str| format!("\n {} \n", name);
    // Loads from a toml file your history
    let history = vec!["Tab 1", "Tab 2", "Tab 3", "Tab 4"];
    let select_tab = 0;
    let light_yellow = Color::LightYellow;
    let light_red = Color::LightRed;
    let light_green = Color::LightGreen;
    let red = Color::Red;
    let yellow = Color::Yellow;
    let green = Color::Green;

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .horizontal_margin(1)
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .horizontal_margin(0)
        .split(main_layout[1]);

    let inner_sibling_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(87), Constraint::Percentage(13)])
        .horizontal_margin(0)
        .split(inner_layout[1]);

    let inner_child_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(inner_layout[0]);

    let lights_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(20),
        ])
        .split(inner_child_layout[0]);

    // Title Bar
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .border_set(symbols::border::ROUNDED)
            .title(text("Traffic Light Application"))
            .title_alignment(Alignment::Center),
        main_layout[0],
    );

    // Traffic Lights
    frame.render_widget(
        Block::default()
            .borders(Borders::NONE)
            .border_set(symbols::border::ROUNDED)
            .title(text("Traffic Lights")),
        inner_layout[0],
    );

    let red_light = Canvas::default()
        .background_color(red)
        .block(
            Block::default()
                .title(text("Red"))
                .border_style(Style::default().fg(light_red))
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|_ctx| {});

    let yellow_light = Canvas::default()
        .background_color(yellow)
        .block(
            Block::default()
                .title(text("Yellow"))
                .border_style(Style::default().fg(light_yellow))
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|_ctx| {});

    let green_light = Canvas::default()
        .background_color(green)
        .block(
            Block::default()
                .title(text("Green"))
                .border_style(Style::default().fg(light_green))
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|_ctx| {});

    // Pins: ({}, {}, {}) Connected
    let brand = Paragraph::new(format!(
        "\n Welcome to Rust Traffic Light Morse Code System.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                \nCreated by ethanAthompson.\n
                \nPins: (1, 17, 20, 30) Connected\n
                \nPress Enter to start\n
                "
    ))
    .block(
        Block::default()
            .title(text("Traffic Light System!"))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Center);

    // Pins: ({}, {}, {}) Connected
    let input_box = Paragraph::new(text(app.input.value()))
        .block(
            Block::default()
                .title(text("Morse Code Generator"))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Left);

    let tabs = Tabs::new(history)
        .block(
            Block::default()
                .title(text("Morse Code"))
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Cyan))
        .select(select_tab);

    // Traffic Light Widegets
    frame.render_widget(red_light, lights_layout[0]);
    frame.render_widget(yellow_light, lights_layout[1]);
    frame.render_widget(green_light, lights_layout[2]);

    // Branding Widgets
    frame.render_widget(brand, inner_child_layout[1]);

    // Morse Code History
    frame.render_widget(tabs, inner_sibling_layout[0]);
    frame.render_widget(input_box, inner_sibling_layout[1]);
}
