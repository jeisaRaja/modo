use std::fs;

use crate::Error;

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self { lines: vec![] }
    }
}

impl Buffer {
    pub fn load(&mut self, filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;

        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(String::from(value));
        }

        Ok(Buffer { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() == 0
    }
}
