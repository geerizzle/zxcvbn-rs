pub(crate) mod dictionary;
pub(crate) mod estimate;
pub(crate) mod matching;
pub mod substitution;

use self::{
    dictionary::Dictionary,
    matching::{patterns::Patterns, sequencer::Sequencer, token::Token},
    substitution::Substitution,
};
use std::{fs::File, io::BufReader};

pub struct Zxcvbn {
    pwn_time: Option<u32>,
    dictionary: Dictionary,
    substitution: Option<Substitution>,
}

impl<'a> Zxcvbn {
    pub fn new() -> Self {
        Self {
            pwn_time: None,
            dictionary: Default::default(),
            substitution: None,
        }
    }

    pub fn set_dictionary(&mut self, file: &File) -> () {
        self.dictionary = Dictionary::new();
        let _ = self.dictionary.build_from(file);
    }

    pub fn set_substitution(&mut self, buffer: &File) -> () {
        let mut subs = Substitution::new();
        subs.build_from(buffer);
    }

    pub fn get_patterns(&mut self, password: &str) -> Option<Vec<Patterns>> {
        let patterns: Vec<Patterns> = Vec::new();
        let token = Token::new(&self.dictionary);
        let sequencer = Sequencer::new();
        Some(patterns)
    }
}
