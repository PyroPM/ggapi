# ggapi
A library for communicating with start.gg's API.

## Usage
```rs
let data = get_tournament_info(
    "evo","INSERT_TOKEN_HERE"
).await;

println!("{}", data.tournament().name());
println!("{}", data.tournament().start_at().to_string());
println!("{}", data.tournament().slug());
println!("{}", data.tournament().short_slug());
```
