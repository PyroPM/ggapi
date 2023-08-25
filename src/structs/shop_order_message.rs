
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    page_info::*,
    player::*,
};

/// Equivalent for start.gg ShopOrderMessageConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGShopOrderMessageConnection {
    pub nodes:      Vec<GGShopOrderMessage>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGShopOrderMessageConnection {

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

/// Equivalent for start.gg ShopOrderMessage.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGShopOrderMessage {

    pub gamertag:   Option<String>,
    pub id:         Option<i64>,
    pub message:    Option<String>,
    pub name:       Option<String>,
    pub player:     Option<Box<GGPlayer>>,
    pub total:      Option<f64>,
}

impl GGShopOrderMessage {

    /// Returns the gamertag of the shop order message.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn gamertag(&self) -> String {
        let mut result: String = "".to_string();
        if self.gamertag.is_some() {
            result = self.gamertag.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the shop order message.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the message of the shop order message.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn message(&self) -> String {
        let mut result: String = "".to_string();
        if self.message.is_some() {
            result = self.message.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the name of the shop order message.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the player of the shop order message.
    ///
    /// Returns an empty player if not set or wasn't queried.
    pub fn player(&self) -> GGPlayer {
        let mut result: GGPlayer = Default::default();
        if self.player.is_some() {
            result = *self.player.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the total of the shop order message.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn total(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.total.is_some() {
            result = self.total.unwrap().clone();
        }
        return result;
    }

}
