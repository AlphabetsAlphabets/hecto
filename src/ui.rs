use super::gap_buffer::GapBuffer;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub fn run_app<B: Backend>(lines: Vec<GapBuffer>, terminal: &mut Terminal<B>) {
    enable_raw_mode().unwrap();

    terminal.draw(|f| ui(lines, f)).unwrap();

    disable_raw_mode().unwrap();
}

fn ui<B: Backend>(lines: Vec<GapBuffer>, f: &mut Frame<B>) {
    let size = f.size();

    // This block is used to clear the screen.
    // This is thrown away immediately after use.
    let block = Block::default().style(Style::default().fg(Color::White));
    f.render_widget(block, size);

    // The actual window where the text will be.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        // .horizontal_margin(30)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let mut win_1 = vec![];

    for line in lines {
        win_1.push(Spans::from(line.line()));
    }

    let create_block = |title: String| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let paragraph = Paragraph::new(win_1)
        .style(Style::default().fg(Color::White))
        .block(create_block("Main Window".to_string()))
        .alignment(Alignment::Left);

    f.render_widget(paragraph, chunks[0]);
}
