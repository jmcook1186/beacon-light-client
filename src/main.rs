use std::format;
use std::fs;
pub mod build_objects;
pub mod constants;
pub mod http_requests;
pub mod light_client_types;
pub mod merkleize;
pub mod node_discovery;
pub mod serialize;
extern crate hex;
// use light_client_types::LightClientStore;

fn main() {
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "finalized";
    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);
    let api_key: String = fs::read_to_string(format!(
        "/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",
        node_number.to_string()
    ))
    .expect("Failed to connect to a Beacon Node");

    // download beacon_state and make a snapshot
    let state = build_objects::get_state(&api_key, &state_id, &endpoint_prefix);
    let _snapshot = build_objects::make_snapshot(&state);

    // download a beacon block and extract the body
    let block = build_objects::get_block(&api_key, &state_id, &endpoint_prefix);

    // ssz serialize the state object, pad and hash each field, build merkle tree
    let (serialized_state, sizes, offsets) = serialize::serialize_beacon_state(&state);
    let chunks = merkleize::generate_chunks(&serialized_state, &sizes, &offsets);
    let tree = merkleize::build_tree(chunks);

    println!("DOWNLOADED STATE ROOT: {:?}\n", block.state_root());
}
