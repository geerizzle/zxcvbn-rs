pub(crate) mod patterns;
pub(crate) mod token;

use patterns::Patterns;

pub trait Match {
    fn get_matches(&self, password: String) -> Patterns;
} 