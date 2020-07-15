use crate::editor::Position;
use std::cmp::Ordering;
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
        let empty_line = String::new();

        let line = self.lines.get(position.row()).unwrap_or(&empty_line);
        let mut new_line = String::new();
        match line.len().cmp(&position.col()) {
            Ordering::Less => {
                new_line += line;
                //                panic!("got: {} {}", line.len(), position.col());
                new_line += &" ".repeat(position.col() - line.len());
                new_line += &c.to_string();
            }
            Ordering::Equal => {
                new_line += line;
                new_line += &c.to_string();
            }
            Ordering::Greater => {
                new_line += &line[..position.col()];
                new_line += &c.to_string();
                new_line += &line[position.col() + 1..];
            }
        }

        if position.row() >= self.lines.len() {
            self.lines.resize(position.row() + 1, String::new())
        }
        self.lines[position.row()] = new_line;
    }

    pub fn row(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
