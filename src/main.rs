mod editor;
use editor::Editor;

mod document;
mod modes;
mod rows;
mod status_message;
mod terminal;
mod window;

use std::io::{stdout, StdoutLock};

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut editor = Editor::new(stdout);
    editor.run();
}
