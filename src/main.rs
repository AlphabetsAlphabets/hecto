mod editor;
use editor::Editor;

mod ui;
mod gap_buffer;
mod document;
mod modes;
mod rows;
mod status_message;
mod terminal;
mod window;

use std::io::stdout;

fn main() {
    let stdout = stdout();
    let mut editor = Editor::new(stdout);
    editor.run();
}
