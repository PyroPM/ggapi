# ggapi
A library for communicating with [start.gg](https://start.gg/)'s API.

## Usage
```rust
let data = get_tournament_info(
    "evo-2023","INSERT_TOKEN_HERE"
).await;

println!("{}", data.tournament().name());
println!("{}", data.tournament().start_at().to_string());
println!("{}", data.tournament().slug());
println!("{}", data.tournament().short_slug());
```

## Notes
[start.gg](https://start.gg/) has a rate limit and additionally a limit to 1000 objects per query response. If you are working with a large tournament with a significant number of events and attendees, you may run into this limit.

- ggapi has been designed with the hope of avoiding this issue: queries are done in smaller, more broken up queries. Instead of executing a single, large query that gets all of the information about the tournament, the library will split the queries up to divide and conquer.
- For example, the get_tournament_info() function only returns basic information about a tournament (name, date, etc.). It does not get information about all of the participants, what games they entered, or any of the tournament standings. Those are achieved in separate queries, as it can not only reduce the amount of time required to execute a query but also the the amount of objects received to hopefully avoid the rate limit.
