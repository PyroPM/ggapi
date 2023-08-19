
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg Streams.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStreams {

    pub enabled:            Option<bool>,

    #[serde(rename(serialize = "followerCount",     deserialize = "followerCount"))]
    pub follower_count:     Option<i64>,
    pub id:                 Option<i64>,

    #[serde(rename(serialize = "isOnline",          deserialize = "isOnline"))]
    pub is_online:          Option<bool>,

    #[serde(rename(serialize = "numSetups",         deserialize = "numSetups"))]
    pub num_setups:         Option<i64>,

    #[serde(rename(serialize = "parentStreamId",    deserialize = "parentStreamId"))]
    pub parent_stream_id:   Option<i64>,

    #[serde(rename(serialize = "streamGame",        deserialize = "streamGame"))]
    pub stream_game:        Option<String>,

    #[serde(rename(serialize = "streamId",          deserialize = "streamId"))]
    pub stream_id:          Option<String>,

    #[serde(rename(serialize = "streamLogo",        deserialize = "streamLogo"))]
    pub stream_logo:        Option<String>,

    #[serde(rename(serialize = "streamName",        deserialize = "streamName"))]
    pub stream_name:        Option<String>,

    #[serde(rename(serialize = "streamSource",      deserialize = "streamSource"))]
    pub stream_source:      Option<i64>,

    #[serde(rename(serialize = "streamStatus",      deserialize = "streamStatus"))]
    pub stream_status:      Option<String>,

    #[serde(rename(serialize = "streamType",        deserialize = "streamType"))]
    pub stream_type:        Option<i64>,

    #[serde(rename(serialize = "streamTypeId",      deserialize = "streamTypeId"))]
    pub stream_type_id:     Option<i64>,

}

impl GGStreams {

    /// Returns if the streams are enabled.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn enabled(&self) -> bool {
        let mut result: bool = false;
        if self.enabled.is_some() {
            result = self.enabled.unwrap().clone();
        }
        return result;
    }

    /// Returns the follower count of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn follower_count(&self) -> i64 {
        let mut result: i64 = 0;
        if self.follower_count.is_some() {
            result = self.follower_count.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the streams is online.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn is_online(&self) -> bool {
        let mut result: bool = false;
        if self.is_online.is_some() {
            result = self.is_online.unwrap().clone();
        }
        return result;
    }

    /// Returns the number of setups on the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_setups(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_setups.is_some() {
            result = self.num_setups.unwrap().clone();
        }
        return result;
    }

    /// Returns the parent stream id of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn parent_stream_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.parent_stream_id.is_some() {
            result = self.parent_stream_id.unwrap().clone();
        }
        return result;
    }

    /// Returns the stream game of the streams.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn stream_game(&self) -> String {
        let mut result: String = "".to_string();
        if self.stream_game.is_some() {
            result = self.stream_game.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream id of the streams.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn stream_id(&self) -> String {
        let mut result: String = "".to_string();
        if self.stream_id.is_some() {
            result = self.stream_id.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream logo of the streams.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn stream_logo(&self) -> String {
        let mut result: String = "".to_string();
        if self.stream_logo.is_some() {
            result = self.stream_logo.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream name of the streams.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn stream_name(&self) -> String {
        let mut result: String = "".to_string();
        if self.stream_name.is_some() {
            result = self.stream_name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream source of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn stream_source(&self) -> i64 {
        let mut result: i64 = 0;
        if self.stream_source.is_some() {
            result = self.stream_source.unwrap().clone();
        }
        return result;
    }

    /// Returns the stream status of the streams.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn stream_status(&self) -> String {
        let mut result: String = "".to_string();
        if self.stream_status.is_some() {
            result = self.stream_status.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the stream type of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn stream_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.stream_type.is_some() {
            result = self.stream_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the stream type id of the streams.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn stream_type_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.stream_type_id.is_some() {
            result = self.stream_type_id.unwrap().clone();
        }
        return result;
    }

}
