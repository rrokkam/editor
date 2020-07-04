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
        let stdout = io::stdout().into_raw_mode()?;
        Terminal::clear_screen()?;
        let terminal = Self {
            width,
            height,
            stdout,
        };
        Ok(terminal)
    }

    pub fn print_buffer(&mut self, buffer: &Buffer) -> Result<(), std::io::Error> {
        for i in 0..self.height {
            if let Some(row) = buffer.row(i as usize) {
                println!("{}\r", &row[0..std::cmp::min(row.len(), self.width as usize)]);
            }
        }
        print!("{}\r", termion::cursor::Goto(1, 1));
        self.stdout.flush()
    }

    pub fn refresh(&mut self, position: &Position) -> Result<(), std::io::Error> {
        print!("{}", termion::cursor::Hide);
        print!(
            "{}",
            termion::cursor::Goto(1 + position.col() as u16, 1 + position.row() as u16)
        );
        print!("{}", termion::cursor::Show);
        self.stdout.flush()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }

    pub fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
