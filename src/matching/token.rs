use crate::common::dictionary::Dictionary;
use super::Match;

pub(crate) struct Token {
    dict: Dictionary
}

impl Token {
    fn new(dict: Dictionary) -> Self {
        Self {
            dict
        }
    }
}

impl Match for Token {
    fn get_matches() -> super::patterns::Patterns {
        todo!()
    }
}