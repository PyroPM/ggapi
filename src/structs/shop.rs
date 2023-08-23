
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    shop_level::*,
    shop_order_message::*,
};

/// Equivalent for start.gg Shop.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see levels()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Serialize, Deserialize)]
pub struct GGShop {

    pub id:         Option<i64>,
    pub levels:     Option<Box<GGShopLevelConnection>>,
    pub messages:   Option<Box<GGShopOrderMessageConnection>>,
    pub name:       Option<String>,
    pub slug:       Option<String>,
    pub url:        Option<String>,

}

impl GGShop {

    /// Returns the id of the shop.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the levels in the shop.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn levels(&self) -> Vec<GGShopLevel> {
        let mut result: Vec<GGShopLevel> = Vec::new();
        if self.levels.is_some() {
            for level in &self.levels.as_ref().unwrap().nodes {
                result.push(level.clone());
            }
        }
        return result;
    }

    /// Returns the messages in the shop.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn messages(&self) -> Vec<GGShopOrderMessage> {
        let mut result: Vec<GGShopOrderMessage> = Vec::new();
        if self.messages.is_some() {
            for message in &self.messages.as_ref().unwrap().nodes {
                result.push(message.clone());
            }
        }
        return result;
    }

    /// Returns the name of the shop.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the slug of the shop.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.slug.is_some() {
            result = self.slug.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the url of the shop.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn url(&self) -> String {
        let mut result: String = "".to_string();
        if self.url.is_some() {
            result = self.url.clone().unwrap().clone();
        }
        return result;
    }

}
