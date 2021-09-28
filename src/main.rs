mod editor;
use editor::Editor;

mod modes;
mod document;
mod terminal;
mod status_message;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
