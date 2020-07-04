mod buffer;
mod editor;
mod terminal;
use editor::Editor;

fn main() -> Result<(), std::io::Error> {
    Editor::new()?.run()
}
