
//! Communicate with start.gg's API in a fast, type-safe manner with little hassle.

use gql_client::{
    Client,
    ClientConfig,
};

use std::collections::HashMap;
use serde::{
    Deserialize,
    Serialize,
};

pub mod user;
pub mod entrant;
pub mod participant;
pub mod event;
pub mod tournament;

//////////////////////////////////////////////////
// structures for start.gg schema
//////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct GGData {
    tournament: Option<tournament::GGTournament>,
}

impl GGData {

    /// Returns a tournament from a query.
    pub fn tournament(&self) -> &tournament::GGTournament {
        return self.tournament.as_ref().unwrap();
    }

}

#[derive(Serialize)]
pub struct Vars {
    pub slug: String,
    pub page: u32,
    pub per_page: u32,
}

//////////////////////////////////////////////////
// helper functions
//////////////////////////////////////////////////

async fn execute_query(
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

//////////////////////////////////////////////////
// general functions
//////////////////////////////////////////////////

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
            }
        }
      }      
    "#;

    
    /*let query = r#"
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
                entrants(query: {perPage: 500, page: 1}) {
                    pageInfo {
                        total
                        totalPages
                    }
                    nodes {
                        id
                    }
                }
            }
            participants(query: {perPage: 500, page: 1}) {
                nodes {
                    id
                    gamerTag
                    prefix
                    user {
                        discriminator
                        name
                    }
                    entrants {
                        id
                    }
                }
            }
        }
      }      
    "#;*/
    

    let vars = Vars { slug: slug.to_string(), page: 1, per_page: 100 };

    return execute_query(token, query, vars).await;
}

#[cfg(test)]
mod tests {
    use super::*;
}
