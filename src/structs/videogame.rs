
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    character::*,
    image::*,
    page_info::*,
};

/// Equivalent for start.gg VideogameConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGVideogameConnection {
    pub nodes:      Vec<GGVideogame>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGVideogameConnection {

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

/// Equivalent for start.gg Videogame.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGVideogame {

    pub characters:     Option<Vec<GGCharacter>>,

    #[serde(rename(serialize = "displayName",   deserialize = "displayName"))]
    pub display_name:   Option<String>,
    pub id:             Option<i64>,
    pub images:         Option<Vec<GGImage>>,
    pub name:           Option<String>,
    pub slug:           Option<String>,

}

impl GGVideogame {

    /// Returns the characters for the videogame.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn characters(&self) -> Vec<GGCharacter> {
        let mut result: Vec<GGCharacter> = Vec::new();
        if self.characters.is_some() {
            for character in self.characters.as_ref().unwrap() {
                result.push(character.clone());
            }
        }
        return result;
    }

    /// Returns the display name of the videogame.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn display_name(&self) -> String {
        let mut result: String = "".to_string();
        if self.display_name.is_some() {
            result = self.display_name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the videogame.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images for the videogame.
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

    /// Returns the name of the videogame.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the slug of the videogame.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.slug.is_some() {
            result = self.slug.clone().unwrap().clone();
        }
        return result;
    }

}
