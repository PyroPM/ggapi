
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
    page_info::*,
};

/// Equivalent for start.gg StationsConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStationsConnection {
    pub nodes:      Vec<GGStations>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGStationsConnection {

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

/// Equivalent for start.gg Stations.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGStations {
    
    #[serde(rename(serialize = "canAutoAssign", deserialize = "canAutoAssign"))]
    pub can_auto_assign:    Option<bool>,

    #[serde(rename(serialize = "clusterNumber", deserialize = "clusterNumber"))]
    pub cluster_number:     Option<String>,

    #[serde(rename(serialize = "clusterPrefix", deserialize = "clusterPrefix"))]
    pub cluster_prefix:     Option<i64>,
    pub enabled:            Option<bool>,
    pub identifier:         Option<i64>,
    pub id:                 Option<i64>,

    #[serde(rename(serialize = "numSetups",     deserialize = "numSetups"))]
    pub num_setups:         Option<i64>,
    pub number:             Option<i64>,
    pub prefix:             Option<String>,
    // pub queue:              Option<JSON>,

    #[serde(rename(serialize = "queueDepth",    deserialize = "queueDepth"))]
    pub queue_depth:        Option<i64>,
    pub state:              Option<i64>,
    
    #[serde(rename(serialize = "updatedAt",                 deserialize = "updatedAt"))]
    pub updated_at:         Option<i64>,

}

impl GGStations {

    /// Returns if the station can be auto assigned.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn can_auto_assign(&self) -> bool {
        let mut result: bool = false;
        if self.can_auto_assign.is_some() {
            result = self.can_auto_assign.unwrap().clone();
        }
        return result;
    }

    /// Returns the cluster number of the station.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn cluster_number(&self) -> String {
        let mut result: String = "".to_string();
        if self.cluster_number.is_some() {
            result = self.cluster_number.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the cluster prefix of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn cluster_prefix(&self) -> i64 {
        let mut result: i64 = 0;
        if self.cluster_prefix.is_some() {
            result = self.cluster_prefix.unwrap().clone();
        }
        return result;
    }

    /// Returns if the station is enabled.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn enabled(&self) -> bool {
        let mut result: bool = false;
        if self.enabled.is_some() {
            result = self.enabled.unwrap().clone();
        }
        return result;
    }

    /// Returns the identifier of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn identifier(&self) -> i64 {
        let mut result: i64 = 0;
        if self.identifier.is_some() {
            result = self.identifier.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns the number of setups in the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_setups(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_setups.is_some() {
            result = self.num_setups.unwrap().clone();
        }
        return result;
    }

    /// Returns the number of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn number(&self) -> i64 {
        let mut result: i64 = 0;
        if self.number.is_some() {
            result = self.number.unwrap().clone();
        }
        return result;
    }

    /// Returns the prefix of the station.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn prefix(&self) -> String {
        let mut result: String = "".to_string();
        if self.prefix.is_some() {
            result = self.prefix.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the queue depth of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn queue_depth(&self) -> i64 {
        let mut result: i64 = 0;
        if self.queue_depth.is_some() {
            result = self.queue_depth.unwrap().clone();
        }
        return result;
    }

    /// Returns the state of the station.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

    /// Returns the time the station was last updated.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn updated_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.updated_at.is_some() {
            result = self.updated_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

}
