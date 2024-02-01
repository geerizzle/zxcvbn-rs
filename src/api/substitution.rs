use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) struct Substitution {
    map: HashMap<String, Vec<String>>,
}

impl Substitution {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn build_from(&mut self, file: &File) -> () {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            let line = line.expect("Not a TSV");
            let (key, values) = line.split_once("\t").expect("Not a TSV");
            let values: Vec<String> = values.split("\t").map(|x| x.to_string()).collect();
            map.entry(key.to_string()).or_insert(values);
        }

        self.map = map;
    }
}

mod tests {
    use super::Substitution;
    use std::{collections::HashMap, fs::File};

    #[test]
    fn test_build_from() -> std::io::Result<()> {
        let mut subs = Substitution::new();
        let dict = File::open("tests/leet_test.tsv")?;
        subs.build_from(&dict);
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        map.insert("a".to_string(), vec!["@".to_string(), "4".to_string()]);
        map.insert("b".to_string(), vec!["8".to_string()]);
        map.insert("e".to_string(), vec!["3".to_string()]);
        assert_eq!(map, subs.map);
        Ok(())
    }
}
