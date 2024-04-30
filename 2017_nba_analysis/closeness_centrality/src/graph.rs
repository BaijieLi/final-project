use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::{connected_components, dijkstra};
use petgraph::visit::IntoNodeIdentifiers;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Reverse, Ordering};
use super::model::Player;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Centrality(f64);

impl Eq for Centrality {}

impl Ord for Centrality {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

pub fn create_graph(players: &[Player]) -> UnGraph<String, &'static str> {
    let mut graph = UnGraph::new_undirected();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();
    // builds an undirected graph that nodes represent players. Nodes are connected by edges if the players are teammates

    for player in players {
        indices.entry(player.player_name.clone())
            .or_insert_with(|| graph.add_node(player.player_name.clone()));
    }

    let mut team_map: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    for player in players {
        let node = indices[&player.player_name];
        team_map.entry(player.team.clone())
            .or_default()
            .push(node);
    }

    for teammates in team_map.values() {
        for (i, &teammate1) in teammates.iter().enumerate() {
            for &teammate2 in &teammates[i + 1..] {
                graph.add_edge(teammate1, teammate2, "teammate");
            }
        }
    }
// creates an undirected graph in which nodes represent players, and edges represent teammates. It uses two HashMaps to manage node indices and team groupings efficiently.
    graph
}

pub fn find_connected_components(graph: &UnGraph<String, &'static str>) -> usize {
    connected_components(graph)
} //  use to count how many separate groups (nba teams) there are in the graph.

pub fn compute_closeness_centrality(graph: &UnGraph<String, &'static str>) -> (HashMap<NodeIndex, f64>, BinaryHeap<Reverse<(Centrality, NodeIndex)>>, BinaryHeap<(Centrality, NodeIndex)>) {
    let mut centrality = HashMap::new();
    let mut top_centrality = BinaryHeap::new();
    let mut bottom_centrality = BinaryHeap::new();
    let node_count = graph.node_count() as f64;

    for node in graph.node_identifiers() {
        let path_lengths = dijkstra(graph, node, None, |_| 1);
        let total_path_length: usize = path_lengths.values().map(|&d| d).sum();
        let closeness = if total_path_length > 0 {
            (node_count - 1.0) / total_path_length as f64
        } else {
            0.0
        };
        centrality.insert(node, closeness);
        top_centrality.push(Reverse((Centrality(closeness), node)));
        bottom_centrality.push((Centrality(closeness), node));

        if top_centrality.len() > 20 {
            top_centrality.pop();
        }
        if bottom_centrality.len() > 20 {
            bottom_centrality.pop();
        }
    }
// computes closeness centrality using Dijkstra's algorithm for all nodes. 
    (centrality, top_centrality, bottom_centrality)
}

pub fn print_top_centrality(graph: &UnGraph<String, &'static str>, top_centrality: BinaryHeap<Reverse<(Centrality, NodeIndex)>>, bottom_centrality: BinaryHeap<(Centrality, NodeIndex)>) {
    let top_five: Vec<(NodeIndex, f64)> = top_centrality.into_sorted_vec().iter().map(|x| (x.0 .1, x.0 .0 .0)).collect();
    let last_five: Vec<(NodeIndex, f64)> = bottom_centrality.into_sorted_vec().iter().map(|x| (x.1, x.0 .0)).collect();

    println!("Top 20 Closeness Centralities:");
    for (node, closeness) in top_five.iter().rev() {
        println!("Node {}: Closeness Centrality = {:.4}", graph[*node], closeness);
    }

    println!("Last 20 Closeness Centralities:");
    for (node, closeness) in last_five.iter() {
        println!("Node {}: Closeness Centrality = {:.4}", graph[*node], closeness);
    }
} // print the nodes with the highest and lowest closeness centralities

