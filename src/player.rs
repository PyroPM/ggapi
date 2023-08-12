
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    user::*,
};

/// Equivalent for start.gg Player.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see sets()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGPlayer {

    #[serde(rename(serialize = "gamerTag",          deserialize = "gamerTag"))]
    pub gamer_tag:          Option<String>,
    pub id:                 Option<i64>,
    pub prefix:             Option<String>,
    // pub rankings:           Option<GGPlayerRank>,
    
    // #[serde(rename(serialize = "recentStandings",   deserialize = "recentStandings"))]
    // pub recent_standings:   Option<GGStanding>,
    // pub sets:               Option<GGSets>,
    pub user:               Option<Box<GGUser>>,

}

impl GGPlayer {

    /// Returns the gamer tag of the player.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn gamer_tag(&self) -> String {
        let mut result: String = "".to_string();
        if self.gamer_tag.is_some() {
            result = self.gamer_tag.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the player.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the prefix of the player.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn prefix(&self) -> String {
        let mut result: String = "".to_string();
        if self.prefix.is_some() {
            result = self.prefix.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the user of the player.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn user(&self) -> GGUser {
        let mut result: GGUser = Default::default();
        if self.user.is_some() {
            result = *self.user.as_ref().unwrap().clone();
        }
        return result;
    }

}
