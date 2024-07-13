use graspologic_native::errors::NetworkError;
use graspologic_native::leiden::leiden;
use graspologic_native::network::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_csv(file_path: &str) -> Result<LabeledNetwork<String>, NetworkError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let source: String = parts[0].to_string();
            let target: String = parts[1].to_string();
            edges.push((source, target, 1.0));
        } else {
            eprintln!("Skipping malformed line: {}", line);
        }
    }

    let mut builder: LabeledNetworkBuilder<String> = LabeledNetworkBuilder::new();
    let labeled_network: LabeledNetwork<String> = builder.build(edges.into_iter(), true);

    Ok(labeled_network)
}

fn main() -> Result<(), NetworkError> {
    let network_path = "files/karate_club.csv";

    // Load the labeled network from the file
    let labeled_network = load_csv(network_path)?;
    // let labeled_network = load_xml(network_path)?;
    println!("Loaded network with {} nodes", labeled_network.num_nodes());

    // Get the compact network
    let compact_network = labeled_network.compact();
    eprintln!(
        "Loaded compact network with {} nodes",
        compact_network.num_nodes()
    );

    // Create a random number generator with a seed
    let seed = [1u8; 32]; // You can use any seed value
    let mut rng = StdRng::from_seed(seed);

    // Run the Leiden algorithm
    let (improved, clustering) =
        leiden(&compact_network, None, None, None, None, &mut rng, false).unwrap();

    // Check if the algorithm improved the clustering
    if improved {
        println!("Leiden algorithm improved the clustering.");
    } else {
        println!("Leiden algorithm did not improve the clustering.");
    }

    // Print the partitions
    let mut cluster_map = std::collections::HashMap::new();
    for (node, cluster) in clustering.into_iter().enumerate() {
        cluster_map
            .entry(cluster.cluster)
            .or_insert_with(Vec::new)
            .push(node);
    }

    for (cluster, nodes) in cluster_map {
        println!("Cluster {}: {:?}", cluster, nodes);
    }

    Ok(())
}
