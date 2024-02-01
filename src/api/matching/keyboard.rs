use crate::consts::keys::{KEYS_MAP, SHIFT_KEYS};
use std::{collections::HashMap, fs::File};

struct Keyboard {
    map: HashMap<String, Vec<Option<String>>>,
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

    // TODO: refactor with str literals instead of strings
    pub(crate) fn get_path(&self, password: String) -> () {
        let mut map = &self.map;
        let mut turns: u8 = 0;
        let mut chain: u8 = 0;
        if self.map.is_empty() {
            map = &KEYS_MAP;
        }

        for ch in password.chars() {
            let ch = ch.to_string();
        }
    }
}
