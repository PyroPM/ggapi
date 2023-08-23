
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    event::*,
    page_info::*,
    participant::*,
};

/// Equivalent for start.gg EntrantConnection.
#[derive(Clone, Serialize, Deserialize)]
pub struct GGEntrantConnection {
    pub nodes:      Vec<GGEntrant>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGEntrantConnection {

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

/// Equivalent for start.gg Entrant.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see paginated_sets()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEntrant {

    pub event:              Option<Box<GGEvent>>,
    pub id:                 Option<i64>,

    #[serde(rename(serialize = "initialSeedNum",    deserialize = "initialSeedNum"))]
    pub initial_seed_num:   Option<i64>,

    #[serde(rename(serialize = "isDisqualified",    deserialize = "isDisqualified"))]
    pub is_disqualified:    Option<bool>,
    pub name:               Option<String>,

    // #[serde(rename(serialize = "paginatedSets",     deserialize = "paginatedSets"))]
    // pub paginated_sets:     Option<GGSetConnection>,
    pub participants:       Option<Vec<GGParticipant>>,
    // pub seeds:              Option<Vec<GGSeed>>,
    pub skill:              Option<i64>,
    // pub standing:           Option<GGStanding>,
    // pub streams:            Option<Vec<GGStreams>>,
    // pub team:               Option<GGTeam>,

}

impl GGEntrant {

    /// Returns the event the entrant is in.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the entrant.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the initial seed of the entrant.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn initial_seed_num(&self) -> i64 {
        let mut result: i64 = 0;
        if self.initial_seed_num.is_some() {
            result = self.initial_seed_num.unwrap().clone();
        }
        return result;
    }

    /// Returns if the entrant is disqualified.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_disqualified(&self) -> bool {
        let mut result: bool = false;
        if self.is_disqualified.is_some() {
            result = self.is_disqualified.unwrap().clone();
        }
        return result;
    }

    /// Returns the name of the entrant.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the participants associated with the entrant.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn participants(&self) -> Vec<GGParticipant> {
        let mut result: Vec<GGParticipant> = Vec::new();
        if self.participants.is_some() {
            for participant in self.participants.as_ref().unwrap() {
                result.push(participant.clone());
            }
        }
        return result;
    }

    /// Returns the skill of the entrant.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn skill(&self) -> i64 {
        let mut result: i64 = 0;
        if self.skill.is_some() {
            result = self.skill.unwrap().clone();
        }
        return result;
    }

}
