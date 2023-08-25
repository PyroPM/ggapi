
use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg PageInfo.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGPageInfo {

    pub total:          Option<i64>,

    #[serde(rename(serialize = "totalPages",    deserialize = "totalPages"))]
    pub total_pages:    Option<i64>,
    pub page:           Option<i64>,

    #[serde(rename(serialize = "perPage",       deserialize = "perPage"))]
    pub per_page:       Option<i64>,

    #[serde(rename(serialize = "sortBy",        deserialize = "sortBy"))]
    pub sort_by:        Option<String>,
    // pub filter:         Option<JSON>,

}

impl GGPageInfo {

    /// Returns the total of the page info.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn total(&self) -> i64 {
        let mut result: i64 = 0;
        if self.total.is_some() {
            result = self.total.unwrap().clone();
        }
        return result;
    }

    /// Returns the total pages of the page info.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn total_pages(&self) -> i64 {
        let mut result: i64 = 0;
        if self.total_pages.is_some() {
            result = self.total_pages.unwrap().clone();
        }
        return result;
    }

    /// Returns the page of the page info.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn page(&self) -> i64 {
        let mut result: i64 = 0;
        if self.page.is_some() {
            result = self.page.unwrap().clone();
        }
        return result;
    }

    /// Returns the number per page of the page info.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn per_page(&self) -> i64 {
        let mut result: i64 = 0;
        if self.per_page.is_some() {
            result = self.per_page.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the sort by of the stream.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn sort_by(&self) -> String {
        let mut result: String = "".to_string();
        if self.sort_by.is_some() {
            result = self.sort_by.clone().unwrap().clone();
        }
        return result;
    }

}
