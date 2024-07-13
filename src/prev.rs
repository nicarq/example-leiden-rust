// use graspologic_native::errors::NetworkError;
// use graspologic_native::leiden::leiden;
// use graspologic_native::network::prelude::*;
// use petgraph::dot::{Config, Dot};
// use petgraph::Graph;
// use rand::rngs::StdRng;
// use rand::SeedableRng;

// // Wrapper type around CompactNetwork
// struct CompactNetworkWrapper<'a> {
//     network: &'a CompactNetwork,
// }

// impl<'a> CompactNetworkWrapper<'a> {
//     fn to_petgraph(&self) -> Graph<(), ()> {
//         let mut graph = Graph::new();
//         for _ in 0..self.network.num_nodes() {
//             graph.add_node(());
//         }
//         for node_id in 0..self.network.num_nodes() {
//             for neighbor in self.network.neighbors_for(node_id) {
//                 graph.add_edge(
//                     petgraph::graph::NodeIndex::new(node_id),
//                     petgraph::graph::NodeIndex::new(neighbor.id),
//                     (),
//                 );
//             }
//         }
//         graph
//     }
// }


// fn main() -> Result<(), NetworkError> {
//     let org_network_path = "files/simple_org_graph.csv";
//     let broken_network_path = "files/broken_org_graph.csv";

//     // Load the labeled network from the file
//     let labeled_network: LabeledNetwork<String> =
//         LabeledNetwork::<String>::load_from(org_network_path, ",", 0, 1, Some(2), false, true)?;
//     println!("Loaded network with {} nodes", labeled_network.num_nodes());

//     // Get the compact network
//     let compact_network = labeled_network.compact();
//     eprintln!("Loaded compact network with {} nodes", compact_network.num_nodes());

//     // Wrap the compact network
//     let compact_network_wrapper = CompactNetworkWrapper { network: &compact_network };

//     // Convert the compact network to a petgraph graph
//     let petgraph_network = compact_network_wrapper.to_petgraph();

//     // Print the graph in DOT format
//     println!("{:?}", Dot::with_config(&petgraph_network, &[Config::EdgeNoLabel]));

//     // // Create a random number generator with a seed
//     // let seed = [0u8; 32]; // You can use any seed value
//     // let mut rng = StdRng::from_seed(seed);

//     // // Run the Leiden algorithm
//     // let (_improved, clustering) =
//     //     leiden(compact_network, None, None, None, None, &mut rng, false).unwrap();

//     // // Print the partitions
//     // for (node, cluster) in clustering.into_iter().enumerate() {
//     //     println!("Node {} is in cluster {}", node, cluster.cluster);
//     // }

//     // // Attempt to load a broken network file
//     // let result: Result<LabeledNetwork<String>, NetworkError> =
//     //     LabeledNetwork::<String>::load_from(broken_network_path, ",", 0, 1, Some(2), false, true);
//     // eprintln!("{:?}", result);

//     // match result {
//     //     Ok(_) => println!("Unexpectedly loaded a broken network file successfully"),
//     //     Err(NetworkError::EdgeFileFormatError) => {
//     //         println!("Correctly identified broken network file format")
//     //     }
//     //     Err(err) => println!("Unexpected error: {:?}", err),
//     // }

//     Ok(())
// }
