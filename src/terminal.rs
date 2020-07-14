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
        let terminal = Self {
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

    pub fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide)
    }

    pub fn move_cursor_to(&self, pos: &Position) {
        print!(
            "{}",
            termion::cursor::Goto(1 + pos.col() as u16, 1 + pos.row() as u16)
        )
    }

    pub fn show_cursor(&self) {
        print!("{}", termion::cursor::Show)
    }

    pub fn clear_screen(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdout.flush()
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}
