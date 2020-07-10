use crate::buffer::Buffer;
use crate::editor::Position;
use std::io::{self, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    width: u16,
    height: u16,
    stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let (width, height) = termion::terminal_size()?;
        let mut terminal = Self {
            width,
            height,
            stdout: io::stdout().into_raw_mode()?,
        };
        terminal.clear_screen();
        Ok(terminal)
    }

    pub fn key(&self) -> Result<Key, std::io::Error> {
        io::stdin().keys().next().unwrap()
    }

    pub fn render(&mut self, buffer: &Buffer, cursor: &Position) -> Result<(), std::io::Error> {
        print!(
            "{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
        );
        for i in 0..self.height {
            if let Some(row) = buffer.row(i as usize) {
                println!(
                    "{}\r",
                    &row[0..std::cmp::min(row.len(), self.width as usize)]
                );
            }
        }
        print!(
            "{}{}",
            termion::cursor::Goto(1 + cursor.col() as u16, 1 + cursor.row() as u16),
            termion::cursor::Show
        );

        self.stdout.flush()
    }

    pub fn clear_screen(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }
}
