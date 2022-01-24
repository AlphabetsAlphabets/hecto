use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color as ColorT, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

pub struct StatefulList {
    state: ListState,
    items: Vec<String>,
}

impl Default for StatefulList {
    fn default() -> Self {
        let mut items = vec![];
        items.push("SAVE".to_string());
        items.push("QUIT".to_string());
        Self {
            state: ListState::default(),
            items,
        }
    }
}

impl StatefulList {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App {
    pub input: String,
    pub commands: StatefulList,
    pub state: State,
    current_command: String,
}

impl Default for App {
    fn default() -> Self {
        let commands = StatefulList::default();
        Self {
            input: "".to_string(),
            commands,
            state: State::Fine,
            current_command: String::new(),
        }
    }
}

fn state_returns<'a>(
    commands: Vec<ListItem<'a>>,
    app: &'a App,
) -> (List<'a>, Paragraph<'a>, Option<Vec<Spans<'a>>>) {
    let command_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " commands ".to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let commands = List::new(commands)
        .style(Style::default())
        .block(command_block.clone().title_alignment(Alignment::Center))
        .highlight_style(
            Style::default()
                .bg(ColorT::Rgb(252, 170, 7))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Type the command")
        .title_alignment(Alignment::Center);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(input_block.clone());

    let cmd = format!("'{}'", app.current_command);
    let text = vec![
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
        let commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::Green)));

        let input = input
            .style(Style::default().fg(ColorT::Green))
            .block(input_block.title("Success"));

        (commands, input, None)
    } else if app.state == State::Fine {
        let commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::White)));

        let input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(ColorT::White))
            .block(input_block.title("Type the command"));

        (commands, input, None)
    } else {
        let commands = commands
            .style(Style::default())
            .block(command_block.border_style(Style::default().fg(ColorT::Red)));

        let input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(ColorT::Red))
            .block(input_block.title("Invalid command"));

        (commands, input, Some(text))
    }
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

    let items: Vec<ListItem> = app
        .commands
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.to_owned())];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let windows = state_returns(items, &app);
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

        let msg = Paragraph::new(windows.2.unwrap()).alignment(Alignment::Center);
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
            Key::Char('j') => {
                if event.modifiers.contains(Mod::CONTROL) {
                    app.commands.next();
                };

                Command::None
            }
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
                let mut iter = app.commands.items.iter();
                app.current_command = app.input.clone();

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
