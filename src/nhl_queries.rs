use reqwest::Client;
use serde::Deserialize;

const PLAYERS_ENDPOINT: &str = "/scores/json/PlayersBasic/";
const PLAYERS_NEWS_ENDPOINT: &str = "/scores/json/NewsByPlayerID/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct Player {
    pub player_ID: usize,
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_players_by_team(
    team: &str,
    api_key: &str,
    client: &Client,
) -> Result<Vec<Player>, reqwest::Error> {
    let response = client
        .get(format!("{}{}{team}", crate::NHL_BASE_URL, PLAYERS_ENDPOINT))
        .header(crate::OCP_HEADER, api_key)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => response.json::<Vec<Player>>().await,
        _ => Err(response.error_for_status_ref().err().unwrap()),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct News {
    pub title: String,
    pub content: String,
}
pub async fn get_news_by_player(
    player_id: usize,
    api_key: &str,
    client: &Client,
) -> Result<Vec<News>, reqwest::Error> {
    // api doesn't like Jack Johsnons for some reason
    if player_id == 30000693 {
        return Ok(vec![]);
    }

    let response = client
        .get(format!(
            "{}{}{player_id}",
            crate::NHL_BASE_URL,
            PLAYERS_NEWS_ENDPOINT
        ))
        .header(crate::OCP_HEADER, api_key)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => response.json::<Vec<News>>().await,
        _ => Err(response.error_for_status_ref().err().unwrap()),
    }
}
