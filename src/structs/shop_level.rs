
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    image::*,
    page_info::*,
};

/// Equivalent for start.gg ShopLevelConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGShopLevelConnection {
    pub nodes:      Vec<GGShopLevel>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGShopLevelConnection {

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

/// Equivalent for start.gg ShopLevel.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see images()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGShopLevel {
    
    #[serde(rename(serialize = "currAmount",    deserialize = "currAmount"))]
    pub curr_amount:    Option<f64>,
    pub description:    Option<String>,

    #[serde(rename(serialize = "goalAmount",    deserialize = "goalAmount"))]
    pub goal_amount:    Option<f64>,
    pub id:             Option<i64>,
    pub images:         Option<Vec<GGImage>>,
    pub name:           Option<String>,

}

impl GGShopLevel {

    /// Returns the current amount of the shop level.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn curr_amount(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.curr_amount.is_some() {
            result = self.curr_amount.unwrap().clone();
        }
        return result;
    }

    /// Returns the description of the shop level.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn description(&self) -> String {
        let mut result: String = "".to_string();
        if self.description.is_some() {
            result = self.description.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the goal amount of the shop level.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn goal_amount(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.goal_amount.is_some() {
            result = self.goal_amount.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the shop level.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images of the shop level.
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

    /// Returns the name of the shop level.
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
