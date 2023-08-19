
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Wave.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGWave {

    pub id:             Option<i64>,
    pub identifier:     Option<String>,

    #[serde(rename(serialize = "startAt",   deserialize = "startAt"))]
    pub start_at:       Option<i64>,

}

impl GGWave {

    /// Returns the id of the wave.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the identifier of the wave.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn identifier(&self) -> String {
        let mut result: String = "".to_string();
        if self.identifier.is_some() {
            result = self.identifier.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the start time of the wave.
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
