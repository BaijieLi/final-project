use serde::Deserialize;
use std::error::Error;
use csv::Reader;

#[derive(Debug, Deserialize)]
pub struct Player {
    #[serde(rename = "PLAYER")]
    pub player_name: String,
    #[serde(rename = "TEAM_pie")]
    pub team: String,
} // map columns to struct fields using serde

pub fn read_data(path: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut players = Vec::new();
    for result in rdr.deserialize() {
        let player: Player = result?;
        players.push(player);
    }
    Ok(players)
}
// read all the player data from a "nba.csv" file into a vector of Player structures.

