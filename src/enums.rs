
//! Enumerations used for serializing and deserializing data from the start.gg API.
//!
//! These enums are used to either aid in serializing / deserializing or are representations of existing enums in the start.gg API.

use serde::{
    Deserialize,
    Serialize,
};

/// Equivalent for start.gg ID.
///
/// An ID is either a String or an i64, which is usually an i64.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GGID {
    
    Int(i64),
    String(String),

}
