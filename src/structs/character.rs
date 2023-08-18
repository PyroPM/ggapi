
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    image::*,
};

/// Equivalent for start.gg Character.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGCharacter {

    pub id:         Option<i64>,
    pub images:     Option<Vec<GGImage>>,
    pub name:       Option<String>,

}

impl GGCharacter {

    /// Returns the id of the character.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images for the character.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn images(&self) -> Vec<GGImage> {
        let mut result: Vec<GGImage> = Vec::new();
        if self.images.is_some() {
            for image in self.images.as_ref().unwrap() {
                result.push(image.clone());
            }
        }
        return result;
    }

    /// Returns the name of the character.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

}
