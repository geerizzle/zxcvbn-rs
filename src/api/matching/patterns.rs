pub enum Patterns {
    Token(Vec<String>),
    Reversed(Vec<String>),
    Sequence(Vec<String>),
    Repeat(Vec<String>),
    Keyboard,
    Date(Vec<String>),
    BruteForce,
}
