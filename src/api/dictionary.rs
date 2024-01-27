use std::{cmp::min, collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub struct Dictionary {
    pub dict: HashMap<String, u128>,
    pub min_len: usize,
}

impl Dictionary {
    pub fn new() -> Self { 
        Default::default()
    }

    pub fn build_from(&mut self, file: &File) -> std::io::Result<()> {
        let buffer: BufReader<&File> = BufReader::new(file);
        for (rank, line) in buffer.lines().enumerate() {
            let line = line?;
            self.min_len = min(line.len(), self.min_len);
            self.dict.entry(line).or_insert((rank + 1) as u128);
        }
        Ok(())
    }

    pub fn get_rank(&self, password: &String) -> Option<&u128> {
        self.dict.get(password)
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            dict: HashMap::new(),
            min_len: usize::MAX
        }
    }
}