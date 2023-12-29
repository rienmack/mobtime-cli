use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, InputMode};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(55),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(0),
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(layout[0]);

    user_section(app, frame, left_layout);

    timer_section(app, frame, &layout);

    goal_section(frame, layout);
}

fn user_section(app: &mut App, frame: &mut Frame<'_>, left_layout: std::rc::Rc<[Rect]>) {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, left_layout[1]);

    let width = left_layout[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.user_input.visual_scroll(width as usize);
    let input = Paragraph::new(app.user_input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, left_layout[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                left_layout[1].x
                    + ((app.user_input.visual_cursor()).max(scroll) - scroll) as u16
                    + 1,
                // Move one line down, from the border to the input line
                left_layout[1].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = app
        .users
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Line::from(Span::raw(format!("{}: {}", i + 1, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Users"));
    frame.render_widget(messages, left_layout[2]);
    // frame.render_widget(.widget(), left_layout[1]);
}

fn timer_section(app: &mut App, frame: &mut Frame<'_>, layout: &std::rc::Rc<[Rect]>) {
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
}

fn goal_section(frame: &mut Frame<'_>, layout: std::rc::Rc<[Rect]>) {
    let goals = Block::default()
        .title("Goals")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // let mut textarea = tui_textarea::TextArea::default();
    // textarea.set_block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Double)
    //         .title("Enter name")
    //         .padding(Padding::new(1, 1, 0, 0)),
    // );

    frame.render_widget(goals, layout[2]);
}
