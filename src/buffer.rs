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
        if position.row() >= self.lines.len() {
            self.lines.resize(position.row() + 1, String::new())
        }

        let mut line = self.lines.get(position.row()).unwrap().clone();
        match line.len().cmp(&position.col()) {
            Ordering::Less => {
                line.push_str(&" ".repeat(position.col() - line.len()));
                line.push(c);
            }
            Ordering::Equal => {
                line.push(c);
            }
            Ordering::Greater => {
                let second_half = line.split_off(position.col());
                line.push(c);
                if !second_half.is_empty() {
                    line.push_str(&second_half[1..]);
                }
            }
        }

        self.lines[position.row()] = line;
    }

    pub fn row(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
