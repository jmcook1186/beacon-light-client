use std::format;
use std::fs;
pub mod build_objects;
pub mod constants;
pub mod http_requests;
pub mod light_client_types;
pub mod node_discovery;
pub mod serialize_and_merkleize;
extern crate hex;
// use light_client_types::LightClientStore;

fn main() {
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "finalized";

    let api_key: String = fs::read_to_string(format!(
        "/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",
        node_number.to_string()
    ))
    .expect("Failed to connect to a Beacon Node");

    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);

    // download beacon_state and make a snapshot
    let state = build_objects::get_state(&api_key, &state_id, &endpoint_prefix);
    let snapshot = build_objects::make_snapshot(&state);

    // download a beacon block and extract the body
    let block = build_objects::get_block(&api_key, &state_id, &endpoint_prefix);
    let finality_header = build_objects::get_header(&api_key, &state_id, &endpoint_prefix); //must have state_id == "finalized"

    let serialized_state = serialize_and_merkleize::serialize_beacon_state(&state);

    // build update object
    //serialization, merkleization and branch extraction for beacon_state are in here
    //let update = build_objects::get_update(state, block, finality_header);

    //serialize_and_merkleize::serialize_beacon_state(&state);
    // let mut store = LightClientStore::create(snapshot);
    // store.valid_updates.push(update);
}
