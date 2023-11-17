
//! Enumerations used for serializing and deserializing data from the start.gg API.
//!
//! These enums are used to either aid in serializing / deserializing or are representations of existing enums in the start.gg API.

use serde::{
    Deserialize,
    Serialize,
};

use std::str::FromStr;

use crate::structs::query::*;

/// Equivalent for start.gg ID.
///
/// An ID is either a String or an i64, which is usually an i64.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum GGID {
    
    Int(i64),
    String(String),

}

impl FromStr for GGID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i64>() {
            Ok(value) => GGID::Int(value),
            Err(_) => GGID::String(s.to_string())
        })
    }
}

/// Enumeration to catch errors.
///
/// This enum will either be a proper response  or just a string containing an error.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GGResponse {
    
    Data(GGData),
    Error(String),

}
