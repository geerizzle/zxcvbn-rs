use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use super::Match;
use super::patterns::Patterns;

pub(crate) struct Token {
    dict: HashMap<String, u128>,
    min_len: usize,
}

// TODO: Refactor with lifetimes later
impl Token {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub fn build_from(&mut self, file: &File) -> std::io::Result<()> {
        let buffer: BufReader<&File> = BufReader::new(file);
        for (rank, line) in buffer.lines().enumerate() {
            let line = line?;
            self.min_len = min(line.len(), self.min_len);
            self.dict.entry(line).or_insert((rank + 1) as u128);
        }
        Ok(())
    }

    fn find_substrs(&mut self, password: String) -> Option<Vec<String>> {
        let mut substrs: Vec<String> = Vec::new();
        let max_len = password.len();
        let mut sub_len = self.min_len;
        while sub_len <= max_len {
            for pos in 0..(max_len - sub_len + 1) {
                let substr: String = password[pos..pos+sub_len].to_string();
                if self.dict.get(substr.as_str()).is_some() {
                    substrs.push(substr);
                }
            }
            sub_len += 1;
        }

        Some(substrs)
    }
}

impl Default for Token {
    fn default() -> Self {
        Self { 
            dict: Default::default(),
            min_len: usize::MAX
        }
    }
}

impl Match for Token {
    fn get_matches(&mut self, password: String) -> Patterns {
        let password = password.to_lowercase();
        let substrs = self.find_substrs(password).unwrap();
        Patterns::Token(substrs)
    }
}