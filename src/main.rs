mod nhl_queries;

use reqwest::Client;
use std::env;

const NHL_BASE_URL: &str = "https://api.sportsdata.io/v3/nhl";
const TEAM: &str = "COL";
const OCP_HEADER: &str = "Ocp-Apim-Subscription-Key";

// use `tokio` as we will be dealing w/
// asynchronous tasks like API calls
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // DO NOT hard code or push secret keys to repositories
    let api_key = env::var("API_KEY")?;

    // build http client
    let client = Client::builder().build()?;

    // request player details
    let players = nhl_queries::get_players_by_team(TEAM, &api_key, &client).await?;
    for player in players.iter() {
        println!(
            "{} {}: {}",
            player.first_name, player.last_name, player.player_ID
        );
        let news = nhl_queries::get_news_by_player(player.player_ID, &api_key, &client).await?;
        if news.len() > 0 {
            println!("\tNews - {:?}", news[0].title);
            println!("\t{:?}\n", news[0].content);
        }

        // TODO: write data to SQLITE here
    }

    Ok(())
}
