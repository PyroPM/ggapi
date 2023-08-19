
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg TeamRosterSize.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTeamRosterSize {

    #[serde(rename(serialize = "maxAlternates", deserialize = "maxAlternates"))]
    pub max_alternates:     Option<i64>,

    #[serde(rename(serialize = "maxPlayers",    deserialize = "maxPlayers"))]
    pub max_players:        Option<i64>,

    #[serde(rename(serialize = "minAlternates", deserialize = "minAlternates"))]
    pub min_alternates:     Option<i64>,

    #[serde(rename(serialize = "minPlayers",    deserialize = "minPlayers"))]
    pub min_players:        Option<i64>,

}

impl GGTeamRosterSize {

    /// Returns the max alternates of the team roster size.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn max_alternates(&self) -> i64 {
        let mut result: i64 = 0;
        if self.max_alternates.is_some() {
            result = self.max_alternates.unwrap().clone();
        }
        return result;
    }

    /// Returns the max players of the team roster size.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn max_players(&self) -> i64 {
        let mut result: i64 = 0;
        if self.max_players.is_some() {
            result = self.max_players.unwrap().clone();
        }
        return result;
    }

    /// Returns the min alternates of the team roster size.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn min_alternates(&self) -> i64 {
        let mut result: i64 = 0;
        if self.min_alternates.is_some() {
            result = self.min_alternates.unwrap().clone();
        }
        return result;
    }

    /// Returns the min players of the team roster size.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn min_players(&self) -> i64 {
        let mut result: i64 = 0;
        if self.min_players.is_some() {
            result = self.min_players.unwrap().clone();
        }
        return result;
    }

}
