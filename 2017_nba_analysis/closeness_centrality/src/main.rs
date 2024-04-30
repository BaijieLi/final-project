mod graph;
mod model;

use graph::{create_graph, find_connected_components, compute_closeness_centrality, print_top_centrality};
use model::read_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let players = read_data("nba.csv")?;
    let graph = create_graph(&players);

    let num_components = find_connected_components(&graph);
    println!("There are {} connected components", num_components);

    // Update here to handle the new third return value
    let (_centrality, top_centrality, bottom_centrality) = compute_closeness_centrality(&graph);
    
    // Pass the correct arguments to the function
    print_top_centrality(&graph, top_centrality, bottom_centrality);

    Ok(())
}


#[test]
    fn test_create_graph() { // test the node and edge of the graph
        use crate::model::Player;
        let players = vec![
            Player { player_name: "Player1".to_string(), team: "TeamA".to_string() },
            Player { player_name: "Player2".to_string(), team: "TeamA".to_string() },
            Player { player_name: "Player3".to_string(), team: "TeamB".to_string() },
        ];
        let graph = create_graph(&players);
        assert_eq!(graph.node_count(), 3); // Three edge between Player1 and Player2
        assert_eq!(graph.edge_count(), 1); // One edge between Player1 and Player2
    }