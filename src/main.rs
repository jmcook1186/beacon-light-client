use eth2::types::BlockHeaderData;
use std::format;
pub mod build_objects;
pub mod constants;
pub mod http_requests;
pub mod light_client_types;
use light_client_types::{LightClientSnapshot, LightClientUpdate};
pub mod merkle_proofs;
pub mod merkleize;
pub mod node_discovery;
pub mod serialize;
extern crate hex;
// use light_client_types::LightClientStore;

fn main() {
    // set basic vars and get api key from secret
    let (node_id, _node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "head";
    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);

    // download beacon_state and make a snapshot
    let state = build_objects::get_state(&state_id, &endpoint_prefix);
    let _snapshot = build_objects::make_snapshot(&state);

    // download a beacon block and extract the body
    let block = build_objects::get_block(&state_id, &endpoint_prefix);
    let header: BlockHeaderData = build_objects::get_header(&state_id, &endpoint_prefix);

    let update: LightClientUpdate = build_objects::build_update(state, block, header);

    // for i in update.finality_branch.iter(){
    //     println!("{:?}", hex::encode(&i));
    // }
}
