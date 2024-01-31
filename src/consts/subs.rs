use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SUBS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        map.insert("a", vec!["@", "4"]);
        map.insert("b", vec!["8"]);
        map.insert("e", vec!["3"]);
        map.insert("g", vec!["6", "9"]);
        map.insert("h", vec!["#"]);
        map.insert("i", vec!["1"]);
        map.insert("l", vec!["1"]);
        map.insert("t", vec!["7"]);
        map.insert("z", vec!["2"]);
        map
    };
}
