use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<(), std::io::Error> {
        let _stdout = io::stdout().into_raw_mode()?;
        self.clear_screen()?;

        for key in io::stdin().keys() {
            let done = self.on_keypress(key?);
            if done {
                self.clear_screen()?;
                break;
            }
        }

        Ok(())
    }

    fn on_keypress(&self, key: Key) -> bool {
        match key {
            Key::Char(c) if c.is_control() => println!("{}\r", c),
            Key::Char(c) => println!("{} ({})\r", c as u8, c),
            Key::Ctrl('q') => return true,
            _ => println!("{:?}\r", key),
        }
        false
    }

    fn clear_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }
}

fn main() {
    Editor::default().run().unwrap();
}
