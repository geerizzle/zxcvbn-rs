pub(crate) mod date;
pub(crate) mod keyboard;
pub(crate) mod patterns;
pub(crate) mod repeater;
pub(crate) mod sequencer;
pub(crate) mod token;

use patterns::Patterns;

pub trait Match {
    fn get_matches(&mut self, password: String) -> Patterns;
}
