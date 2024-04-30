use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;


// Define a structure to hold player data
struct Player {
    player_name: String,
    salary_millions: u32,
    twitter_favorite_count: u32,
}

fn read_player_data<P: AsRef<Path>>(path: P) -> Result<Vec<Player>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut players = Vec::new(); // initialize an empty vector to store the player data

    for line in reader.lines().skip(1) { // Skip the header line
        let line = line?; // read each line, avoid potential errors
        let parts: Vec<&str> = line.split(',').collect(); // split each line by commas
        let player = Player {
            player_name: parts[0].to_string(), // Assuming the column of PLAYER is at index 0
            salary_millions: parts[1].parse().unwrap_or_default(), // Assuming the column SALARY_MILLIONS is at index 1
            twitter_favorite_count: parts[2].parse().unwrap_or_default(), // Assuming the column of TWITTER_FAVORITE_COUNT is at index 2
        };
        players.push(player);
    }

    Ok(players)
}

// calculate Euclidean distance between two players in the csv
fn euclidean_distance(a: &Player, b: &Player) -> f64 {
    let salary_diff = a.salary_millions as i64 - b.salary_millions as i64;
    let twitter_diff = a.twitter_favorite_count as i64 - b.twitter_favorite_count as i64;
    ((salary_diff.pow(2) + twitter_diff.pow(2)) as f64).sqrt()
}

// use main function to calculate average distance
fn main() -> Result<()> {
    let players = read_player_data("nba.csv")?;

    let mut total_distance = 0.0;
    let mut count = 0;
    // used to find the maximum distance between two players
    let mut min_distance = f64::MAX; // Initialize with maximum float value
    // used to find the minimum distance between two players
    let mut max_distance = f64::MIN; // Initialize with minimum float value
    let mut min_pair = ("", "");
    let mut max_pair = ("", "");

    for i in 0..players.len() {
        for j in i + 1..players.len() {
            let distance = euclidean_distance(&players[i], &players[j]);
            println!("Distance between {} and {} is {}", players[i].player_name, players[j].player_name, distance);

            // Update the total distance, count, and check for min/max distances
            total_distance += distance;
            count += 1;

            if distance < min_distance {
                min_distance = distance;
                min_pair = (&players[i].player_name, &players[j].player_name);
            }
            if distance > max_distance {
                max_distance = distance;
                max_pair = (&players[i].player_name, &players[j].player_name);
            }
        }
    }

    let average_distance = total_distance / count as f64;
    println!("The average euclidean distance is : {}", average_distance);
    println!("The shortest distance of two players is between {} and {} at {}", min_pair.0, min_pair.1, min_distance);
    println!("The longest distance of two players is between {} and {} at {}", max_pair.0, max_pair.1, max_distance);
    
    Ok(())

}


#[test]
fn test_euclidean_distance() {
        let player1 = Player {
            player_name: "player one".to_string(),
            salary_millions: 10,
            twitter_favorite_count: 200,
        };
        let player2 = Player {
            player_name: "player two".to_string(),
            salary_millions: 30,
            twitter_favorite_count: 500,
        };
        
        let expected_distance = ((20_i64.pow(2) + 300_i64.pow(2)) as f64).sqrt(); // calculate the expected distance for the set
        assert_eq!(euclidean_distance(&player1, &player2), expected_distance);
    }

