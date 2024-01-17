use crate::common::dictionary::Dictionary;
use super::Match;
use super::patterns::Patterns;

pub(crate) struct Token {
    dict: Dictionary,
    matches: Vec<(String, u128)>
}

// TODO: Refactor with lifetimes later
impl Token {
    pub(crate) fn new(dict: Dictionary) -> Self {
        Self {
            dict,
            matches: Vec::new()
        }
    }

    fn find_substrs(&self, password: String) -> Vec<String>{
        let mut substrs: Vec<String> = Vec::new();
        let max_len = password.len();
        let mut sub_len = self.dict.min_len;
        while sub_len <= max_len {
            for pos in 0..(max_len - sub_len + 1) {
                let substr: String = password[pos..pos+sub_len].to_string();
                if self.dict.get_rank(substr.as_str()).is_some() {
                    substrs.push(substr);
                }
            }
            sub_len += 1;
        }

        substrs
    }

    fn leet_sub(password: String) -> String {
        password
    }

}

impl Match for Token {
    fn get_matches(&self, password: String) -> Patterns {
        let password = password.to_lowercase();
        let substrs = self.find_substrs(password);
        Patterns::Token(substrs)
    }
}

mod tests {
    use std::{fs::File, vec};
    use crate::common::dictionary::Dictionary;
    use super::Token;

    #[test]
    fn test_find_substrs() -> std::io::Result<()> {
        let mut dict = Dictionary::new();
        let _ = dict.build_from(&File::open("SecLists/Passwords/Common-Credentials/10-million-password-list-top-100.txt")?);
        let token = Token::new(dict);
        let mut substrs = token.find_substrs("password".to_string());
        substrs.sort_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(vec!["pass", "password"], substrs);
        Ok(())
    }
}