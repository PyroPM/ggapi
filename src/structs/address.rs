
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Address.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGAddress {

    pub city:           Option<String>,
    pub country:        Option<String>,

    #[serde(rename(serialize = "countryId", deserialize = "countryId"))]
    pub country_id:     Option<i64>,
    pub id:             Option<i64>,
    pub state:          Option<String>,

    #[serde(rename(serialize = "stateId",   deserialize = "stateId"))]
    pub state_id:       Option<i64>,

}

impl GGAddress {

    /// Returns the city of the address.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn city(&self) -> String {
        let mut result: String = "".to_string();
        if self.city.is_some() {
            result = self.city.clone().unwrap().clone();
        }
        return result;
    }
    
    /// Returns the country of the address.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn country(&self) -> String {
        let mut result: String = "".to_string();
        if self.country.is_some() {
            result = self.country.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the country id of the address.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn country_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.country_id.is_some() {
            result = self.country_id.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the address.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the state of the address.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn state(&self) -> String {
        let mut result: String = "".to_string();
        if self.state.is_some() {
            result = self.state.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the state id of the address.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state_id.is_some() {
            result = self.state_id.unwrap().clone();
        }
        return result;
    }

}
