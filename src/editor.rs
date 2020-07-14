use crate::buffer::Buffer;
use crate::terminal::Terminal;
use termion::event::Key;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn up(&mut self) {
        self.row = self.row.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.row = self.row.saturating_add(1);
    }

    pub fn left(&mut self) {
        self.col = self.col.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.col = self.col.saturating_add(1);
    }
}

pub struct Editor {
    terminal: Terminal,
    buffer: Buffer,
    cursor_position: Position,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Self {
        Editor {
            terminal: Terminal::new().unwrap(),
            buffer,
            cursor_position: Position::default(),
        }
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        self.terminal.clear_screen();
        loop {
            self.render()?;
            let key = self.terminal.key().unwrap();
            match key {
                Key::Char(c) if !c.is_control() => self.buffer.write(c, &self.cursor_position),
                Key::Up => self.cursor_position.up(),
                Key::Down => self.cursor_position.down(),
                Key::Left => self.cursor_position.left(),
                Key::Right => self.cursor_position.right(),
                _ => break,
            }
        }
        self.terminal.clear_screen();
        Ok(())
    }

    fn render(&mut self) -> Result<(), std::io::Error> {
        self.terminal.hide_cursor();
        self.terminal.move_cursor_to(&Position::default());
        for i in 0..self.terminal.height() {
            if let Some(row) = self.buffer.row(i as usize) {
                println!(
                    "{}\r",
                    &row[0..std::cmp::min(row.len(), self.terminal.width() as usize)]
                );
            }
        }
        self.terminal.move_cursor_to(&self.cursor_position);
        self.terminal.show_cursor();
        self.terminal.flush()
    }
}
