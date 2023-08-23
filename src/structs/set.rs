
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    event::*,
    game::*,
    image::*,
    page_info::*,
    phase_group::*,
    set_slot::*,
    stations::*,
    streams::*,
};

/// Equivalent for start.gg SetConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGSetConnection {
    pub nodes:      Vec<GGSet>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGSetConnection {

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

/// Equivalent for start.gg Set.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGSet {
    
    #[serde(rename(serialize = "completedAt",       deserialize = "completedAt"))]
    pub completed_at:       Option<i64>,
    
    #[serde(rename(serialize = "createdAt",         deserialize = "createdAt"))]
    pub created_at:         Option<i64>,

    #[serde(rename(serialize = "displayScore",      deserialize = "displayScore"))]
    pub display_score:      Option<String>,
    pub event:              Option<Box<GGEvent>>,

    #[serde(rename(serialize = "fullRoundText",     deserialize = "fullRoundText"))]
    pub full_round_text:    Option<String>,
    pub game:               Option<Box<GGGame>>,
    pub games:              Option<Vec<GGGame>>,

    #[serde(rename(serialize = "hasPlaceholder",    deserialize = "hasPlaceholder"))]
    pub has_placeholder:    Option<bool>,
    pub id:                 Option<i64>,
    pub identifier:         Option<String>,
    pub images:             Option<Vec<GGImage>>,

    #[serde(rename(serialize = "lPlacement",        deserialize = "lPlacement"))]
    pub l_placement:        Option<i64>,

    #[serde(rename(serialize = "phaseGroup",        deserialize = "phaseGroup"))]
    pub phase_group:        Option<Box<GGPhaseGroup>>,
    pub round:              Option<i64>,

    #[serde(rename(serialize = "setGamesType",      deserialize = "setGamesType"))]
    pub set_games_type:     Option<i64>,
    pub slots:              Option<Vec<GGSetSlot>>,
    
    #[serde(rename(serialize = "startAt",           deserialize = "startAt"))]
    pub start_at:           Option<i64>,

    #[serde(rename(serialize = "startedAt",         deserialize = "startedAt"))]
    pub started_at:         Option<i64>,
    pub state:              Option<i64>,
    pub station:            Option<Box<GGStations>>,
    pub stream:             Option<Box<GGStreams>>,

    #[serde(rename(serialize = "totalGames",        deserialize = "totalGames"))]
    pub total_games:        Option<i64>,

    #[serde(rename(serialize = "vodUrl",            deserialize = "vodUrl"))]
    pub vod_url:            Option<String>,

    #[serde(rename(serialize = "wPlacement",        deserialize = "wPlacement"))]
    pub w_placement:        Option<i64>,

    #[serde(rename(serialize = "winnerId",          deserialize = "winnerId"))]
    pub winner_id:          Option<i64>,

}

impl GGSet {

    /// Returns the time the set was completed.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn completed_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.completed_at.is_some() {
            result = self.completed_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the time the set was created.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn created_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.created_at.is_some() {
            result = self.created_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the display score of the set.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn display_score(&self) -> String {
        let mut result: String = "".to_string();
        if self.display_score.is_some() {
            result = self.display_score.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the event the set is in.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the full round text of the set.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn full_round_text(&self) -> String {
        let mut result: String = "".to_string();
        if self.full_round_text.is_some() {
            result = self.full_round_text.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the requested game of the set.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn game(&self) -> GGGame {
        let mut result: GGGame = Default::default();
        if self.game.is_some() {
            result = *self.game.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the games in the set.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn games(&self) -> Vec<GGGame> {
        let mut result: Vec<GGGame> = Vec::new();
        if self.games.is_some() {
            for game in self.games.as_ref().unwrap() {
                result.push(game.clone());
            }
        }
        return result;
    }

    /// Returns if the set has a placeholder.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn has_placeholder(&self) -> bool {
        let mut result: bool = false;
        if self.has_placeholder.is_some() {
            result = self.has_placeholder.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the identifier of the set.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn identifier(&self) -> String {
        let mut result: String = "".to_string();
        if self.identifier.is_some() {
            result = self.identifier.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the images in the set.
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

    /// Returns the loser's placement of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn l_placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.l_placement.is_some() {
            result = self.l_placement.unwrap().clone();
        }
        return result;
    }

    /// Returns the phase group of the set.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn phase_group(&self) -> GGPhaseGroup {
        let mut result: GGPhaseGroup = Default::default();
        if self.phase_group.is_some() {
            result = *self.phase_group.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the round of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn round(&self) -> i64 {
        let mut result: i64 = 0;
        if self.round.is_some() {
            result = self.round.unwrap().clone();
        }
        return result;
    }

    /// Returns the set games type of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn set_games_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.set_games_type.is_some() {
            result = self.set_games_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the slots in the set.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn slots(&self) -> Vec<GGSetSlot> {
        let mut result: Vec<GGSetSlot> = Vec::new();
        if self.slots.is_some() {
            for slot in self.slots.as_ref().unwrap() {
                result.push(slot.clone());
            }
        }
        return result;
    }

    /// Returns the time the set starts at.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn start_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.start_at.is_some() {
            result = self.start_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the time the set started at.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn started_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.started_at.is_some() {
            result = self.started_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the state of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

    /// Returns the station of the set.
    ///
    /// Returns an empty station if not set or wasn't queried.
    pub fn station(&self) -> GGStations {
        let mut result: GGStations = Default::default();
        if self.station.is_some() {
            result = *self.station.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream of the set.
    ///
    /// Returns an empty stream if not set or wasn't queried.
    pub fn stream(&self) -> GGStreams {
        let mut result: GGStreams = Default::default();
        if self.stream.is_some() {
            result = *self.stream.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the total games of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn total_games(&self) -> i64 {
        let mut result: i64 = 0;
        if self.total_games.is_some() {
            result = self.total_games.unwrap().clone();
        }
        return result;
    }

    /// Returns the VOD url of the set.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn vod_url(&self) -> String {
        let mut result: String = "".to_string();
        if self.vod_url.is_some() {
            result = self.vod_url.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the winner's placement of the set.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn w_placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.w_placement.is_some() {
            result = self.w_placement.unwrap().clone();
        }
        return result;
    }

    /// Returns the winner id of the set.
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
