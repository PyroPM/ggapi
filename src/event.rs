
use serde::{
    Deserialize,
    Serialize,
};

use crate::entrant::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct GGEvents {
    pub nodes: Vec<GGEvent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GGEvent {
    pub id: i64,
    pub name: String,
    pub entrants: Option<GGEntrants>,
}
