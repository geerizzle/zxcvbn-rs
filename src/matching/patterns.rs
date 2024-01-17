use super::token::Token;

pub(crate) enum Patterns {
    Token(Token),
    Reveresed,
    Sequence,
    Repeat,
    Keyboard,
    Date,
    BruteForce
}