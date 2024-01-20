pub(crate) mod matching;

use std::fs::File;

use crate::api::matching::patterns::Patterns;

use self::matching::token::Token;

pub struct Zxcvbn {
    pwn_time: Option<u32>,
    token: Option<Token>
}

impl Zxcvbn {
    pub fn new() -> Self {
        Self {
            pwn_time: None,
            token: None,
        }
    }

    pub fn set_dictionary(&mut self, file: &File) -> () {
        let mut token = Token::new();
        let _ = token.build_from(file)
            .expect("Failed to build the dict from provided file");
        self.token = Some(token);
    }

    pub fn set_substitutions(&mut self, file: &File) -> () {
        todo!()
    }

    pub fn get_patterns(&mut self, password: &str) -> Option<Vec<Patterns>> {
        let patterns: Vec<Patterns> = Vec::new();
        // Check all patterns from provided patterns here
        Some(patterns)
    }

}