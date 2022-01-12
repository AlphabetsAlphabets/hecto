use tui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color as ColorT, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crossterm::{
    queue,
    cursor::{position, CursorShape, Hide, Show},
    event::{poll, read, Event, KeyCode as Key, KeyEvent, KeyModifiers as Mod},
    style::Color as ColorC,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct App<'a> {
    pub input: String,
    pub commands: Vec<Spans<'a>>
}

impl Default for App<'_> {
    fn default() -> Self {
        let mut commands = vec![];
        commands.push(Spans::from("SAVE"));
        commands.push(Spans::from("QUIT"));
        Self { input: "".to_string(), commands }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    let paragraph = Paragraph::new(app.commands.clone())
        .style(Style::default())
        .block(create_block("COMMANDS".to_string()))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[1]);

    // Clear the area from text to make space for input box.
    let block = Block::default().style(Style::default().fg(ColorT::White));
    f.render_widget(block, chunks[2]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[2]);
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, key: Event)  {
    terminal.draw(|f| ui(f, app)).unwrap();

    if let Event::Key(event) = key {
        match event.code {
            Key::Char(c) => {
                app.input.push(c);
            }
            Key::Backspace => {
                app.input.pop();
            }
            Key::Enter => {
                let command = app.input.to_uppercase();
                app.input.clear();
            }
            _ => (),
        }
    }
}
