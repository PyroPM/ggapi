
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    enums::*,
    page_info::*,
    phase::*,
    set::*,
};

/// Equivalent for start.gg PhaseGroupConnection.
#[derive(Clone, Serialize, Deserialize)]
pub struct GGPhaseGroupConnection {
    pub nodes:      Vec<GGPhaseGroup>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGPhaseGroupConnection {

    /// Returns the page info of the connection.
    ///
    /// Returns empty page info if not set or wasn't queried.
    pub fn page_info(&self) -> GGPageInfo {
        let mut result: GGPageInfo = Default::default();
        if self.page_info.is_some() {
            result = *self.page_info.as_ref().unwrap().clone();
        }
        return result;
    }

}

/// Equivalent for start.gg PhaseGroup.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see seeds()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGPhaseGroup {
    
    #[serde(rename(serialize = "bracketType",       deserialize = "bracketType"))]
    pub bracket_type:               Option<i64>,

    #[serde(rename(serialize = "bracketUrl",        deserialize = "bracketUrl"))]
    pub bracket_url:                Option<String>,

    #[serde(rename(serialize = "displayIdentifier", deserialize = "displayIdentifier"))]
    pub display_identifier:         Option<String>,

    #[serde(rename(serialize = "firstRoundTime",    deserialize = "firstRoundTime"))]
    pub first_round_time:           Option<i64>,
    pub id:                         Option<GGID>,

    #[serde(rename(serialize = "numRounds",         deserialize = "numRounds"))]
    pub num_rounds:                 Option<i64>,
    pub phase:                      Option<Box<GGPhase>>,

    // pub progressions_out:           Option<GGProgression>,
    // pub rounds:                     Option<GGRound>,
    // pub seed_map:                   JSON,
    // pub seeds:                      Option<GGSeeds>,
    pub sets:                       Option<GGSetConnection>,
    // pub standings:                  Option<GGStandings>,

    #[serde(rename(serialize = "startAt",                   deserialize = "startAt"))]
    pub start_at:                   Option<i64>,
    pub state:                      Option<i64>,

    // pub tiebreaker_order:           JSON,
    // pub wave:                       Option<Box<GGWave>>,

}

impl GGPhaseGroup {

    /// Returns the bracket type of the phase group.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn bracket_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.bracket_type.is_some() {
            result = self.bracket_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the bracket url of the phase group.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn bracket_url(&self) -> String {
        let mut result: String = "".to_string();
        if self.bracket_url.is_some() {
            result = self.bracket_url.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the display indentifier of the phase group.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn display_identifier(&self) -> String {
        let mut result: String = "".to_string();
        if self.display_identifier.is_some() {
            result = self.display_identifier.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the time the first round starts.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn first_round_time(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.first_round_time.is_some() {
            result = self.first_round_time.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the id of the phase group.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> GGID {
        let mut result: GGID = GGID::Int(0);
        if self.id.is_some() {
            match self.id.clone().unwrap() {
                GGID::Int(_) => result = self.id.as_ref().unwrap().clone(),
                GGID::String(_) => result = self.id.as_ref().unwrap().clone(),
            };
        }
        return result;
    }

    /// Returns the number of round in the phase group.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_rounds(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_rounds.is_some() {
            result = self.num_rounds.unwrap().clone();
        }
        return result;
    }

    /// Returns the phase the phase group is in.
    ///
    /// Returns an empty tournament if not set or wasn't queried.
    pub fn phase(&self) -> GGPhase {
        let mut result: GGPhase = Default::default();
        if self.phase.is_some() {
            result = *self.phase.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the sets in the phase group.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn sets(&self) -> Vec<GGSet> {
        let mut result: Vec<GGSet> = Vec::new();
        if self.sets.is_some() {
            for set in &self.sets.as_ref().unwrap().nodes {
                result.push(set.clone());
            }
        }
        return result;
    }

    /// Returns the time the phase group starts.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn start_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.start_at.is_some() {
            result = self.start_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the state of the phase group.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

}
