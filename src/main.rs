mod editor;
use editor::Editor;

mod document;
mod terminal;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
