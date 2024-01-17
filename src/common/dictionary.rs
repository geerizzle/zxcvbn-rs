use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, cmp::min};

pub struct Dictionary {
    map: HashMap<String, u128>,
    pub(crate) min_len: usize
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            min_len: usize::MAX
        }
    }

    pub fn build_from(&mut self, file: &File) -> std::io::Result<()> {
        let buffer: BufReader<&File> = BufReader::new(file);
        for (rank, line) in buffer.lines().enumerate() {
            let line = line.expect("No more lines to be read");
            self.min_len = min(line.len(), self.min_len);
            self.map.entry(line).or_insert((rank + 1) as u128);
        }

        Ok(())
    }

    pub(crate) fn get_rank(&self, word: &str) -> Option<&u128> {
        self.map.get(word)
    }
}

mod tests {
    use std::{fs::File, collections::HashMap};
    use super::Dictionary;

    
    #[test]
    fn test_build() -> std::io::Result<()> {
        let mut dict = Dictionary::new();
        let file = File::open("SecLists/Passwords/Common-Credentials/best15.txt")?;
        let _ = dict.build_from(&file)?;
        let best_15 = HashMap::from(
            [("111111".to_string(), 1),
            ("1234".to_string(), 2),
            ("12345".to_string(), 3),
            ("123456".to_string(), 4),
            ("1234567".to_string(), 5),
            ("12345678".to_string(), 6),
            ("abc123".to_string(), 7),
            ("dragon".to_string(), 8),
            ("iloveyou".to_string(), 9),
            ("letmein".to_string(), 10),
            ("monkey".to_string(), 11),
            ("password".to_string(), 12),
            ("qwerty".to_string(), 13),
            ("tequiero".to_string(), 14),
            ("test".to_string(), 15)]
        );
        assert_eq!(best_15, dict.map);
        Ok(())
    }
}