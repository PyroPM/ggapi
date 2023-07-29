
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    event::*,
    tournament::*,
};

/// Equivalent for start.gg User.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see tournaments()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Serialize, Deserialize)]
pub struct GGUser {

    // authorizations: Option<Vec<GGProfileAuthorization>>
    bio:            Option<String>,
    birthday:       Option<String>,
    discriminator:  Option<String>,
    email:          Option<String>,
    events:         Option<GGEvents>,

    #[serde(rename(serialize = "genderPronoun", deserialize = "genderPronoun"))]
    gender_pronoun: Option<String>,
    id:             Option<i64>,
    // images:         Option<Vec<Image>>
    // leagues:        Option<GGLeagues>
    // location:       Option<GGAddress>
    name:           Option<String>,
    // player:         Option<Player>
    slug:           Option<String>,
    tournaments:    Option<GGTournaments>,

}

impl GGUser {

    /// Returns the bio of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn bio(&self) -> String {
        let mut result: String = "".to_string();
        if self.bio.is_some() {
            result = self.bio.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the birthday of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn birthday(&self) -> String {
        let mut result: String = "".to_string();
        if self.birthday.is_some() {
            result = self.birthday.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the discriminator of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn discriminator(&self) -> String {
        let mut result: String = "".to_string();
        if self.discriminator.is_some() {
            result = self.discriminator.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the email of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn email(&self) -> String {
        let mut result: String = "".to_string();
        if self.email.is_some() {
            result = self.email.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the events the user has competed in.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn events(&self) -> Vec<GGEvent> {
        let mut result: Vec<GGEvent> = Vec::new();
        if self.events.is_some() {
            for event in &self.events.as_ref().unwrap().nodes {
                result.push(event.clone());
            }
        }
        return result;
    }

    /// Returns the gender pronoun of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn gender_pronoun(&self) -> String {
        let mut result: String = "".to_string();
        if self.gender_pronoun.is_some() {
            result = self.gender_pronoun.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the user.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the name of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the slug of the user.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.slug.is_some() {
            result = self.slug.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the tournaments the user has competed in.
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

}
