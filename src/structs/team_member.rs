
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    participant::*,
    player::*,
};

/// Equivalent for start.gg TeamMember.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTeamMember {

    pub id:                 Option<i64>,

    #[serde(rename(serialize = "isAlternate",   deserialize = "isAlternate"))]
    pub is_alternate:       Option<bool>,

    #[serde(rename(serialize = "isCaptain",     deserialize = "isCaptain"))]
    pub is_captain:         Option<bool>,

    #[serde(rename(serialize = "memberType",    deserialize = "memberType"))]
    pub member_type:        Option<i64>,
    pub participant:        Option<Box<GGParticipant>>,
    pub player:             Option<Box<GGPlayer>>,
    pub status:             Option<i64>,
}

impl GGTeamMember {

    /// Returns the id of the team member.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the team member is an alternate.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_alternate(&self) -> bool {
        let mut result: bool = false;
        if self.is_alternate.is_some() {
            result = self.is_alternate.unwrap().clone();
        }
        return result;
    }

    /// Returns if the team member is a captain.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_captain(&self) -> bool {
        let mut result: bool = false;
        if self.is_captain.is_some() {
            result = self.is_captain.unwrap().clone();
        }
        return result;
    }

    /// Returns the member type of the team member.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn member_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.member_type.is_some() {
            result = self.member_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the participant of the team member.
    ///
    /// Returns an empty participant if not set or wasn't queried.
    pub fn participant(&self) -> GGParticipant {
        let mut result: GGParticipant = Default::default();
        if self.participant.is_some() {
            result = *self.participant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the player of the team member.
    ///
    /// Returns an empty player if not set or wasn't queried.
    pub fn player(&self) -> GGPlayer {
        let mut result: GGPlayer = Default::default();
        if self.player.is_some() {
            result = *self.player.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the status of the team member.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn status(&self) -> i64 {
        let mut result: i64 = 0;
        if self.status.is_some() {
            result = self.status.unwrap().clone();
        }
        return result;
    }

}
