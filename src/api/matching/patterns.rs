use super::keyboard::KeyMatch;
use crate::api::matching::date::DateMatch;
use crate::api::matching::repeater::RepeatMatch;
use crate::api::matching::sequencer::SequenceMatch;
use crate::api::matching::token::TokenMatch;
pub enum Patterns {
    Token(Vec<TokenMatch>),
    Reversed(Vec<String>),
    Sequence(Vec<SequenceMatch>),
    Repeat(Vec<RepeatMatch>),
    Keyboard(Vec<KeyMatch>),
    Date(Vec<DateMatch>),
    BruteForce,
}
