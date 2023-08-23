
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg BracketConfig.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGBracketConfig {

    #[serde(rename(serialize = "bracketType",   deserialize = "bracketType"))]
    pub bracket_type:   Option<i64>,
    pub id:             Option<i64>,

}

impl GGBracketConfig {

    /// Returns the bracket type of the bracket.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn bracket_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.bracket_type.is_some() {
            result = self.bracket_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the bracket.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

}
