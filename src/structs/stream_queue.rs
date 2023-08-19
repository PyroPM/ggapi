
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    streams::*,
};

/// Equivalent for start.gg StreamQueue.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see entrants()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStreamQueue {
    
    pub id:         Option<String>,
    // pub sets:       Option<Vec<GGSet>>,
    pub stream:     Option<Box<GGStreams>>,
    
}

impl GGStreamQueue {

    /// Returns the id of the stream queue.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn id(&self) -> String {
        let mut result: String = "".to_string();
        if self.id.is_some() {
            result = self.id.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream of the stream queue.
    ///
    /// Returns an empty tournament if not set or wasn't queried.
    pub fn stream(&self) -> GGStreams {
        let mut result: GGStreams = Default::default();
        if self.stream.is_some() {
            result = *self.stream.as_ref().unwrap().clone();
        }
        return result;
    }

}
