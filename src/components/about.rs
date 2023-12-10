pub mod layout;
pub mod queries;
pub mod systems;

// Delete later
// // Pins: ({}, {}, {}) Connected

// let (msg, style) = match app.input_mode {
//     InputMode::Normal => (
//         vec![" Press".into(), " e".into(), " to start editing.".into()],
//         Style::default().add_modifier(Modifier::RAPID_BLINK),
//     ),
//     InputMode::Editing => (
//         vec![
//             "Press".into(),
//             " Esc".into(),
//             " to stop editing, ".into(),
//             "Enter".into(),
//             " to save message to history".into(),
//         ],
//         Style::default(),
//     ),
// };

// let mut help_text = Text::from(Line::from(msg));
// help_text.patch_style(style);
// let help_msg = Paragraph::new(help_text).alignment(Alignment::Right);

// // Morse Code Generator
// let input_box = Paragraph::new(text(app.input.as_str()))
//     .block(
//         Block::default()
//             .title(text("Morse Code Generator"))
//             .title_alignment(Alignment::Left)
//             .borders(Borders::ALL)
//             .border_type(BorderType::Rounded),
//     )
//     .style(match app.input_mode {
//         InputMode::Normal => Style::default(),
//         InputMode::Editing => Style::default().fg(Color::Cyan),
//     })
//     .alignment(Alignment::Left);

// // Morse Code
// let message_tabs = Tabs::new(app.tabs.clone())
//     .block(
//         Block::default()
//             .title(text("Morse Code"))
//             .borders(Borders::ALL)
//             .border_set(symbols::border::ROUNDED),
//     )
//     .style(Style::default().fg(Color::White))
//     .highlight_style(Style::default().fg(Color::Cyan))
//     .select(app.selected);

// // let rnd: Vec<u64> = (0..100).map(|v| v + 100).collect();
// let rnd: Vec<u64> = vec![2, 4, 2, 4, 2, 4, 4, 4, 2];

// // Pages
// let message_pages = match app.selected {
//     0 => LineGauge::default()
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .title("Progress")
//                 .border_type(BorderType::Rounded),
//         )
//         .gauge_style(
//             Style::default()
//                 .fg(Color::White)
//                 .bg(Color::Black)
//                 .add_modifier(Modifier::BOLD),
//         )
//         .line_set(symbols::line::THICK)
//         .ratio(0.4),
//     // 0 => Sparkline::default()
//     //     .block(
//     //         Block::default()
//     //             .title("Sparkline")
//     //             .borders((Borders::ALL))
//     //             .border_set(symbols::border::ROUNDED),
//     //     )
//     //     .data(&rnd)
//     //     .style(Style::default().fg(Color::Black)),

//     // 1 => Block::default()
//     //     .title("Inner 2")
//     //     .borders(Borders::ALL)
//     //     .style(Style::default().fg(Color::Cyan))
//     //     .border_type(BorderType::Rounded),
//     // 2 => Block::default()
//     //     .title("Inner 3")
//     //     .borders(Borders::ALL)
//     //     .borders(Borders::ALL)
//     //     .style(Style::default().fg(Color::Yellow))
//     //     .border_type(BorderType::Rounded),
//     // 3 => Block::default()
//     //     .title("Inner 4")
//     //     .borders(Borders::ALL)
//     //     .borders(Borders::ALL)
//     //     .style(Style::default().fg(Color::Cyan))
//     //     .border_type(BorderType::Rounded),
//     // 4 => Block::default()
//     //     .title("Inner 5")
//     //     .borders(Borders::ALL)
//     //     .borders(Borders::ALL)
//     //     .style(Style::default().fg(Color::Blue))
//     //     .border_type(BorderType::Rounded),
//     _ => unreachable!(),
// };

// // Code
// let message_result = Paragraph::new(format!("\n \n{}\n \n", app.code))
//     .alignment(Alignment::Center)
//     .block(
//         Block::default()
//             .title("Code")
//             .title_style(Style::default().add_modifier(Modifier::BOLD))
//             .borders(Borders::ALL)
//             .style(Style::default().fg(Color::LightCyan))
//             .title_alignment(Alignment::Center)
//             .border_type(BorderType::Rounded),
//     );
// // pages should be in message_result

// // Morse Code History
// frame.render_widget(message_tabs, tab_layout[0]);
// frame.render_widget(message_pages, inner_tab_layout[1]);
// frame.render_widget(message_result, inner_tab_layout[2]);

// Morse Code Input Box
// frame.render_widget(input_box, inner_sibling_layout[1]);
// frame.render_widget(help_msg, inner_sibling_layout[1]);
