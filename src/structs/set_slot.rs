
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    seed::*,
    standing::*,
};

/// Equivalent for start.gg SetSlot.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGSetSlot {

    pub entrant:            Option<Box<GGEntrant>>,
    pub id:                 Option<i64>,

    #[serde(rename(serialize = "prereqId",          deserialize = "prereqId"))]
    pub prereq_id:          Option<String>,
    
    #[serde(rename(serialize = "prereqPlacement",   deserialize = "prereqPlacement"))]
    pub prereq_placement:   Option<i64>,

    #[serde(rename(serialize = "prereqType",        deserialize = "prereqType"))]
    pub prereq_type:        Option<String>,
    pub seed:               Option<Box<GGSeed>>,

    #[serde(rename(serialize = "slotIndex",         deserialize = "slotIndex"))]
    pub slot_index:         Option<i64>,
    pub standing:           Option<Box<GGStanding>>,
}

impl GGSetSlot {

    /// Returns the entrant of the set slot.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the set slot.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the prereq id of the set slot.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn prereq_id(&self) -> String {
        let mut result: String = "".to_string();
        if self.prereq_id.is_some() {
            result = self.prereq_id.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the prereq placement of the set slot.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn prereq_placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.prereq_placement.is_some() {
            result = self.prereq_placement.unwrap().clone();
        }
        return result;
    }

    /// Returns the prereq type of the set slot.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn prereq_type(&self) -> String {
        let mut result: String = "".to_string();
        if self.prereq_type.is_some() {
            result = self.prereq_type.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the seed of the set slot.
    ///
    /// Returns an empty seed if not set or wasn't queried.
    pub fn seed(&self) -> GGSeed {
        let mut result: GGSeed = Default::default();
        if self.seed.is_some() {
            result = *self.seed.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the slot index of the set slot.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn slot_index(&self) -> i64 {
        let mut result: i64 = 0;
        if self.slot_index.is_some() {
            result = self.slot_index.unwrap().clone();
        }
        return result;
    }

    /// Returns the standing of the set slot.
    ///
    /// Returns an empty standing if not set or wasn't queried.
    pub fn standing(&self) -> GGStanding {
        let mut result: GGStanding = Default::default();
        if self.standing.is_some() {
            result = *self.standing.as_ref().unwrap().clone();
        }
        return result;
    }

}
