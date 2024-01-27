use crate::api::dictionary::Dictionary;

use super::Match;
use super::patterns::Patterns;

pub(crate) struct Token<'a> {
    dict: &'a Dictionary
}

impl<'a> Token<'a> {
    pub(crate) fn new(dict: &'a Dictionary) -> Self {
        Self {
            dict
        }
    }

    fn find_substrs(&mut self, password: String) -> Option<Vec<String>> {
        let mut substrs: Vec<String> = Vec::new();
        let max_len = password.len();
        let mut sub_len = self.dict.min_len;
        while sub_len <= max_len {
            for pos in 0..(max_len - sub_len + 1) {
                let substr: String = password[pos..pos+sub_len].to_string();
                if self.dict.get_rank(&substr).is_some() {
                    substrs.push(substr);
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