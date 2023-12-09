use crossterm::event::{Event, KeyCode, KeyEvent};
use std::error;
use tui_input::Input;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: Input,
    pub input_mode: InputMode,
    // History is saved to toml file, then is loaded back constantly
    pub history: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Normal,
            history: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn adjust_input_modes(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }

                KeyCode::Char('q') => {
                    self.quit();
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    self.history.push(self.input.value().into());
                    self.input.reset();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {
                    // self.input.handle_event(&Event::Key(key));
                }
            },
        }
    }
}
