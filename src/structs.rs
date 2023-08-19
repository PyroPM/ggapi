
//! Structures used for serializing and deserializing data structures from the start.gg API.

/// These structures are used internally to get a proper, type-safe output from GraphQL.
/// You are welcome to use these structures directly, however they are meant to be used only when returned from a query.

// pub mod action_set;
// pub use action_set::*;

pub mod address;
pub use address::*;

// pub mod bracket_config;
// pub use bracket_config::*;

pub mod character;
pub use character::*;

pub mod contact_info;
pub use contact_info::*;

pub mod entrant;
pub use entrant::*;

pub mod event;
pub use event::*;

pub mod event_owner;
pub use event_owner::*;

pub mod event_team;
pub use event_team::*;

pub mod event_tier;
pub use event_tier::*;

pub mod game;
pub use game::*;

pub mod game_selection;
pub use game_selection::*;

pub mod global_team;
pub use global_team::*;

pub mod image;
pub use image::*;

// pub mod league;
// pub use league::*;

// pub mod match_config;
// pub use match_config::*;

pub mod page_info;
pub use page_info::*;

pub mod participant;
pub use participant::*;

pub mod phase;
pub use phase::*;

pub mod phase_group;
pub use phase_group::*;

pub mod player;
pub use player::*;

pub mod player_rank;
pub use player_rank::*;

pub mod profile_authorization;
pub use profile_authorization::*;

pub mod progression;
pub use progression::*;

pub mod query;
pub use query::*;

pub mod race_bracket_config;
pub use race_bracket_config::*;

pub mod race_match_config;
pub use race_match_config::*;

pub mod round;
pub use round::*;

pub mod score;
pub use score::*;

pub mod seed;
pub use seed::*;

// pub mod set;
// pub mod set_slot;
// pub mod shop;
// pub mod shop_level;
// pub mod shop_order_message;

pub mod stage;
pub use stage::*;

// pub mod standing;
// pub mod standing_stats;
// pub mod stations;

pub mod stream;
pub use stream::*;

pub mod stream_queue;
pub use stream_queue::*;

pub mod streams;
pub use streams::*;

pub mod team;
pub use team::*;

pub mod team_action_set;
pub use team_action_set::*;

pub mod team_member;
pub use team_member::*;

pub mod team_roster_size;
pub use team_roster_size::*;

pub mod tournament;
pub use tournament::*;

pub mod tournament_links;
pub use tournament_links::*;

pub mod user;
pub use user::*;

pub mod videogame;
pub use videogame::*;

pub mod wave;
pub use wave::*;
