mod editor;
use editor::Editor;

mod document;
mod modes;
mod rows;
mod status_message;
mod terminal;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
