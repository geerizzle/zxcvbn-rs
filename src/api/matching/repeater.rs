use fancy_regex::Regex;

use super::{patterns::Patterns, Match};
struct Repeater {
    greedy: Regex,
    lazy: Regex,
}

pub(crate) struct RepeatMatch {
    matched: String,
    rank: u128,
    n: u32,
}

impl RepeatMatch {
    fn new(matched: String, rank: u128, n: u32) -> Self {
        Self { matched, rank, n }
    }

    fn find_base(&self) -> String {
        let length = self.matched.len();
        let half = length / 2;

        for window_size in 1..=half {
            for start in 0..=length - window_size * 2 {
                let window = &self.matched[start..start + window_size];
                let next_window = &self.matched[start + window_size..start + window_size * 2];

                if window == next_window {
                    return window.to_string();
                }
            }
        }

        String::new()
    }
}

impl Repeater {
    fn new() -> Self {
        Default::default()
    }

    fn get_largest(&self, password: &str) -> Option<RepeatMatch> {
        let mut matches = get_sequences(password, &self.lazy);
        matches.append(&mut get_sequences(password, &self.greedy));
        let matched = matches
            .iter()
            .max_by_key(|m| m.len())
            .expect("No lazy best match")
            .to_owned();

        Some(RepeatMatch::new(matched, 42, 0))
    }
}

impl Default for Repeater {
    fn default() -> Self {
        Self {
            greedy: Regex::new(r"(.+)\1+").unwrap(),
            lazy: Regex::new(r"(.+?)\1+").unwrap(),
        }
    }
}

impl Match for Repeater {
    fn get_matches(&mut self, password: String) -> Patterns {
        let repeater = Repeater::new();
        let result = repeater.get_largest(password.as_str()).unwrap();
        Patterns::Repeat(vec![result])
    }
}

fn get_sequences(password: &str, re: &Regex) -> Vec<String> {
    let matches: Vec<String> = re
        .captures_iter(password)
        .filter_map(|cap| cap.unwrap().get(0))
        .map(|m| m.as_str().to_string())
        .collect();

    matches
}

mod tests {
    use super::Repeater;

    #[test]
    fn test_largest() -> std::io::Result<()> {
        let repeater = Repeater::new();
        let test = repeater.get_largest("aabaab").unwrap();
        assert_eq!("aabaab".to_string(), test.matched);
        let test = repeater.get_largest("aaaaa").unwrap();
        assert_eq!("aaaaa".to_string(), test.matched);
        assert_eq!("a".to_string(), test.find_base());
        let test = repeater.get_largest("l0giT3CHl0giT3CH").unwrap();
        assert_eq!("l0giT3CHl0giT3CH".to_string(), test.matched);
        assert_eq!("l0giT3CH".to_string(), test.find_base());
        Ok(())
    }
}
