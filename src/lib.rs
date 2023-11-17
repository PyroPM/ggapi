
//! Communicate with start.gg's API in a fast, type-safe manner with little hassle.

use gql_client::{
    Client,
    ClientConfig,
};

use std::collections::HashMap;
use serde::Serialize;

pub mod enums;
pub use enums::*;

pub mod structs;
pub use structs::*;

/// Variables for a query.
#[derive(Serialize)]
pub struct Vars {
    pub id: GGID,
    pub slug: String,
    pub page: u32,
    pub per_page: u32,
}

/// Execute a query.
///
/// When given a token, query, and a set of variables, this function will execute a query and return a deserialized object.
pub async fn execute_query(
    token: &str,
    query: &str,
    vars: Vars,
) -> GGResponse {

    let mut headers = HashMap::new();
    headers.insert("authorization".to_string(), format!("Bearer {}", token));

    let config = ClientConfig {
        endpoint: "https://api.start.gg/gql/alpha".to_string(),
        timeout: Some(60),
        headers: Some(headers),
        proxy: None,
    };

    let client = Client::new_with_config(config);
    let data = client.query_with_vars::<GGResponse, Vars>(query, vars).await.unwrap();

    return data.unwrap();
}

/// Get some basic tournament information.
///
/// Returns the tournament id, name, slug, short slug, as well as a list of events, phases within those events, and all of the phase groups in each phase.
pub async fn get_tournament_info(
    slug: &str,
    token: &str,
) -> GGResponse {

    let query = r#"
    query GetTournamentInfo($slug: String!) {
        tournament(slug: $slug) {
            id
            name
            slug
            shortSlug
            startAt
            events {
                id
                name
                phases {
                    id
                    name
                    phaseGroups(query: { page: 1, perPage: 100 }) {
                        nodes {
                            id
                            displayIdentifier
                        }
                    }
                }
                slug
            }
        }
    }
    "#;

    let vars = Vars { id: GGID::Int(0), slug: slug.to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get events from a tournament.
///
/// Returns a list of events within a tournament.
pub async fn get_events_from_tournament(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query GetEvents($id: ID!) {
        tournament(id: $id) {
            id
            events {
                id
                name
                slug
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get phases from an event.
///
/// Returns a list of phases within a event.
pub async fn get_phases_from_event(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query GetPhases($id: ID!) {
        event(id: $id) {
            phases {
                id
                name
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get phase groups from a phase.
///
/// Returns a list of phase groups within a phase.
pub async fn get_phase_groups_from_phase(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query GetPhaseGroups($id: ID!) {
        phase(id: $id) {
            phaseGroups(query: { page: 1, perPage: 100 }) {
                nodes {
                    id
                    displayIdentifier
                }
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get all of the sets in a given phase group.
///
/// Returns a list of sets including the set name and entrants.
pub async fn get_sets_from_phase_group(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query PhaseGroupSets($id: ID!){
        phaseGroup(id: $id){
            sets(page: 1, perPage: 100, sortType: STANDARD) {
                nodes {
                    id
                    fullRoundText
                    identifier
                    slots {
                        entrant {
                            id
                            name
                        }
                    }
                }
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get specific set and various information about it.
///
/// Returns a set including the set name, entrants, and all of the participant information.
pub async fn get_entrants_from_set(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query SetEntrants($id: ID!){
        set(id: $id){
            id
            event {
                name
            }
            fullRoundText
            identifier
            slots {
                standing {
                    stats {
                        score {
                            label
                            value
                        }
                    }
                }
                entrant {
                    id
                    name
                    participants {
                        id
                        gamerTag
                        user {
                            discriminator
                            name
                        }
                    }
                }
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

/// Get information about a specific entrant.
///
/// Returns various important information regarding an entrant, including the name, tag and discriminator of each participant.
pub async fn get_entrant_info(
    id: GGID,
    token: &str,
) -> GGResponse {

    let query = r#"
    query EntrantInfo($id: ID!) {
        entrant(id: $id) {
            id
            name
            participants {
                id
                gamerTag
                user {
                    discriminator
                    name
                }
            }
        }
    }
    "#;

    let vars = Vars { id: id, slug: "".to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

#[cfg(test)]
mod tests {
    use super::*;
}
