use super::keyboard::KeyMatch;
pub enum Patterns {
    Token(Vec<String>),
    Reversed(Vec<String>),
    Sequence(Vec<String>),
    Repeat(Vec<String>),
    Keyboard(Vec<KeyMatch>),
    Date(Vec<String>),
    BruteForce,
}
