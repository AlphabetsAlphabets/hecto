use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color as ColorT, Modifier, Style},
    terminal::Frame,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crossterm::event::{Event, KeyCode as Key};

#[derive(PartialEq)]
enum State {
    Fine,
    InvalidCommand,
}

pub struct App {
    pub input: String,
    pub commands: Vec<String>,
    state: State,
}

impl Default for App {
    fn default() -> Self {
        let mut commands = vec![];
        commands.push("SAVE".to_string());
        commands.push("QUIT".to_string());
        Self {
            input: "".to_string(),
            commands,
            state: State::Fine,
        }
    }
}

pub fn command_window<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(50)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(7),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(size);

    // Clear the area from text to make space for list of commands.
    let block = Block::default().style(Style::default().fg(ColorT::White));
    f.render_widget(block, chunks[1]);

    let create_block = |title: String| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let mut commands = vec![];
    for command in &app.commands {
        commands.push(ListItem::new(command.clone()));
    }

    let commands = List::new(commands)
        .style(Style::default())
        .block(create_block("COMMANDS".to_string()));

    f.render_widget(commands, chunks[1]);

    // Clear the area from text to make space for input box.
    let block = Block::default().style(Style::default().fg(ColorT::White));
    f.render_widget(block, chunks[2]);

    let input = if app.state == State::Fine {
        Paragraph::new(app.input.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Input"))
    } else {
        Paragraph::new(app.input.as_ref())
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ColorT::Red))
                    .title("Invalid Command"),
            )
    };

    f.render_widget(input, chunks[2]);
    f.set_cursor(chunks[2].x + app.input.len() as u16 + 1, chunks[2].y + 1);
}

pub fn run_command_mode<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, key: Event) {
    terminal.hide_cursor().unwrap();
    terminal.draw(|f| command_window(f, app)).unwrap();

    if let Event::Key(event) = key {
        match event.code {
            Key::Char(c) => {
                app.input.push(c);
            }
            Key::Backspace => {
                app.input.pop();
            }
            Key::Esc => {
                app.input.clear();
                terminal.show_cursor().unwrap();
            }
            Key::Enter => {
                let command = app.input.to_uppercase();
                let mut iter = app.commands.iter();
                if let Some(_) = iter.find(|&e| e == &command) {
                    app.state = State::Fine;
                } else {
                    app.state = State::InvalidCommand;
                }

                app.input.clear();
            }
            _ => (),
        }
    }
}
