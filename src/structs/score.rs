
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Score.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGScore {

    #[serde(rename(serialize = "displayValue",  deserialize = "displayValue"))]
    pub display_value:      Option<String>,
    pub label:              Option<String>,
    pub value:              Option<f64>,

}

impl GGScore {

    /// Returns the display value of the score.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn display_value(&self) -> String {
        let mut result: String = "".to_string();
        if self.display_value.is_some() {
            result = self.display_value.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the label of the score.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn label(&self) -> String {
        let mut result: String = "".to_string();
        if self.label.is_some() {
            result = self.label.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the value of the score.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn value(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.value.is_some() {
            result = self.value.unwrap().clone();
        }
        return result;
    }

}
