
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    stream::*,
};

/// Equivalent for start.gg ProfileAuthorization.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGProfileAuthorization {

    #[serde(rename(serialize = "externalId",        deserialize = "externalId"))]
    pub external_id:        Option<String>,

    #[serde(rename(serialize = "externalUsername",  deserialize = "externalUsername"))]
    pub external_username:  Option<String>,

    pub id:                 Option<i64>,
    pub stream:             Option<Box<GGStream>>,
    pub r#type:             Option<i64>,

}

impl GGProfileAuthorization {

    /// Returns the external id of the profile authorization.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn external_id(&self) -> String {
        let mut result: String = "".to_string();
        if self.external_id.is_some() {
            result = self.external_id.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the external username of the profile authorization.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn external_username(&self) -> String {
        let mut result: String = "".to_string();
        if self.external_username.is_some() {
            result = self.external_username.clone().unwrap().clone();
        }
        return result;
    }
    
    /// Returns the id of the profile authorization.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the stream of the profile authorization.
    ///
    /// Returns an empty stream if not set or wasn't queried.
    pub fn stream(&self) -> GGStream {
        let mut result: GGStream = Default::default();
        if self.stream.is_some() {
            result = *self.stream.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the type of the profile authorization.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn r#type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.r#type.is_some() {
            result = self.r#type.unwrap().clone();
        }
        return result;
    }

}
