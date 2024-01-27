pub(crate) mod dictionary;
pub(crate) mod matching;

use self::{
    dictionary::Dictionary,
    matching::{patterns::Patterns, sequencer::Sequencer, token::Token},
};
use std::fs::File;

pub struct Zxcvbn {
    pwn_time: Option<u32>,
    dictionary: Dictionary,
}

impl<'a> Zxcvbn {
    pub fn new() -> Self {
        Self {
            pwn_time: None,
            dictionary: Default::default(),
        }
    }

    // TODO: need to handle the substitutions to be full token search
    // TODO: inserting custom substitution tables in TSV format
    pub fn set_dictionary(&mut self, file: &File) -> () {
        self.dictionary = Dictionary::new();
        let _ = self.dictionary.build_from(file);
    }

    pub fn get_patterns(&mut self, password: &str) -> Option<Vec<Patterns>> {
        let patterns: Vec<Patterns> = Vec::new();
        let token = Token::new(&self.dictionary);
        let sequencer = Sequencer::new();
        Some(patterns)
    }
}
