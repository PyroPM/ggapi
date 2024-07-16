# ggapi
A library for communicating with [start.gg](https://start.gg/)'s API.

## Usage
You can use helper functions to get values off of start.gg:
```rust
let data = ggapi::get_tournament_info(
    "evo-2023","INSERT_TOKEN_HERE"
).await;

match data {
    ggapi::GGResponse::Data(data) => {
        println!("{}", data.tournament().name());
        println!("{}", data.tournament().start_at().to_string());
        println!("{}", data.tournament().slug());
        println!("{}", data.tournament().short_slug());
    }
    ggapi::GGResponse::Error(data) => {
        println!("ggapi error: {}", data);
    }
}
```
Each helper function will get a small specific set of values instead of a large query. See the notes below to see the reasoning why.

These helper functions are what are some of the most common uses of the API and should be able to cope with even the largest tournaments effectively.

If these helper functions aren't enough, you can execute a query directly like so:
```rust

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
let data = execute_query(&token, &query, vars).await;

println!("{}", data.tournament().name());
println!("{}", data.tournament().start_at().to_string());
println!("{}", data.tournament().slug());
println!("{}", data.tournament().short_slug());
```
This example does the same as the helper function get_tournament_info(), but it lets you customize the query to your liking.
> When using execute_query() directly like this, you are not guaranteed to get a safe value back. Aside from potential errors, you are likely to hit the 1000 object limit if you are working with a large tournament, so try and use the helper functions whenever possible!

## Notes
- [start.gg](https://start.gg/) has a rate limit and additionally a limit to 1000 objects per query response. If you are working with a large tournament with a significant number of events and attendees, you may run into this limit.
    - ggapi has been designed with the hope of avoiding this issue: queries are done in smaller, more broken up queries. Instead of executing a single, large query that gets all of the information about the tournament, the library will split the queries up to divide and conquer.
    - For example, the get_tournament_info() function only returns basic information about a tournament (name, date, etc.). It does not get information about all of the participants, what games they entered, or any of the tournament standings. Those are achieved in separate queries, as it can not only reduce the amount of time required to execute a query but also the the amount of objects received to hopefully avoid the rate limit.

## Contributing
If you want to help contribute to the project, pull requests are encouraged! Most of the aspects of this are relatively simple, but any amount of help is appreciated! Just try and follow the same style of the existing files, the convention is there and (mostly) consistent across the entire library.

## Todo
- Complete remaining structures missing from the library
    - action_set (ActionSet)
    - bracket_config (BracketConfig)
    - league (League, LeagueConnection)
    - match_config (MatchConfig)
    - set (Set, SetConnection)
    - set_slot (SetSlot)
    - shop (Shop)
    - shop_level (ShopLevel, ShopLevelConnection)
    - shop_order_message (ShopOrderMessage, ShopOrderMesageConnection)
    - standing (Standing, StandingConnection)
    - standing_stats (StandingStats)
    - stations (Stations, StationsConnection)
    - wave (Wave)
- Finish existing structure implementations by adding missing types
    - Some types that were made first had a few missing values while the rest of the structures were being implemented. Those were commented out in each structure where they would be placed, but have not yet been completed implemented. Once every structure is implemented above, complete a sweep of each structure and fill out the rest of the structure.
- Implement enums for start.gg types
    - activity_state (ActivityState)
    - authorization_type (AuthorizationType)
    - bracket_type (BracketType)
    - comparator (Comparator)
    - game_selection_type (GameSelectionType)
    - match_config_verification_method (MatchConfigVerificationMethod)
    - race_limit_mode (RaceLimitMode)
    - race_type (RaceType)
    - set_sort_type (SetSortType)
    - social_connection_type (SocialConnectionType)
    - stream_source (StreamSource)
    - stream_type (StreamType)
    - team_member_status (TeamMemberStatus)
    - team_member_type (TeamMemberType)
    - tournament_pagination_sort (TournamentPaginationSort)
- Figure out how to deal with JSON types
- Error Handling
    - Figure out how to deal with queries that are too large
- Unplanned
    - Mutations?
    - Input Objects?
