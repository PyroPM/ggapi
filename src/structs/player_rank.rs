
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg PlayerRank.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGPlayerRank {

    pub id:         Option<i64>,
    pub rank:       Option<i64>,
    pub title:      Option<String>,

}

impl GGPlayerRank {

    /// Returns the id of the player rank.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the rank of the player rank.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn rank(&self) -> i64 {
        let mut result: i64 = 0;
        if self.rank.is_some() {
            result = self.rank.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the title of the player rank.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn title(&self) -> String {
        let mut result: String = "".to_string();
        if self.title.is_some() {
            result = self.title.clone().unwrap().clone();
        }
        return result;
    }

}
