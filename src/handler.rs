use crate::{
    app::{App, AppResult, Selected},
    components::{
        inputbox::InputMode,
        lights::{play_lights, Hardware, TrafficLight},
        morsecode::{ConnectedPins, MorseCode, Sound, SuperBuzzer},
    },
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rppal::gpio::*;
use rust_gpiozero::Buzzer;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on Ctrl-Z`
        KeyCode::Char('z') | KeyCode::Char('Z') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
                // app.reacted = 0;
            }
        }

        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
                // app.reacted = 0;
            }
        }

        KeyCode::Char('f') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                if app.input.is_empty() {
                } else {
                    // play the sound in live person
                    SuperBuzzer::play_sound(
                        &mut ConnectedPins::buzzer(),
                        MorseCode::new(&app.input),
                    );
                }
            }
        }

        KeyCode::Char('p') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                play_lights();
            }
        }

        KeyCode::Char('j') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                // play the sound in live person
                app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
                // app.reacted = 2
            }
        }

        KeyCode::Char('k') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                // play the sound in live person
                app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
                // app.reacted = 2
            }
        }

        // Counter handlers
        KeyCode::Right => {
            app.move_tab_right();
            // app.reacted = 7;
        }

        KeyCode::Left => {
            app.move_tab_left();
            // app.reacted = 7;
        }

        KeyCode::Char('x') => match app.input_mode {
            // Clears only on normal
            InputMode::Normal => {
                if app.selected == 0 {
                } else {
                    app.remove_tab(app.selected);
                }
            }
            InputMode::Editing => {}
        },
        // Other handlers you could add here.
        _ => {}
    }

    match app.is_selected {
        Selected::Yes if key_event.kind == KeyEventKind::Release => match key_event.code {
            _ => {
                // app.reacted = 0;
            }
        },
        _ => {}
    }

    // move to inputbox/queries.rs
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('e') => {
                // if translating { } else {}
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                return Ok(());
            }
            _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => {
                if app.input.len() == 0 {
                } else {
                    app.add_tab(app.tab_added as usize);
                    app.submit_message();
                    // app.reacted = 6;
                    app.tab_save();
                }
            }

            KeyCode::Char('f') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    // play the sound in live person
                }
            }
            KeyCode::Char(to_insert) => {
                app.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.delete_char();
            }
            KeyCode::Left => {
                app.move_cursor_left();
            }
            KeyCode::Right => {
                app.move_cursor_right();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                // app.reacted = 5;
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
