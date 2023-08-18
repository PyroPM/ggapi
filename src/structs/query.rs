
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    event::*,
    participant::*,
    phase::*,
    phase_group::*,
    player::*,
    tournament::*,
    user::*,
    videogame::*,
};

/// Equivalent for start.gg Query.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see tournaments()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Serialize, Deserialize)]
pub struct GGData {

    #[serde(rename(serialize = "currentUser",   deserialize = "currentUser"))]
    pub current_user:   Option<Box<GGUser>>,
    pub entrant:        Option<Box<GGEntrant>>,
    pub event:          Option<Box<GGEvent>>,
    // pub league:         Option<league::GGLeague>,
    // pub leagues:        Option<league::GGLeagues>,
    pub participant:    Option<Box<GGParticipant>>,
    pub phase:          Option<Box<GGPhase>>,
    
    #[serde(rename(serialize = "phaseGroup",    deserialize = "phaseGroup"))]
    pub phase_group:    Option<Box<GGPhaseGroup>>,
    pub player:         Option<Box<GGPlayer>>,
    // pub seed:           Option<seed::GGSeed>,
    // pub set:            Option<set::GGSet>,
    // pub shop:           Option<shop::GGShop>,
    // pub stream:         Option<streams::GGStreams>,
    // pub stream_queue:   Option<stream_queue::GGStreamQueue>,
    // pub team:           Option<team::GGTeam>,
    pub tournament:     Option<Box<GGTournament>>,
    pub tournaments:    Option<Box<GGTournaments>>,
    pub user:           Option<Box<GGUser>>,
    pub videogame:      Option<Box<GGVideogame>>,
    pub videogames:     Option<Box<GGVideogames>>,

}

impl GGData {

    /// Returns the current user.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn current_user(&self) -> GGUser {
        let mut result: GGUser = Default::default();
        if self.current_user.is_some() {
            result = *self.current_user.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the entrant.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the event.
    ///
    /// Returns an empty event if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the participant.
    ///
    /// Returns an empty participant if not set or wasn't queried.
    pub fn participant(&self) -> GGParticipant {
        let mut result: GGParticipant = Default::default();
        if self.participant.is_some() {
            result = *self.participant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the phase.
    ///
    /// Returns an empty phase if not set or wasn't queried.
    pub fn phase(&self) -> GGPhase {
        let mut result: GGPhase = Default::default();
        if self.phase.is_some() {
            result = *self.phase.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the phase group.
    ///
    /// Returns an empty phase group if not set or wasn't queried.
    pub fn phase_group(&self) -> GGPhaseGroup {
        let mut result: GGPhaseGroup = Default::default();
        if self.phase_group.is_some() {
            result = *self.phase_group.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the player.
    ///
    /// Returns an empty player if not set or wasn't queried.
    pub fn player(&self) -> GGPlayer {
        let mut result: GGPlayer = Default::default();
        if self.player.is_some() {
            result = *self.player.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the tournament.
    ///
    /// Returns an empty tournament if not set or wasn't queried.
    pub fn tournament(&self) -> GGTournament {
        let mut result: GGTournament = Default::default();
        if self.tournament.is_some() {
            result = *self.tournament.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns a vector of tournaments.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn tournaments(&self) -> Vec<GGTournament> {
        let mut result: Vec<GGTournament> = Vec::new();
        if self.tournaments.is_some() {
            for tournament in &self.tournaments.as_ref().unwrap().nodes {
                result.push(tournament.clone());
            }
        }
        return result;
    }

    /// Returns the user.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn user(&self) -> GGUser {
        let mut result: GGUser = Default::default();
        if self.user.is_some() {
            result = *self.user.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the videogame.
    ///
    /// Returns an empty videogame if not set or wasn't queried.
    pub fn videogame(&self) -> GGVideogame {
        let mut result: GGVideogame = Default::default();
        if self.videogame.is_some() {
            result = *self.videogame.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns a vector of videogames.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn videogames(&self) -> Vec<GGVideogame> {
        let mut result: Vec<GGVideogame> = Vec::new();
        if self.videogames.is_some() {
            for videogame in &self.videogames.as_ref().unwrap().nodes {
                result.push(videogame.clone());
            }
        }
        return result;
    }

}
