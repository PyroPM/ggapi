
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game_selection::*,
    image::*,
    stage::*,
};

/// Equivalent for start.gg Game.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGGame {

    pub id:             Option<i64>,
    pub images:         Option<Vec<GGImage>>,

    #[serde(rename(serialize = "orderNum",  deserialize = "orderNum"))]
    pub order_num:      Option<i64>,
    pub selections:     Option<Vec<GGGameSelection>>,
    pub stage:          Option<Box<GGStage>>,
    pub state:          Option<i64>,

    #[serde(rename(serialize = "winnerId",  deserialize = "winnerId"))]
    pub winner_id:      Option<i64>,

}

impl GGGame {

    /// Returns the id of the game.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images of the game.
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

    /// Returns the order number of the game.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn order_num(&self) -> i64 {
        let mut result: i64 = 0;
        if self.order_num.is_some() {
            result = self.order_num.unwrap().clone();
        }
        return result;
    }

    /// Returns the selections of the game.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn selections(&self) -> Vec<GGGameSelection> {
        let mut result: Vec<GGGameSelection> = Vec::new();
        if self.selections.is_some() {
            for selection in self.selections.as_ref().unwrap() {
                result.push(selection.clone());
            }
        }
        return result;
    }

    /// Returns the stage of the game.
    ///
    /// Returns an empty stage if not set or wasn't queried.
    pub fn stage(&self) -> GGStage {
        let mut result: GGStage = Default::default();
        if self.stage.is_some() {
            result = *self.stage.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the state of the game.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

    /// Returns the winner id of the game.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn winner_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.winner_id.is_some() {
            result = self.winner_id.unwrap().clone();
        }
        return result;
    }

}
