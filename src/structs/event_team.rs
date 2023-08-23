
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    event::*,
    global_team::*,
    image::*,
    page_info::*,
    team_member::*,
};

/// Equivalent for start.gg EventTeamConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEventTeamConnection {
    pub nodes:      Vec<GGEventTeam>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGEventTeamConnection {

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

/// Equivalent for start.gg EventTeam.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEventTeam {
    
    pub discriminator:      Option<String>,
    pub entrant:            Option<Box<GGEntrant>>,
    pub event:              Option<Box<GGEvent>>,

    #[serde(rename(serialize = "globalTeam",    deserialize = "globalTeam"))]
    pub global_team:        Option<Box<GGGlobalTeam>>,
    pub id:                 Option<i64>,
    pub images:             Option<Vec<GGImage>>,
    pub members:            Option<Vec<GGTeamMember>>,
    pub name:               Option<String>,

}

impl GGEventTeam {

    /// Returns the discriminator of the event team.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn discriminator(&self) -> String {
        let mut result: String = "".to_string();
        if self.discriminator.is_some() {
            result = self.discriminator.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the entrant of the event team.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.entrant.is_some() {
            result = *self.entrant.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the event of the event team.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn event(&self) -> GGEvent {
        let mut result: GGEvent = Default::default();
        if self.event.is_some() {
            result = *self.event.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the global team of the event team.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn global_team(&self) -> GGGlobalTeam {
        let mut result: GGGlobalTeam = Default::default();
        if self.global_team.is_some() {
            result = *self.global_team.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the event team.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images of the event team.
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

    /// Returns the members in the event team.
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

    /// Returns the name of the event team.
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
