
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg RaceMatchConfig.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGRaceMatchConfig {

    #[serde(rename(serialize = "bracketType",               deserialize = "bracketType"))]
    pub bracket_type:                   Option<i64>,
    pub id:                             Option<i64>,

    #[serde(rename(serialize = "playerReportingEnabled",    deserialize = "playerReportingEnabled"))]
    pub player_reporting_enabled:       Option<bool>,

    #[serde(rename(serialize = "verificationMethods",       deserialize = "verificationMethods"))]
    pub verification_methods:           Option<Vec<i64>>,

    #[serde(rename(serialize = "verificationRequired",      deserialize = "verificationRequired"))]
    pub verification_required:          Option<bool>,

}

impl GGRaceMatchConfig {

    /// Returns the bracket type of the race match configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn bracket_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.bracket_type.is_some() {
            result = self.bracket_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the race match configuration.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the race match configuration has player reporting enabled.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn player_reporting_enabled(&self) -> bool {
        let mut result: bool = false;
        if self.player_reporting_enabled.is_some() {
            result = self.player_reporting_enabled.unwrap().clone();
        }
        return result;
    }

    /// Returns the verification methods of the race match configuration.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn verification_methods(&self) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        if self.verification_methods.is_some() {
            for verification_method in self.verification_methods.as_ref().unwrap() {
                result.push(verification_method.clone());
            }
        }
        return result;
    }

    /// Returns if the race match configuration has verification required.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn verification_required(&self) -> bool {
        let mut result: bool = false;
        if self.verification_required.is_some() {
            result = self.verification_required.unwrap().clone();
        }
        return result;
    }

}
