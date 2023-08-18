
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    participant::*,
};

/// Equivalent for start.gg GameSelection.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGGameSelection {

    pub entrant:                Option<Box<GGEntrant>>,
    pub id:                     Option<i64>,

    #[serde(rename(serialize = "orderNum",          deserialize = "orderNum"))]
    pub order_num:              Option<i64>,
    pub participant:            Option<Box<GGParticipant>>,

    #[serde(rename(serialize = "selectionType",     deserialize = "selectionType"))]
    pub selection_type:         Option<i64>,

    #[serde(rename(serialize = "selectionValue",    deserialize = "selectionValue"))]
    pub selection_value:        Option<i64>,

}

impl GGGameSelection {

    /// Returns the entrant of the game selection.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the game selection.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the order number of the game selection.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn order_num(&self) -> i64 {
        let mut result: i64 = 0;
        if self.order_num.is_some() {
            result = self.order_num.unwrap().clone();
        }
        return result;
    }

    /// Returns the participant of the game selection.
    ///
    /// Returns an empty participant if not set or wasn't queried.
    pub fn participant(&self) -> GGParticipant {
        let mut result: GGParticipant = Default::default();
        if self.participant.is_some() {
            result = *self.participant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the selection type of the game selection.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn selection_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.selection_type.is_some() {
            result = self.selection_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the selection value of the game selection.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn selection_value(&self) -> i64 {
        let mut result: i64 = 0;
        if self.selection_value.is_some() {
            result = self.selection_value.unwrap().clone();
        }
        return result;
    }

}
