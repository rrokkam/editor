use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Buffer {
    lines: Vec<String>,
    name: String,
}

impl Buffer {
    pub fn new(name: String) -> Result<Self, std::io::Error> {
        Ok(Buffer {
            lines: Self::open_file(&name)?,
            name,
        })
    }

    /// open returns Err if the file contains invalid UTF-8.
    fn open_file(name: &str) -> Result<Vec<String>, std::io::Error> {
        Ok(match File::open(&name) {
            Ok(file) => BufReader::new(file).lines().collect::<Result<_, _>>()?,
            Err(_) => Vec::new(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn row(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
