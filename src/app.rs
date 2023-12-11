use crate::components::{
    inputbox::InputMode,
    morsecode::{ConnectedPins, MorseCode, MorseCodeUnits},
};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::widgets::ScrollbarState;
use std::{collections::HashSet, error};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Selected {
    Yes,
    No,
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// User reacts to a key
    pub reacted: u8,
    /// detects pressed reaction
    pub pressed: bool,
    pub is_selected: Selected,
    /// Is pi connected?
    pub is_connected: bool,
    /// Pins needed to connect
    pub pins: ConnectedPins,
    /// Current value of input box
    pub input: String,
    /// Position of cursor
    pub cursor_position: usize,
    /// Current mode of input
    pub input_mode: InputMode,
    /// History is saved to toml file, then is loaded back constantly
    pub history: Vec<String>,
    /// Status of translating input to morse code
    pub translating: bool,
    /// The state for popup on key [p]
    pub is_helped: bool,
    /// Hidden word for user to know
    pub hidden_word: String,
    /// The tab selected
    pub selected: usize,
    /// Vector of Tabs
    pub tabs: Vec<String>,
    /// Tracker of added tabs
    pub tab_added: u8,
    // pub msg: String,
    pub code: String,
    /// Scroll bar horizontal state
    pub vertical_scroll_state: ScrollbarState,
    /// Horizontal scroll #amount
    pub vertical_scroll: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Narrow this down, simplify the code
            running: true,
            reacted: 0,
            is_selected: Selected::Yes,
            pressed: false,
            is_connected: false,
            is_helped: false,
            pins: ConnectedPins::None,
            translating: false,
            input: String::new(),
            cursor_position: 0,
            input_mode: InputMode::Normal,
            history: vec!["\n\n.... --- -- .".into()],
            selected: 0,
            tabs: vec!["Home".into()],
            code: String::new(),
            hidden_word: String::new(),
            tab_added: 0,
            vertical_scroll_state: ScrollbarState::new(0),
            vertical_scroll: 0,
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

    pub fn toggle_help(&mut self) {
        self.is_helped = !self.is_helped;
    }

    // Morse Code Feature
    pub fn generate(&mut self) {
        let input = self.input.clone();
        let morse_code = MorseCode::new(input.as_str());
        self.code = morse_code.join("");

        let tip = format!("\n\n{}", morse_code.join(""));
        self.history.push(tip);
    }

    // Tab Box Struct
    pub fn move_tab_left(&mut self) {
        if self.selected == 0 {
            self.selected = self.tabs.len();
        }

        let moved_left = self.selected.saturating_sub(1);
        self.selected = self.clamp_selected(moved_left)
    }

    pub fn move_tab_right(&mut self) {
        if self.selected == self.tabs.len() - 1 {
            self.selected = 0;
        } else {
            let moved_right = self.selected.saturating_add(1);
            self.selected = self.clamp_selected(moved_right)
        }
    }

    pub fn remove_tab(&mut self, tab: usize) {
        // self.tabs.remove(tab);
        self.tabs = self.tabs.clone();
    }

    /// super simple tab addition
    pub fn add_tab(&mut self, tab: usize) {
        let tab_title = format!("Tab {}", self.tab_added + 1);

        // tabs were being added behind Home tab
        self.tabs.insert(tab + 1, tab_title);
    }

    /// I needed to upload tabs to a toml file and load in extra tabs there as soon as you enter
    pub fn tab_save() {}

    pub fn clamp_selected(&self, new_selected: usize) -> usize {
        new_selected.clamp(0, self.tabs.len())
    }

    // Input Box Struct
    pub fn clamp_cursor(&self, new_cursor_position: usize) -> usize {
        new_cursor_position.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn locked_message(&mut self) {
        self.history.push(self.input.clone());

        // idx bugs out if I clear the input
        // self.input.clear();

        // In this case I want the user to not edit the message,
        // when translation is in process
        self.input_mode = InputMode::Normal;
        // self.reset_cursor();

        // self.tabs.push(self.input.to_lowercase());
        self.generate();
    }

    pub fn submit_message(&mut self) {
        // self.tabs.push(self.input.to_lowercase());

        self.generate();
        self.tab_added += 1;

        // idx bugs out if I clear the input
        self.input.clear();

        // clean up
        self.reset_cursor();
    }

    pub fn move_cursor_left(&mut self) {
        let moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(moved_right);
    }

    pub fn enter_char(&mut self, new_character: char) {
        self.input.insert(self.cursor_position, new_character);

        // As you enter text [start]->>>>>>...[end]
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;

        if is_not_cursor_leftmost {
            let current_idx = self.cursor_position;
            let from_left_to_current_idx = current_idx - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_idx);
            let after_char_to_delete = self.input.chars().skip(current_idx);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
}
