
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
    entrant::*,
    event::*,
    user::*,
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGParticipants {
    pub nodes: Vec<GGParticipant>,
}

/// Equivalent for start.gg Participant.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGParticipant {

    #[serde(rename(serialize = "checkedIn", deserialize = "checkedIn"))]
    pub checked_in:             Option<bool>,

    #[serde(rename(serialize = "checkedInAt", deserialize = "checkedInAt"))]
    pub checked_in_at:          Option<i64>,
    // pub connected_accounts:     JSON,
    // pub contact_info:           Option<GGContactInfo>,
    pub email:                  Option<String>,
    pub entrants:               Option<Vec<GGEntrant>>,
    pub events:                 Option<Vec<GGEvent>>,

    #[serde(rename(serialize = "gamerTag", deserialize = "gamerTag"))]
    pub gamer_tag:              Option<String>,
    pub id:                     Option<i64>,
    // pub images:                 Option<Vec<GGImage>>,
    // pub player:                 Option<GGPlayer>,
    pub prefix:                 Option<String>,
    // pub required_connections:   Option<GGProfileAuthorization>,
    pub user:                   Option<Box<GGUser>>,
    pub verified:               Option<bool>,
}

impl GGParticipant {

    /// Returns if the participant is checked in.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn checked_in(&self) -> bool {
        let mut result: bool = false;
        if self.checked_in.is_some() {
            result = self.checked_in.unwrap().clone();
        }
        return result;
    }

    /// Returns the time the participant checked in at.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn checked_in_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.checked_in_at.is_some() {
            result = self.checked_in_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the email of the participant.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn email(&self) -> String {
        let mut result: String = "".to_string();
        if self.email.is_some() {
            result = self.email.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the entrants associated with the participant.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn entrants(&self) -> Vec<GGEntrant> {
        let mut result: Vec<GGEntrant> = Vec::new();
        if self.entrants.is_some() {
            for entrant in self.entrants.as_ref().unwrap() {
                result.push(entrant.clone());
            }
        }
        return result;
    }

    /// Returns the events associated with the participant.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn events(&self) -> Vec<GGEvent> {
        let mut result: Vec<GGEvent> = Vec::new();
        if self.events.is_some() {
            for event in self.events.as_ref().unwrap() {
                result.push(event.clone());
            }
        }
        return result;
    }

    /// Returns the gamer tag of the participant.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn gamer_tag(&self) -> String {
        let mut result: String = "".to_string();
        if self.gamer_tag.is_some() {
            result = self.gamer_tag.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the participant.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the prefix of the participant.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn prefix(&self) -> String {
        let mut result: String = "".to_string();
        if self.prefix.is_some() {
            result = self.prefix.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the user of the participant.
    ///
    /// Returns an empty tournament if not set or wasn't queried.
    pub fn user(&self) -> GGUser {
        let mut result: GGUser = Default::default();
        if self.user.is_some() {
            result = *self.user.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns if the participant is verified.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn verified(&self) -> bool {
        let mut result: bool = false;
        if self.verified.is_some() {
            result = self.verified.unwrap().clone();
        }
        return result;
    }

}
