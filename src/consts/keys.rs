use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYS_MAP: HashMap<&'static str, Vec<Option<&'static str>>> = {
        let mut map: HashMap<&str, Vec<Option<&str>>> = HashMap::new();
        map.insert("q", vec![Some("w"), Some("a"), None, Some("1")]);
        map
    };
}
