
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg TournamentLinks.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTournamentLinks {

    pub facebook:       Option<String>,
    pub discord:        Option<String>,

}

impl GGTournamentLinks {

    /// Returns the facebook link of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn facebook(&self) -> String {
        let mut result: String = "".to_string();
        if self.facebook.is_some() {
            result = self.facebook.clone().unwrap().clone();
        }
        return result;
    }
    
    /// Returns the discord link of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn discord(&self) -> String {
        let mut result: String = "".to_string();
        if self.discord.is_some() {
            result = self.discord.clone().unwrap().clone();
        }
        return result;
    }

}
