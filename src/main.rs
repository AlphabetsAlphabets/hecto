mod gap_buffer;

mod ui;

mod document;
use document::Document;

use std::{env, io};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};

fn create_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    terminal
}

fn main() {
    let mut terminal = create_terminal();
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let doc = Document::open(file.as_str());
    ui::run_app(doc.lines, &mut terminal);
}
