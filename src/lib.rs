
//! Communicate with start.gg's API in a fast, type-safe manner with little hassle.

use gql_client::{
    Client,
    ClientConfig,
};

use std::collections::HashMap;
use serde::Serialize;

pub mod structs;

pub use structs::*;

/// Variables for a query.
#[derive(Serialize)]
pub struct Vars {
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
) -> GGData {

    let mut headers = HashMap::new();
    headers.insert("authorization".to_string(), format!("Bearer {}", token));

    let config = ClientConfig {
        endpoint: "https://api.start.gg/gql/alpha".to_string(),
        timeout: Some(60),
        headers: Some(headers),
        proxy: None,
    };

    let client = Client::new_with_config(config);
    let data = client.query_with_vars::<GGData, Vars>(query, vars).await.unwrap();

    return data.unwrap();
}

/// Get some basic tournament information.
///
/// Returns the tournament id, name, slug, short slug, as well as a list of events, phases within those events, and all of the phase groups in each phase.
pub async fn get_tournament_info(
    slug: &str,
    token: &str,
) -> GGData {

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

    let vars = Vars { slug: slug.to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

#[cfg(test)]
mod tests {
    use super::*;
}
