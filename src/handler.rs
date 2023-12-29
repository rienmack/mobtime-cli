use std::io;

use crate::app::{App, AppResult, InputMode};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            _ => {}
        },
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => {
                app.users.push(app.user_input.value().into());
                app.user_input.reset();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {
                app.user_input.handle_event(&Event::Key(key_event));
            }
        },
    }
    Ok(())
}
