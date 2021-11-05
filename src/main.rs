use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
mod query_node;
mod types;
mod build_objects;
use std::mem;
use std::option;




fn main(){
    
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope"); 
    let initial_snapshot = build_objects::initial_snapshot(&api_key, &node_id);

    // basic test print statements
    println!("{}", initial_snapshot.header.state_root.to_string());
    println!("{}", initial_snapshot.current_sync_committee.aggregate_pubkey.to_string());

    let initial_store = build_objects::initialize_store(initial_snapshot);


    let next_snapshot = build_objects::next_snapshot(initial_store, &api_key, &node_id);
    let update = query_node::get_update(&api_key, &node_id);

}

