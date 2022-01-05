mod editor;
use editor::Editor;

mod document;
mod gap_buffer;
mod modes;
mod rows;
mod status_message;
mod terminal;
mod ui;
mod window;

use std::io::stdout;

fn main() {
    let stdout = stdout();
    let mut editor = Editor::new(stdout);
    editor.run();
}
