mod editor;
use editor::Editor;

mod terminal;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
