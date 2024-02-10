use crate::consts::keys::{KEYS_MAP, SHIFT_KEYS};
use std::{collections::HashMap, fs::File};

use super::{patterns::Patterns, Match};

struct Keyboard {
    map: HashMap<String, Vec<Option<String>>>,
}

pub(crate) struct KeyMatch {
    sequence: String,
    chain: u8,
    turns: u8,
    shifted: u8,
}

impl KeyMatch {
    fn new(sequence: String, chain: u8, turns: u8, shifted: u8) -> Self {
        Self {
            sequence,
            chain,
            turns,
            shifted,
        }
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn build_from(&self, file: &File) -> std::io::Result<()> {
        todo!()
    }

    fn get_path(&mut self, password: &str) -> Vec<KeyMatch> {
        let mut keys = password.chars().peekable();
        let mut buffer = String::new();
        let mut last_dir: Option<usize> = None;
        let mut turns: u8 = 0;
        let mut shifted: u8 = 0;
        let mut matches: Vec<KeyMatch> = Vec::new();
        while let Some(key) = keys.next() {
            let mut tmp: [u8; 4] = [0; 4];
            buffer += key.encode_utf8(&mut tmp);
            if keys.peek().is_none() {
                if buffer.len() != 1 {
                    let matched = KeyMatch::new(buffer.clone(), buffer.len() as u8, turns, shifted);
                    matches.push(matched);
                }
                break;
            }
            let next_key = keys.peek().unwrap();
            let (key_pos, was_shift) = self.get_key_with_shift(key, &next_key);
            if key_pos.is_none() {
                if buffer.len() != 1 {
                    let matched = KeyMatch::new(buffer.clone(), buffer.len() as u8, turns, shifted);
                    matches.push(matched);
                }
                turns = 0;
                buffer = String::new()
            } else {
                let key_pos = key_pos.unwrap();
                shifted += was_shift;
                if last_dir == None {
                    last_dir = Some(key_pos % 2);
                    continue;
                }
                if Some(key_pos % 2) != last_dir {
                    turns += 1;
                    last_dir = Some(key_pos % 2);
                }
            }
        }

        matches
    }

    fn get_key_with_shift(&self, key: char, next_key: &char) -> (Option<usize>, u8) {
        let mut map = &self.map;
        let mut shifted: u8 = 0;
        if map.is_empty() {
            map = &KEYS_MAP;
        }
        let mut key_neighs = map.get(&(key.to_string()));
        if key_neighs.is_none() {
            let key_as_string = key.to_string();
            if let Some(shift_key) = SHIFT_KEYS.get(key_as_string.as_str()) {
                key_neighs = map.get(&(shift_key.to_string()));
            }
        }
        let mut key_pos = key_neighs
            .unwrap()
            .iter()
            .position(|x| *x == Some(next_key.to_string()));

        if key_pos.is_none() {
            let next_as_string = next_key.to_string();
            if let Some(shift_next) = SHIFT_KEYS.get(next_as_string.as_str()) {
                key_pos = key_neighs
                    .unwrap()
                    .iter()
                    .position(|x| *x == Some(shift_next.to_string()));
                if key_pos.is_some() {
                    shifted += 1;
                }
            }
        }

        (key_pos, shifted)
    }
}

impl Match for Keyboard {
    fn get_matches(&mut self, password: String) -> super::patterns::Patterns {
        let matches = self.get_path(password.as_str());
        Patterns::Keyboard(matches)
    }
}

mod tests {
    use super::Keyboard;

    #[test]
    fn test_keyboard() -> std::io::Result<()> {
        let mut keyboard = Keyboard::new();
        let test = keyboard.get_path("zaq1@34%^");

        Ok(())
    }
}
