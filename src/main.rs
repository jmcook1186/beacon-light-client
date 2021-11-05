use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
mod query_node;
mod types;
mod build_objects;
use std::mem;



//fn initialize_store(node_id: &String)-> LightClientStore{
    
    // // initialize store will always want the finalized block
    // // as this is the trusted start point for building store
    // let state_id: String = "finalized".to_string();
    // let next_state_id: String = "justified".to_string();

       
    // // get current header and committee info from finalized block
    // let current_block_header = http_requests::get_block_header(&node_id, &state_id);
    // let (current_sync_committee, current_validator_aggregates) = http_requests::get_sync_committee(&node_id, &state_id);
    // let (next_sync_committee, next_validator_aggregates) = http_requests::get_sync_committee(&node_id, &next_state_id);

    // // get slot number of finalized block - parse to int to enable increment
    // let mut _current_slot: Value = serde_json::from_str(&current_block_header).unwrap();
    // let _current_slot = &_current_slot["data"]["header"]["message"]["slot"].to_string();
    // let current_slot: &str = serde_json::from_str(&_current_slot).unwrap();

//     // increment current slot and assign to next_slot
//     let mut _next_slot: i32 = current_slot.parse::<i32>().unwrap();
//     _next_slot = _next_slot+1;
//     let next_slot: String = _next_slot.to_string();

//     let _new_block_header = http_requests::get_block_header(&node_id, &next_slot);

    // let _snapshot = 
    //   LightClientSnapshot{
    //     header: current_block_header,
    //     current_sync_committee: current_sync_committee,
    //     next_sync_committee: next_sync_committee
    //   };

    // let store = LightClientStore{
    //     snapshot: _snapshot,
    //     valid_updates: None
    //   };

   // return store;

//}




fn main(){
    
    for i in ["finalized"]{

    // set basic vars and get api key from secret
        let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
        let state_id = String::from(i);
        let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope");

        println!("api key = {}",&api_key.to_string());

        // let validator_ids: Vec<u8> = query_node::get_sync_committee_ids(&api_key, &node_id, &state_id);
        // let sync_committee_pubkeys: Vec<Vec<u8>> = query_node::get_sync_committee_pubkeys(&api_key, &node_id, &state_id, validator_ids);
        let block_header = query_node::get_block_header(&api_key, &node_id, &state_id);
        
        // println!("{}",sync_committee_pubkeys[0].len());
        // println!("{}",block_header);
        let (current_sync_committee, next_sync_committee) = 
          query_node::get_sync_committees(&api_key, &node_id, &state_id);

       
        let snapshot = build_objects::get_snapshot(block_header, current_sync_committee, next_sync_committee);
        
        // basic test print statements
        println!("{}", snapshot.header.state_root.to_string());
        println!("{}", snapshot.current_sync_committee.aggregate_pubkey.to_string());
    }
    
}

