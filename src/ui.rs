use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Length(10),
        ])
        .split(frame.size());

    let paragraph = Paragraph::new(format!(
        "Welcome to mobtime-cli .\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press enter to start the timer.\n\
                Timer: {}",
        app.timer
    ))
    .block(
        Block::default()
            .title("Mobtime-cli")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);

    frame.render_widget(paragraph, chunks[0]);
}
