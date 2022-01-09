// use super::gap_buffer::GapBuffer;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Default)]
pub struct App {
    pub input: String,
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

    let block = Block::default().style(Style::default().fg(Color::White));
    f.render_widget(block, chunks[1]);

    let mut commands = vec![];
    commands.push(Spans::from("SAVE"));
    commands.push(Spans::from("QUIT"));

    let create_block = |title: String| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let paragraph = Paragraph::new(commands)
        .style(Style::default())
        .block(create_block("COMMANDS".to_string()))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[1]);

    let x = vec![Spans::from("> ")];
    let paragraph = Paragraph::new(x)
        .style(Style::default())
        .block(create_block("Filter".to_string()));

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[2]);
    // f.render_widget(input, chunks[2]);
    // f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1);
}
