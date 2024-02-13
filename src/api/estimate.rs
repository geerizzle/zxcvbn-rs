use crate::api::matching::token::TokenMatch;

trait Estimate {
    fn estimate(&self) -> u128;
}

impl Estimate for TokenMatch {
    fn estimate(&self) -> u128 {
        todo!()
    }
}
