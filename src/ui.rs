use ratatui::{
    layout::Alignment,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    widgets::{canvas::*, Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let branding = format!(
        "\nWelcome to Rust Traffic Light Morse Code System.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                \nCreated by Ethan Thompson.\n\
                "
    );

    let text = |name: &str| format!("\n {} \n", name);

    let light_yellow = Color::LightYellow;
    let yellow = Color::LightYellow;
    let light_red = Color::LightRed;
    let red = Color::LightRed;
    let light_green = Color::LightGreen;
    let green = Color::Green;

    // frame.render_widget(
    //     Paragraph::new(branding)
    //         .block(
    //             Block::default()
    //                 .title("Traffic Light System!")
    //                 .title_alignment(Alignment::Center)
    //                 .borders(Borders::ALL)
    //                 .border_type(BorderType::Rounded),
    //         )
    //         .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //         .alignment(Alignment::Center),
    //     frame.size(),
    // );

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
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(text("Traffic Lights")),
        inner_layout[0],
    );

    // Morse Code
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(text("Morse Code")),
        inner_layout[1],
    );

    let red_light = Canvas::default()
        .background_color(light_red)
        .block(
            Block::default()
                .title(text("Red"))
                .border_style(Style::default().fg(light_red))
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {});

    let yellow_light = Canvas::default()
        .background_color(light_yellow)
        .block(
            Block::default()
                .title(text("Yellow"))
                .border_style(Style::default().fg(light_yellow))
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {});

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
        .paint(|ctx| {});

    let status_bar = Block::default()
        .title(text("Status"))
        .border_style(Style::default().fg(Color::Cyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    frame.render_widget(red_light, lights_layout[0]);
    frame.render_widget(yellow_light, lights_layout[1]);
    frame.render_widget(green_light, lights_layout[2]);
    frame.render_widget(status_bar, inner_child_layout[1]);
}
