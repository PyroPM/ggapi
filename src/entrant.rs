
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GGEntrants {
    pub nodes: Vec<GGEntrant>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GGEntrant {
    pub id: i64,
}
