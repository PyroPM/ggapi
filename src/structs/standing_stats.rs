
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    score::*,
};

/// Equivalent for start.gg StandingStats.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStandingStats {
    
    pub score:      Option<Box<GGScore>>,

}

impl GGStandingStats {

    /// Returns the score of the standing.
    ///
    /// Returns an empty score if not set or wasn't queried.
    pub fn score(&self) -> GGScore {
        let mut result: GGScore = Default::default();
        if self.score.is_some() {
            result = *self.score.as_ref().unwrap().clone();
        }
        return result;
    }

}
