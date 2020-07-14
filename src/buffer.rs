use crate::editor::Position;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(name: String) -> Result<Self, std::io::Error> {
        Ok(Buffer {
            lines: Self::open_file(&name)?,
        })
    }

    /// open returns Err if the file contains invalid UTF-8.
    fn open_file(name: &str) -> Result<Vec<String>, std::io::Error> {
        Ok(match File::open(&name) {
            Ok(file) => BufReader::new(file).lines().collect::<Result<_, _>>()?,
            Err(_) => Vec::new(),
        })
    }

    pub fn write(&mut self, c: char, position: &Position) {
        let line = self.lines.get(position.row()).unwrap();
        let new_line =
            String::new() + &line[..position.col()] + &c.to_string() + &line[position.col() + 1..];
        self.lines[position.row()] = new_line;
    }

    pub fn row(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
