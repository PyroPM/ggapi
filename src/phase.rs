
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    event::*,
};

/// Equivalent for start.gg Phase.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see phase_groups()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGPhase {
    
    #[serde(rename(serialize = "bracketType",   deserialize = "bracketType"))]
    pub bracket_type:               Option<i64>,
    pub event:                      Option<Box<GGEvent>>,

    #[serde(rename(serialize = "groupCount",    deserialize = "groupCount"))]
    pub group_count:                Option<i64>,
    pub id:                         Option<i64>,

    #[serde(rename(serialize = "isExhibition",  deserialize = "isExhibition"))]
    pub is_exhibition:              Option<bool>,
    pub name:                       Option<String>,

    #[serde(rename(serialize = "numSeeds",      deserialize = "numSeeds"))]
    pub num_seeds:                  Option<i64>,

    // pub phase_groups:               Option<GGPhaseGroups>,

    #[serde(rename(serialize = "phaseOrder",    deserialize = "phaseOrder"))]
    pub phase_order:                Option<i64>,
    // pub seeds:                      Option<GGSeeds>,
    // pub sets:                       Option<GGSets>,
    pub state:                      Option<i64>,
    // pub waves:                      Option<GGWave>,

}

impl GGPhase {

    /// Returns the bracket type of the phase.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn bracket_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.bracket_type.is_some() {
            result = self.bracket_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the event the phase is in.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the group count of the phase.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn group_count(&self) -> i64 {
        let mut result: i64 = 0;
        if self.group_count.is_some() {
            result = self.group_count.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the phase.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the phase is an exhibition.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_exhibition(&self) -> bool {
        let mut result: bool = false;
        if self.is_exhibition.is_some() {
            result = self.is_exhibition.unwrap().clone();
        }
        return result;
    }

    /// Returns the name of the phase.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the number of seeds in the phase.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_seeds(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_seeds.is_some() {
            result = self.num_seeds.unwrap().clone();
        }
        return result;
    }

    /// Returns the phase order of the phase.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn phase_order(&self) -> i64 {
        let mut result: i64 = 0;
        if self.phase_order.is_some() {
            result = self.phase_order.unwrap().clone();
        }
        return result;
    }

    /// Returns the state of the phase.
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
