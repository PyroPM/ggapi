
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    entrant::*,
    user::*,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GGParticipants {
    pub nodes: Vec<GGParticipant>,
}

#[allow(non_snake_case)]
#[derive(Clone, Serialize, Deserialize)]
pub struct GGParticipant {
    pub id: i64,
    pub gamerTag: String,
    pub prefix: Option<String>,
    pub user: Option<GGUser>,
    pub entrants: Option<Vec<GGEntrant>>,
}
