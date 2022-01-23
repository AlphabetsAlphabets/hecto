use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color as ColorT, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use super::modes::Mode;
use crossterm::event::{Event, KeyCode as Key, KeyModifiers as Mod};

pub enum Command {
    Instruction(Mode, (Key, Mod)),
    None,
}

#[derive(PartialEq)]
pub enum State {
    Fine,
    Success,
    InvalidCommand,
}

pub struct App {
    pub input: String,
    pub commands: Vec<String>,
    pub state: State,
    command: String,
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
            command: String::new(),
        }
    }
}
fn state_returns<'a>(commands: Vec<ListItem<'a>>, app: &'a App) -> (List<'a>, Paragraph<'a>) {
    let command_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " commands ".to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let mut commands = List::new(commands.clone())
        .style(Style::default())
        .block(command_block.clone());

    let input_block = Block::default()
                .borders(Borders::ALL)
                .title("Type the command")
                .title_alignment(Alignment::Center);

    let mut input = Paragraph::new(app.input.as_ref())
            .style(Style::default())
            .block(input_block.clone());

    if app.state == State::Success {
        commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::Green)));

        input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(ColorT::Green))
            .block(input_block.title("Success"));

    } else if app.state == State::Fine {
        commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::White)));

        input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(ColorT::White))
            .block(input_block.title("Type the command"));

    } else {
        commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::Red)));

        input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(ColorT::Red))
            .block(input_block.title("Invalid command"));
    }

    (commands, input)
}

pub fn command_window<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // FIXME: The reason the window isn't resized properly is because the horizontal margin is set.
    // It needs to be dynamic instead of static.
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

    // TODO: The styles, and such change depending on the state of the app.
    // Find a way to make it cleaner, because this is way too ugly to look at
    // plus a lot of code is replicated.
    let windows = state_returns(commands, &app);

    f.render_widget(windows.0, chunks[1]);

    // Clear the area from text to make space for input box.
    let block = Block::default().style(Style::default().fg(ColorT::White));
    f.render_widget(block, chunks[2]);
    f.render_widget(windows.1, chunks[2]);

    if app.state == State::InvalidCommand {
        let dyn_chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(60)
            .vertical_margin(19)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(30),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(size);

        let block = Block::default()
            .style(Style::default().fg(ColorT::White))
            .borders(Borders::ALL);
        let chunk = 1;
        f.render_widget(block, dyn_chunks[chunk]);

        let cmd = format!("'{}'", app.command);

        let mut text = vec![
            Spans::from(vec![Span::styled(
                "Invalid Command",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(ColorT::Red),
            )]),
            Spans::from(vec![
                Span::from("The command "),
                Span::styled(
                    cmd.clone(),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(ColorT::Blue),
                ),
                Span::from(" is not valid."),
            ]),
        ];

        if app.state == State::Success {
            text = vec![
                Spans::from(vec![Span::styled(
                    "Success",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(ColorT::Red),
                )]),
                Spans::from(vec![
                    Span::from("The command "),
                    Span::styled(
                        cmd,
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(ColorT::Blue),
                    ),
                    Span::from(" has been processed."),
                ]),
            ];
        }

        let msg = Paragraph::new(text).alignment(Alignment::Center);

        f.render_widget(msg, dyn_chunks[chunk]);
    }

    f.set_cursor(chunks[2].x + app.input.len() as u16 + 1, chunks[2].y + 1);
}

pub fn run_command_mode<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    key: Event,
) -> Command {
    terminal.hide_cursor().unwrap();
    terminal.draw(|f| command_window(f, app)).unwrap();

    if let Event::Key(event) = key {
        match event.code {
            Key::Char(c) => {
                app.state = State::Fine;
                app.input.push(c);
                Command::None
            }
            Key::Backspace => {
                app.input.pop();
                Command::None
            }
            Key::Esc => {
                app.input.clear();
                terminal.show_cursor().unwrap();
                Command::None
            }
            Key::Enter => {
                let command = app.input.to_uppercase();
                let mut iter = app.commands.iter();
                app.command = app.input.clone();

                app.input.clear();
                if let Some(_) = iter.find(|&e| e == &command) {
                    app.state = State::Fine;
                    Command::Instruction(Mode::Normal, (Key::Char('w'), Mod::ALT))
                } else {
                    app.state = State::InvalidCommand;
                    Command::None
                }
            }
            _ => Command::None,
        }
    } else {
        Command::None
    }
}
