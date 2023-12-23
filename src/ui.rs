use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(frame.size());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(layout[0]);

    let users = Paragraph::new(format!(
        "To add a user press A .\n\
        Type their name, and press A \n\
        again to save. \n\n\n
        Users: {} \n",
        "Ryan"
    ))
    .block(
        Block::default()
            .title("Members of the Mob ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(users, left_layout[0]);

    let mut textarea = tui_textarea::TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title("Enter prompt")
            .padding(Padding::new(1, 1, 0, 0)),
    );

    frame.render_widget(textarea.widget(), left_layout[1]);

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

    frame.render_widget(paragraph, layout[1]);

    let goals = Block::default()
        .title("Goals")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(goals, layout[2]);
}
