
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    event::*,
    page_info::*,
    phase_group::*,
    player::*,
    standing_stats::*,
    tournament::*,
};

/// Equivalent for start.gg StandingContainer.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub enum GGStandingContainer {
    
    Tournament(GGTournament),
    Event(GGEvent),
    PhaseGroup(GGPhaseGroup),
    // GGSet,

}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStandingConnection {
    pub nodes:      Vec<GGStanding>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGStandingConnection {

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

/// Equivalent for start.gg Standing.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStanding {

    pub container:      Option<Box<GGStandingContainer>>,
    pub entrant:        Option<Box<GGEntrant>>,
    pub id:             Option<i64>,
    
    #[serde(rename(serialize = "isFinal",       deserialize = "isFinal"))]
    pub is_final:       Option<bool>,
    // pub metadata:       Option<JSON>,
    pub placement:      Option<i64>,
    pub player:         Option<Box<GGPlayer>>,
    pub stats:          Option<Box<GGStandingStats>>,

    #[serde(rename(serialize = "totalPoints",   deserialize = "totalPoints"))]
    pub total_points:   Option<f64>,

}

impl GGStanding {

    /// Returns the container of the standing.
    ///
    /// Returns an empty container if not set or wasn't queried.
    pub fn container(&self) -> i64 {
        let result: i64 = 0;
        // let mut result: GGStandingContainer;
        match **self.container.as_ref().unwrap() {
            GGStandingContainer::Tournament(_)  => println!("{}", "type: tournament"),
            GGStandingContainer::Event(_)       => println!("{}", "type: event"),
            GGStandingContainer::PhaseGroup(_)  => println!("{}", "type: phase group"),
        }
        if self.container.is_some() {
            //result = *self.container.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the entrant of the standing.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the standing.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the standing is final.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_final(&self) -> bool {
        let mut result: bool = false;
        if self.is_final.is_some() {
            result = self.is_final.unwrap().clone();
        }
        return result;
    }

    /// Returns the placement of the standing.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn placement(&self) -> i64 {
        let mut result: i64 = 0;
        if self.placement.is_some() {
            result = self.placement.unwrap().clone();
        }
        return result;
    }

    /// Returns the player of the standing.
    ///
    /// Returns an empty player if not set or wasn't queried.
    pub fn player(&self) -> GGPlayer {
        let mut result: GGPlayer = Default::default();
        if self.player.is_some() {
            result = *self.player.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the stats of the standing.
    ///
    /// Returns an empty stats if not set or wasn't queried.
    pub fn stats(&self) -> GGStandingStats {
        let mut result: GGStandingStats = Default::default();
        if self.stats.is_some() {
            result = *self.stats.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the total points of the standing.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn total_points(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.total_points.is_some() {
            result = self.total_points.unwrap().clone();
        }
        return result;
    }

}
