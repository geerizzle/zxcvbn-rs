use std::{collections::HashMap, fs::File};

struct Keyboard {
    map: HashMap<char, Vec<Option<char>>>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn build_from(&self, file: &File) -> std::io::Result<()> {
        todo!()
    }
}
