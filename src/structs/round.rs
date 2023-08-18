
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Round.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGRound {
    
    #[serde(rename(serialize = "bestOf",    deserialize = "bestOf"))]
    pub best_of:    Option<i64>,
    pub id:         Option<i64>,
    pub number:     Option<i64>,

    #[serde(rename(serialize = "startAt",   deserialize = "startAt"))]
    pub start_at:   Option<i64>,

}

impl GGRound {

    /// Returns the number of games needed to win a majority of in the round.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn best_of(&self) -> i64 {
        let mut result: i64 = 0;
        if self.best_of.is_some() {
            result = self.best_of.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the round.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the number of the round.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn number(&self) -> i64 {
        let mut result: i64 = 0;
        if self.number.is_some() {
            result = self.number.unwrap().clone();
        }
        return result;
    }

    /// Returns the start time of the round.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn start_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.start_at.is_some() {
            result = self.start_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

}
