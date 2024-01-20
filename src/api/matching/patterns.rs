use super::token::Token;

pub enum Patterns {
    Token(Vec<String>),
    Reveresed(Vec<String>),
    Sequence(Vec<String>),
    Repeat(Vec<String>),
    Keyboard,
    Date(Vec<String>),
    BruteForce
}