use fancy_regex::Regex;
struct Repeater {
    greedy: Regex,
    lazy: Regex,
}

impl Repeater {
    fn new() -> Self {
        Default::default()
    }

    fn get_largest(&self, password: &str) -> String {
        let mut matches = get_sequences(password, &self.lazy);
        matches.append(&mut get_sequences(password, &self.greedy));
        matches
            .iter()
            .max_by_key(|m| m.len())
            .expect("No lazy best match")
            .to_owned()
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
        let test = repeater.get_largest("aabaab");
        assert_eq!("aabaab".to_string(), test);
        let test = repeater.get_largest("aaaaa");
        assert_eq!("aaaaa".to_string(), test);
        let test = repeater.get_largest("l0giT3CHl0giT3CH");
        assert_eq!("l0giT3CHl0giT3CH".to_string(), test);
        Ok(())
    }
}
