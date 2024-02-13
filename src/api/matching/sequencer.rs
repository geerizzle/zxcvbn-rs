use std::f32::consts::E;

use super::{patterns::Patterns, Match};
pub(crate) struct Sequencer {}

pub(crate) struct SequenceMatch {
    matched: String,
    cp_delta: i32,
}

impl SequenceMatch {
    fn new(matched: String, cp_delta: i32) -> Self {
        Self { matched, cp_delta }
    }
}

impl Sequencer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn get_sequence(&self, password: &str, dist: &i32) -> Option<Vec<SequenceMatch>> {
        let mut seq: Vec<SequenceMatch> = Vec::new();
        let mut chars = password.chars().peekable();
        if chars.peek().is_none() {
            return None;
        }
        let mut buffer = String::from(chars.next().unwrap());
        for ch in chars {
            if i32::abs(ch as i32 - buffer.chars().last().unwrap() as i32) != *dist {
                if buffer.len() != 1 {
                    let matched = SequenceMatch::new(buffer, *dist);
                    seq.push(matched);
                }
                buffer = String::from(ch);
            } else {
                buffer.push(ch);
            }
        }
        let matched = SequenceMatch::new(buffer, *dist);
        seq.push(matched);
        Some(seq)
    }
}

impl Match for Sequencer {
    fn get_matches(&mut self, password: String) -> Patterns {
        let mut seq: Vec<SequenceMatch> = Vec::new();
        let sequencer = Sequencer::new();
        // TODO: Remove duplicates
        [1, 2, 3, 4].iter().for_each(|dist| {
            seq.append(
                sequencer
                    .get_sequence(password.as_str(), dist)
                    .as_mut()
                    .unwrap(),
            )
        });

        Patterns::Sequence(seq)
    }
}
mod tests {
    use crate::api::matching::sequencer::{SequenceMatch, Sequencer};
    #[test]
    fn test_get_sequence() -> std::io::Result<()> {
        let seq = Sequencer::new();
        let test = seq.get_sequence("abcd", &1).unwrap();
        assert_eq!("abcd".to_string(), test.iter().nth(0).unwrap().matched);
        assert_eq!(1, test.iter().nth(0).unwrap().cp_delta);
        Ok(())
    }
}
