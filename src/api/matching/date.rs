use fancy_regex::Regex;

use super::patterns::Patterns;
use super::Match;

struct Date {
    regex: Regex,
}

impl Date {
    fn new() -> Self {
        Default::default()
    }
}

impl Match for Date {
    fn get_matches(&mut self, password: String) -> Patterns {
        let dates: Vec<String> = self
            .regex
            .captures_iter(password.as_str())
            .filter_map(|cap| cap.unwrap().get(0))
            .map(|m| m.as_str().to_string())
            .collect();

        Patterns::Date(dates)
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            regex: Regex::new(r"(\d{1,2}[-./]?){2}\d{2,4}").unwrap(),
        }
    }
}

mod tests {
    use super::Date;
    use super::Patterns;
    use crate::api::matching::Match;

    #[test]
    fn test_matches() -> std::io::Result<()> {
        let mut date: Date = Date::new();
        let matches = date.get_matches("11-23-45lenovo11/23/45".to_string());
        match matches {
            Patterns::Date(test) => {
                assert_eq!(vec!["11-23-45", "11/23/45"], test)
            }
            _ => (),
        }

        Ok(())
    }
}
