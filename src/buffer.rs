use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        Ok(Buffer {
            lines: BufReader::new(File::open(filename)?)
                .lines()
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn row(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
