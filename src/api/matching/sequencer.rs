use std::f32::consts::E;

use super::{patterns::Patterns, Match};
pub(crate) struct Sequencer {}

impl Sequencer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn get_sequence(&self, password: &str, dist: &i32) -> Vec<String> {
        let mut seq: Vec<String> = Vec::new();
        let mut chars = password.chars().peekable();
        if chars.peek().is_none() {
            return seq;
        }
        let mut buffer = String::from(chars.next().unwrap());
        for ch in chars {
            if i32::abs(ch as i32 - buffer.chars().last().unwrap() as i32) != *dist {
                if buffer.len() != 1 {
                    seq.push(buffer);
                }
                buffer = String::from(ch);
            } else {
                buffer.push(ch);
            }
        }
        seq.push(buffer);

        seq
    }
}

impl Match for Sequencer {
    fn get_matches(&mut self, password: String) -> Patterns {
        let mut seq: Vec<String> = Vec::new();
        let sequencer = Sequencer::new();
        // TODO: Remove duplicates
        [1, 2, 3, 4]
            .iter()
            .for_each(|dist| seq.append(sequencer.get_sequence(password.as_str(), dist).as_mut()));

        Patterns::Sequence(seq)
    }
}
mod tests {
    use crate::api::matching::sequencer::Sequencer;
    #[test]
    fn test_get_sequence() -> std::io::Result<()> {
        let seq = Sequencer::new();
        let test = seq.get_sequence("abcd", &1);
        assert_eq!(vec!["abcd".to_string()], test);
        let test = seq.get_sequence("456a123", &1);
        assert_eq!(vec!["456".to_string(), "123".to_string()], test);
        Ok(())
    }
}
