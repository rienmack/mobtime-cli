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
    /// users
    pub user_input: Input,
    pub users: Vec<String>,
    /// goals
    pub goal_input: String,
    /// input mode
    pub input_mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            timer: 0,
            user_input: Input::default(),
            users: vec![],
            goal_input: String::from(""),
            input_mode: InputMode::Normal,
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
