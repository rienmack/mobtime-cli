use std::error;
use tui_input::Input;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// timer
    pub timer: u8,
    pub user_input: Input,
    pub goal_input: String,
    pub input_mode: InputMode,
    pub users: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            timer: 0,
            user_input: Input::default(),
            goal_input: String::from(""),
            input_mode: InputMode::Normal,
            users: vec![],
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

    pub fn start_timer(&mut self) {
        //todo implement timer
    }

    pub fn end_timer(&mut self) {
        //todo implement timer
    }
}
