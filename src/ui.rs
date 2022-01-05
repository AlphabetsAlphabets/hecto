// use super::gap_buffer::GapBuffer;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, size: Rect) {
    // let mut size = f.size();
    // size.width /= 2;
    // size.height /= 2;

    let block = Block::default().style(Style::default().fg(Color::White));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(size);

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

    f.render_widget(paragraph, chunks[0]);
}
