
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    image::*,
    team_member::*,
};

/// Equivalent for start.gg TeamConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTeamConnection {
    pub nodes: Vec<GGTeam>,
}

/// Equivalent for start.gg Team.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTeam {
    
    pub discriminator:      Option<String>,
    pub id:                 Option<i64>,
    pub images:             Option<Vec<GGImage>>,
    pub members:            Option<Vec<GGTeamMember>>,
    pub name:               Option<String>,

}

impl GGTeam {

    /// Returns the discriminator of the team.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn discriminator(&self) -> String {
        let mut result: String = "".to_string();
        if self.discriminator.is_some() {
            result = self.discriminator.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the team.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the images of the team.
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

    /// Returns the members in the team.
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

    /// Returns the name of the team.
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
