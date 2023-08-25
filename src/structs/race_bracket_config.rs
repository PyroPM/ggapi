
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg RaceBracketConfig.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGRaceBracketConfig {
    
    #[serde(rename(serialize = "automaticEndTime",      deserialize = "automaticEndTime"))]
    pub automatic_end_time:         Option<i64>,

    #[serde(rename(serialize = "automaticStartTime",    deserialize = "automaticStartTime"))]
    pub automatic_start_time:       Option<i64>,

    #[serde(rename(serialize = "bracketType",           deserialize = "bracketType"))]
    pub bracket_type:               Option<i64>,

    #[serde(rename(serialize = "goalTargetComparator",  deserialize = "goalTargetComparator"))]
    pub goal_target_comparator:     Option<i64>,

    #[serde(rename(serialize = "goalTargetValue",       deserialize = "goalTargetValue"))]
    pub goal_target_value:          Option<String>,
    pub id:                         Option<i64>,

    #[serde(rename(serialize = "limitMode",             deserialize = "limitMode"))]
    pub limit_mode:                 Option<i64>,

    #[serde(rename(serialize = "limitValue",            deserialize = "limitValue"))]
    pub limit_value:                Option<i64>,

    #[serde(rename(serialize = "raceType",              deserialize = "raceType"))]
    pub race_type:                  Option<i64>,

}

impl GGRaceBracketConfig {

    /// Returns the time the bracket ends.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn automatic_end_time(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.automatic_end_time.is_some() {
            result = self.automatic_end_time.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the time the bracket starts.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn automatic_start_time(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.automatic_start_time.is_some() {
            result = self.automatic_start_time.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the bracket type of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn bracket_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.bracket_type.is_some() {
            result = self.bracket_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the goal target comparator of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn goal_target_comparator(&self) -> i64 {
        let mut result: i64 = 0;
        if self.goal_target_comparator.is_some() {
            result = self.goal_target_comparator.unwrap().clone();
        }
        return result;
    }

    /// Returns the goal target value of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn goal_target_value(&self) -> String {
        let mut result: String = "".to_string();
        if self.goal_target_value.is_some() {
            result = self.goal_target_value.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the limit mode of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn limit_mode(&self) -> i64 {
        let mut result: i64 = 0;
        if self.limit_mode.is_some() {
            result = self.limit_mode.unwrap().clone();
        }
        return result;
    }

    /// Returns the limit value of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn limit_value(&self) -> i64 {
        let mut result: i64 = 0;
        if self.limit_value.is_some() {
            result = self.limit_value.unwrap().clone();
        }
        return result;
    }

    /// Returns the race type of the race bracket configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn race_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.race_type.is_some() {
            result = self.race_type.unwrap().clone();
        }
        return result;
    }

}
