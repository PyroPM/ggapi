
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    page_info::*,
};

/// Equivalent for start.gg EventOwnerConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEventOwnerConnection {
    pub nodes:      Vec<GGEventOwner>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGEventOwnerConnection {

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

/// Equivalent for start.gg EventOwner.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEventOwner {

    pub email:          Option<String>,

    #[serde(rename(serialize = "eventId",   deserialize = "eventId"))]
    pub event_id:       Option<i64>,

    #[serde(rename(serialize = "gamerTag",  deserialize = "gamerTag"))]
    pub gamer_tag:      Option<String>,

    #[serde(rename(serialize = "fullName",  deserialize = "fullName"))]
    pub full_name:      Option<String>,

}

impl GGEventOwner {

    /// Returns the email of the event owner.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn email(&self) -> String {
        let mut result: String = "".to_string();
        if self.email.is_some() {
            result = self.email.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the event id of the event owner.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn event_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.event_id.is_some() {
            result = self.event_id.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the gamer tag of the event owner.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn gamer_tag(&self) -> String {
        let mut result: String = "".to_string();
        if self.gamer_tag.is_some() {
            result = self.gamer_tag.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the full name of the event owner.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn full_name(&self) -> String {
        let mut result: String = "".to_string();
        if self.full_name.is_some() {
            result = self.full_name.clone().unwrap().clone();
        }
        return result;
    }

}
