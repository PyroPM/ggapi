
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    page_info::*,
    phase::*,
    phase_group::*,
    player::*,
    progression::*,
};

/// Equivalent for start.gg SeedConnection.
#[derive(Clone, Serialize, Deserialize)]
pub struct GGSeedConnection {
    pub nodes:      Vec<GGSeed>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGSeedConnection {

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

/// Equivalent for start.gg Seed.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGSeed {
    
    // pub checked_in_participants: JSON
    pub entrant:                    Option<Box<GGEntrant>>,

    #[serde(rename(serialize = "groupSeedNum",      deserialize = "groupSeedNum"))]
    pub group_seed_num:             Option<i64>,
    pub id:                         Option<i64>,

    #[serde(rename(serialize = "isBye",             deserialize = "isBye"))]
    pub is_bye:                     Option<bool>,

    #[serde(rename(serialize = "phase",             deserialize = "phase"))]
    pub phase:                      Option<Box<GGPhase>>,

    #[serde(rename(serialize = "phaseGroup",        deserialize = "phaseGroup"))]
    pub phase_group:                Option<Box<GGPhaseGroup>>,

    #[serde(rename(serialize = "placeholderName",   deserialize = "placeholderName"))]
    pub placeholder_name:           Option<String>,
    pub placement:                  Option<i64>,
    pub players:                    Option<Vec<GGPlayer>>,

    #[serde(rename(serialize = "progressionSeedId", deserialize = "progressionSeedId"))]
    pub progression_seed_id:        Option<i64>,

    #[serde(rename(serialize = "progressionSource", deserialize = "progressionSource"))]
    pub progression_source:         Option<Box<GGProgression>>,

    #[serde(rename(serialize = "seedNum",           deserialize = "seedNum"))]
    pub seed_num:                   Option<i64>,
    // pub set_record_without_byes:    JSON
    // pub standings:                  Option<Vec<GGStanding>>

}

impl GGSeed {

    /// Returns the entrant of the seed.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the group seed number of the seed.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn group_seed_num(&self) -> i64 {
        let mut result: i64 = 0;
        if self.group_seed_num.is_some() {
            result = self.group_seed_num.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the seed.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the seed is a bye.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_bye(&self) -> bool {
        let mut result: bool = false;
        if self.is_bye.is_some() {
            result = self.is_bye.unwrap().clone();
        }
        return result;
    }

    /// Returns the phase of the seed.
    ///
    /// Returns an empty phase if not set or wasn't queried.
    pub fn phase(&self) -> GGPhase {
        let mut result: GGPhase = Default::default();
        if self.phase.is_some() {
            result = *self.phase.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the phase group of the seed.
    ///
    /// Returns an empty phase group if not set or wasn't queried.
    pub fn phase_group(&self) -> GGPhaseGroup {
        let mut result: GGPhaseGroup = Default::default();
        if self.phase_group.is_some() {
            result = *self.phase_group.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the placeholder name of the seed.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn placeholder_name(&self) -> String {
        let mut result: String = "".to_string();
        if self.placeholder_name.is_some() {
            result = self.placeholder_name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the placement of the seed.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.placement.is_some() {
            result = self.placement.unwrap().clone();
        }
        return result;
    }

    /// Returns the players in the event.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn players(&self) -> Vec<GGPlayer> {
        let mut result: Vec<GGPlayer> = Vec::new();
        if self.players.is_some() {
            for player in self.players.as_ref().unwrap() {
                result.push(player.clone());
            }
        }
        return result;
    }

    /// Returns the progression source of the seed.
    ///
    /// Returns an empty progression if not set or wasn't queried.
    pub fn progression_source(&self) -> GGProgression {
        let mut result: GGProgression = Default::default();
        if self.progression_source.is_some() {
            result = *self.progression_source.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the seed number of the seed.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn seed_num(&self) -> i64 {
        let mut result: i64 = 0;
        if self.seed_num.is_some() {
            result = self.seed_num.unwrap().clone();
        }
        return result;
    }

}
