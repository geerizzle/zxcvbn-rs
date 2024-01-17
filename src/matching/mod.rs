pub(crate) mod patterns;
pub(crate) mod token;

use patterns::Patterns;

pub(crate) trait Match {
    fn get_matches() -> Patterns;
} 
