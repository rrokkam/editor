mod buffer;
mod editor;
mod terminal;

use buffer::Buffer;
use editor::Editor;

pub fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args()
        .nth(1)
        .expect("Need to be given a filename for a buffer to open");

    let buffer = Buffer::new(filename)?;

    Editor::new(buffer)?.run()
}
