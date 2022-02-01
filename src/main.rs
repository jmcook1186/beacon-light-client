use std::format;
use std::fs;
pub mod build_objects;
pub mod constants;
pub mod http_requests;
pub mod light_client_types;
pub mod merkleize;
pub mod node_discovery;
pub mod serialize;
pub mod merkle_proofs;
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
    let state = build_objects::get_state(&state_id, &endpoint_prefix);
    let _snapshot = build_objects::make_snapshot(&state);

    // download a beacon block and extract the body
    let block = build_objects::get_block(&state_id, &endpoint_prefix);

    // ssz serialize the state object, pad and hash each field, build merkle tree
    let (serialized_state, sizes, offsets) = serialize::serialize_beacon_state(&state);
    let chunks = merkleize::generate_chunks(&serialized_state, &sizes, &offsets);
    let tree: Vec<Vec<u8>> = merkleize::merkle_tree(chunks);
    
    let sync_comm_branch: Vec<Vec<u8>> = merkle_proofs::get_branch(&tree, constants::NEXT_SYNC_COMMITTEE_INDEX);
    assert_eq!(sync_comm_branch.len() as u64, constants::NEXT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2);

    // let finalized_root_branch: Vec<Vec<u8>> =merkle_proofs::get_branch(&tree, constants::FINALIZED_ROOT_INDEX);
    // assert_eq!(finalized_root_branch.len() as u64, constants::FINALIZED_ROOT_INDEX_FLOOR_LOG2);

    println!("nodes in sync_comm_branch:\n");
    for i in sync_comm_branch.iter(){
        println!("{:?}", hex::encode(&i));
    }

    println!("\nCALCULATED STATE_ROOT: 0x{:?}\n", hex::encode(&sync_comm_branch[0]));
    println!("DOWNLOADED STATE ROOT: {:?}\n", block.state_root());

    // println!("predicted next sync comm root {:?}", hex::encode(&tree[55]));
}
