use super::token::Token;

pub enum Patterns {
    Token(Vec<String>),
    Reveresed,
    Sequence,
    Repeat,
    Keyboard,
    Date,
    BruteForce
}