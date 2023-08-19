
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    event::*,
    event_team::*,
    image::*,
    team_member::*,
};

/// Equivalent for start.gg GlobalTeam.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGGlobalTeam {
    
    pub discriminator:      Option<String>,
    pub entrant:            Option<Box<GGEntrant>>,
    pub event:              Option<Box<GGEvent>>,

    #[serde(rename(serialize = "eventTeams",    deserialize = "eventTeams"))]
    pub event_teams:        Option<GGEventTeams>,
    pub id:                 Option<i64>,
    pub images:             Option<Vec<GGImage>>,

    #[serde(rename(serialize = "leagueTeams",   deserialize = "leagueTeams"))]
    pub league_teams:       Option<GGEventTeams>,
    pub members:            Option<Vec<GGTeamMember>>,
    pub name:               Option<String>,

}

impl GGGlobalTeam {

    /// Returns the discriminator of the global team.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn discriminator(&self) -> String {
        let mut result: String = "".to_string();
        if self.discriminator.is_some() {
            result = self.discriminator.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the entrant of the global team.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the event of the global team.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the event teams in the global team.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn event_teams(&self) -> Vec<GGEventTeam> {
        let mut result: Vec<GGEventTeam> = Vec::new();
        if self.event_teams.is_some() {
            for event_team in &self.event_teams.as_ref().unwrap().nodes {
                result.push(event_team.clone());
            }
        }
        return result;
    }

    /// Returns the id of the global team.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images of the global team.
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

    /// Returns the league teams in the global team.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn league_teams(&self) -> Vec<GGEventTeam> {
        let mut result: Vec<GGEventTeam> = Vec::new();
        if self.league_teams.is_some() {
            for league_team in &self.league_teams.as_ref().unwrap().nodes {
                result.push(league_team.clone());
            }
        }
        return result;
    }

    /// Returns the members in the global team.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn members(&self) -> Vec<GGTeamMember> {
        let mut result: Vec<GGTeamMember> = Vec::new();
        if self.members.is_some() {
            for member in self.members.as_ref().unwrap() {
                result.push(member.clone());
            }
        }
        return result;
    }

    /// Returns the name of the global team.
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
