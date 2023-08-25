
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Image.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGImage {

    pub height:     Option<f64>,
    pub id:         Option<i64>,
    pub ratio:      Option<f64>,
    pub r#type:     Option<String>,
    pub url:        Option<String>,
    pub width:      Option<f64>,

}

impl GGImage {

    /// Returns the height of the image.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn height(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.height.is_some() {
            result = self.height.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the image.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the ratio of the image.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn ratio(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.ratio.is_some() {
            result = self.ratio.unwrap().clone();
        }
        return result;
    }

    /// Returns the type of the image.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn r#type(&self) -> String {
        let mut result: String = "".to_string();
        if self.r#type.is_some() {
            result = self.r#type.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the url of the image.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn url(&self) -> String {
        let mut result: String = "".to_string();
        if self.url.is_some() {
            result = self.url.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the width of the image.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn width(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.width.is_some() {
            result = self.width.unwrap().clone();
        }
        return result;
    }

}
