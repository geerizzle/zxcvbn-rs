use crate::api::dictionary::Dictionary;

use super::patterns::Patterns;
use super::Match;

pub(crate) struct Token<'a> {
    dict: &'a Dictionary,
}

pub(crate) struct TokenMatch {
    matched: String,
    rank: u128,
}

impl TokenMatch {
    fn new(matched: String, rank: u128) -> Self {
        Self { matched, rank }
    }
}

impl<'a> Token<'a> {
    pub(crate) fn new(dict: &'a Dictionary) -> Self {
        Self { dict }
    }

    fn find_substrs(&mut self, password: String) -> Option<Vec<TokenMatch>> {
        let mut substrs: Vec<TokenMatch> = Vec::new();
        let max_len = password.len();
        let mut sub_len = self.dict.min_len;
        while sub_len <= max_len {
            for pos in 0..(max_len - sub_len + 1) {
                let substr: String = password[pos..pos + sub_len].to_string();
                if let Some(rank) = self.dict.get_rank(&substr) {
                    let matched = TokenMatch::new(substr, *rank);
                    substrs.push(matched);
                }
            }
            sub_len += 1;
        }

        Some(substrs)
    }
}

impl<'a> Match for Token<'a> {
    fn get_matches(&mut self, password: String) -> Patterns {
        let password = password.to_lowercase();
        let substrs = self.find_substrs(password).unwrap();
        Patterns::Token(substrs)
    }
}
