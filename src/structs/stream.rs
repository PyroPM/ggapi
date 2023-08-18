
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Stream.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStream {

    pub id:         Option<i64>,

    #[serde(rename(serialize = "isOnline",  deserialize = "isOnline"))]
    pub is_online:  Option<bool>,
    pub name:       Option<String>,
    pub r#type:     Option<i64>,

}

impl GGStream {

    /// Returns the id of the stream.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the stream is online.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn is_online(&self) -> bool {
        let mut result: bool = false;
        if self.is_online.is_some() {
            result = self.is_online.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the name of the stream.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the type of the stream.
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
