mod editor;
use editor::Editor;

mod modes;
use modes::Mode;

mod document;
mod terminal;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
