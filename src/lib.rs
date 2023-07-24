

use gql_client::Client;

use std::collections::HashMap;
use serde::{
    Deserialize,
    Serialize,
};

//////////////////////////////////////////////////
// structures for start.gg schema
//////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct GGData {
    pub tournament: GGTournament,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GGTournament {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub shortSlug: Option<String>,
    pub startAt: i64,
    pub events: Vec<GGEvent>,
    pub participants: GGParticipants,
}

#[derive(Serialize, Deserialize)]
pub struct GGEvent {
    pub id: i64,
    pub name: String,
    pub entrants: GGEntrants,
}

#[derive(Serialize, Deserialize)]
pub struct GGParticipants {
    pub nodes: Vec<GGParticipant>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GGParticipant {
    pub id: i64,
    pub gamerTag: String,
    pub prefix: Option<String>,
    pub user: GGUser,
    pub entrants: Option<Vec<GGEntrant>>,
}

#[derive(Serialize, Deserialize)]
pub struct GGEntrants {
    pub nodes: Vec<GGEntrant>,
}

#[derive(Serialize, Deserialize)]
pub struct GGEntrant {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GGUser {
    pub discriminator: String,
    pub name: Option<String>,
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

pub async fn get_tournament_info(
    slug: String,
    token: String,
) -> GGData {

    let endpoint = "https://api.start.gg/gql/alpha";
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
    "#;

    let mut headers = HashMap::new();
    headers.insert("authorization", format!("Bearer {}", token));

    let client = Client::new_with_headers(endpoint, headers);
    let vars = Vars { slug: slug.to_string(), page: 1, per_page: 100 };
    let data = client.query_with_vars::<GGData, Vars>(query, vars).await.unwrap();

    return data.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
