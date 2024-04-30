use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Player {
    nba_player: String,
    team: String,
    assists: f32,
    // defines a Player with name, team, and assists data
}

fn print_similar_assist_pairs(graph: &UnGraph<Player, &str>) {
    let mut seen_edges = std::collections::HashSet::new();

    for node_index in graph.node_indices() {
        let player = &graph[node_index];
        // First, collect all valid neighbor indices into a vector
        let neighbors: Vec<_> = graph.neighbors(node_index).filter(|&n| {
            graph.find_edge(node_index, n).map_or(false, |e| {
                *graph.edge_weight(e).unwrap() == "similar assists" && !seen_edges.contains(&(node_index, n))
            })
        }).collect();

        for neighbor_index in neighbors {
            let neighbor_player = &graph[neighbor_index];
            println!("{} ({}) and {} ({}) have similar assists.", 
                player.nba_player, player.team, neighbor_player.nba_player, neighbor_player.team);
            seen_edges.insert((node_index, neighbor_index));
            seen_edges.insert((neighbor_index, node_index));
        }
    } // it would the pairs and ensures each pair is processed only once
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = UnGraph::<Player, &str>::new_undirected();

    let players = vec![
        Player { nba_player: "James Harden".to_string(), team: "HOU".to_string(), assists: 11.2 },
        Player { nba_player: "J.J. Barea".to_string(), team: "DAL".to_string(), assists: 5.5 },
        Player { nba_player: "Nerlens Noel".to_string(), team: "DAL".to_string(), assists: 1.0 },
        Player { nba_player: "Dirk Nowitzki".to_string(), team: "DAL".to_string(), assists: 1.5 },
        Player { nba_player: "Dwight Powell".to_string(), team: "DAL".to_string(), assists: 0.6},
        Player { nba_player: "Sam Dekker".to_string(), team: "HOU".to_string(), assists: 1.0 },
        Player { nba_player: "Trevor Ariza".to_string(), team: "HOU".to_string(), assists: 2.2 },
        Player { nba_player: "Eric Gordon".to_string(), team: "HOU".to_string(), assists: 2.5 },
        Player { nba_player: "Reggie Bullock".to_string(), team: "DET".to_string(), assists: 0.9 },
        Player { nba_player: "Marcus Morris".to_string(), team: "DET".to_string(), assists: 2.0 },
        Player { nba_player: "Jamal Crawford".to_string(), team: "LAC".to_string(), assists: 2.6 },
        Player { nba_player: "Kentavious Caldwell-Pope".to_string(), team: "DET".to_string(), assists: 2.5 },
        Player { nba_player: "Henry Ellenson".to_string(), team: "DET".to_string(), assists: 0.4 },
        Player { nba_player: "Gerald Henderson".to_string(), team: "PHI".to_string(), assists: 1.6 },
        Player { nba_player: "Austin Rivers".to_string(), team: "LAC".to_string(), assists: 2.8 },
        Player { nba_player: "Nik Stauskas".to_string(), team: "PHI".to_string(), assists: 2.4 },
        Player { nba_player: "Troy Williams".to_string(), team: "HOU".to_string(), assists: 0.8 },
        Player { nba_player: "Stanley Johnson".to_string(), team: "DET".to_string(), assists: 1.4 },
        Player { nba_player: "DeAndre Liggins".to_string(), team: "DAL".to_string(), assists: 0.9 },
        Player { nba_player: "Paul Pierce".to_string(), team: "LAC".to_string(), assists: 0.4 }  

    ];

    let mut indices = Vec::new();
    for player in &players {
        indices.push(graph.add_node(player.clone()));
    }

    for i in 0..indices.len() {
        for j in (i + 1)..indices.len() {
            let team_similarity = graph[indices[i]].team == graph[indices[j]].team;
            let assists_difference = (graph[indices[i]].assists as f32 - graph[indices[j]].assists as f32).abs() <= 5.0;

            if team_similarity {
                graph.add_edge(indices[i], indices[j], "same team");
            }
            if assists_difference {
                graph.add_edge(indices[i], indices[j], "similar assists");
            }
        }
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    let mut file = File::create("graph.dot")?;
    writeln!(file, "{:?}", dot)?;
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    print_similar_assist_pairs(&graph);
    Ok(())
}

// command line "dot -Tpng graph.dot -o graph.png", it would generate a graph.png from graph.dot


#[test]
    fn test_graph_construction() { // test data
        let mut graph = UnGraph::<Player, &str>::new_undirected();
        let players = vec![
            Player { nba_player: "player A".to_string(), team: "Team X".to_string(), assists: 1.0 },
            Player { nba_player: "player B".to_string(), team: "Team Y".to_string(), assists: 1.5 },
            Player { nba_player: "player C".to_string(), team: "Team X".to_string(), assists: 2.0 },
        ]; // sample player for testing

        let mut indices = Vec::new();
        for player in &players {
            indices.push(graph.add_node(player.clone()));
        }

        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                let team_similarity = graph[indices[i]].team == graph[indices[j]].team;
                let assists_difference = (graph[indices[i]].assists - graph[indices[j]].assists).abs() <= 1.0;

                if team_similarity {
                    graph.add_edge(indices[i], indices[j], "same team");
                }
                if assists_difference {
                    graph.add_edge(indices[i], indices[j], "similar assists");
                }
            }
        }

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 4);
    }