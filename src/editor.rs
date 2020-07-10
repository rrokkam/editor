use crate::buffer::Buffer;
use crate::terminal::Terminal;
use termion::event::Key;

#[derive(Clone, Copy, Default)]
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

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.terminal.clear_screen();
        loop {
            self.terminal.render(&self.buffer)?;
            let key = self.terminal.key().unwrap();
            match key {
                Key::Char(c) if !c.is_control() => println!("{}\r", c),
                Key::Up => self.move_up(),
                Key::Down => self.move_down(),
                Key::Left => self.move_left(),
                Key::Right => self.move_right(),
                Key::Ctrl('s') => self.buffer.save(),
                _ => break,
            }
        }
        self.terminal.clear_screen();
        Ok(())
    }

    fn move_up(&mut self) {
        self.cursor_position = Position {
            row: pos.row.saturating_sub(1),
            col: pos.col,
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let pos = &self.cursor_position;
        self.cursor_position = match key {
            Key::Up => Position {
                row: pos.row.saturating_sub(1),
                col: pos.col,
            },
            Key::Down => Position {
                row: pos.row.saturating_add(1),
                col: pos.col,
            },
            Key::Left => Position {
                row: pos.row,
                col: pos.col.saturating_sub(1),
            },
            Key::Right => Position {
                row: pos.row,
                col: pos.col.saturating_add(1),
            },
            _ => *pos,
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let pos = &self.cursor_position;
        self.cursor_position = match key {
            Key::Up => Position {
                row: pos.row.saturating_sub(1),
                col: pos.col,
            },
            Key::Down => Position {
                row: pos.row.saturating_add(1),
                col: pos.col,
            },
            Key::Left => Position {
                row: pos.row,
                col: pos.col.saturating_sub(1),
            },
            Key::Right => Position {
                row: pos.row,
                col: pos.col.saturating_add(1),
            },
            _ => *pos,
        }
    }

    fn move_right(&mut self) {
        self.cursor_position = Position {
            row: pos.row,
            col: pos.col.saturating_add(1),
            }
        }
    }
}
