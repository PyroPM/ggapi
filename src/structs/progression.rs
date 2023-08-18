
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    phase::*,
    phase_group::*,
};

/// Equivalent for start.gg Progression.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGProgression {
    
    pub id:                     Option<i64>,

    #[serde(rename(serialize = "originOrder",       deserialize = "originOrder"))]
    pub origin_order:           Option<i64>,

    #[serde(rename(serialize = "originPhase",       deserialize = "originPhase"))]
    pub origin_phase:           Option<Box<GGPhase>>,

    #[serde(rename(serialize = "originPhaseGroup",  deserialize = "originPhaseGroup"))]
    pub origin_phase_group:     Option<Box<GGPhaseGroup>>,

    #[serde(rename(serialize = "originPlacement",   deserialize = "originPlacement"))]
    pub origin_placement:       Option<i64>,

}

impl GGProgression {

    /// Returns the id of the progression.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the origin order of the progression.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn origin_order(&self) -> i64 {
        let mut result: i64 = 0;
        if self.origin_order.is_some() {
            result = self.origin_order.unwrap().clone();
        }
        return result;
    }

    /// Returns the origin phase of the progression.
    ///
    /// Returns an empty phase if not set or wasn't queried.
    pub fn origin_phase(&self) -> GGPhase {
        let mut result: GGPhase = Default::default();
        if self.origin_phase.is_some() {
            result = *self.origin_phase.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the origin phase group of the progression.
    ///
    /// Returns an empty phase group if not set or wasn't queried.
    pub fn origin_phase_group(&self) -> GGPhaseGroup {
        let mut result: GGPhaseGroup = Default::default();
        if self.origin_phase_group.is_some() {
            result = *self.origin_phase_group.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the origin placement of the progression.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn origin_placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.origin_placement.is_some() {
            result = self.origin_placement.unwrap().clone();
        }
        return result;
    }

}
