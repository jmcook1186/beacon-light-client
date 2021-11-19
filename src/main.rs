use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
mod build_objects;
mod light_client_types;
use eth2::types::*;
use eth2_hashing::{hash};
use std::sync::Arc;
extern crate hex;
use swap_or_not_shuffle::compute_shuffled_index;
use bytes::{BufMut, BytesMut};
use ssz::{ssz_encode, Decode, DecodeError, Encode};
use ssz_types::{typenum::Unsigned, BitVector, FixedVector};


fn main(){
    
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "finalized";
    let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope"); 
    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);

    // download beacon_state and make a snapshot
    let state = build_objects::get_state(&api_key, &state_id, &endpoint_prefix);
    let snapshot = build_objects::make_snapshot(&state);
    
    // download a beacon block and extract the body
    let block = build_objects::get_block(&api_key, &state_id, &endpoint_prefix);
    let body = block.message().body();
    let header = build_objects::get_header(&api_key, &state_id, &endpoint_prefix);

    //ssz serialize the state object
    let serialized_state = state.as_ssz_bytes();

    // NOW MERKLEIZE IT!!
        
}



//pub fn get_update(state: &BeaconState<MainnetEthSpec>, current_snapshot: &LightClientSnapshot, beacon_block_body: &serde_json::Value )->LightClientUpdate{


//     let new_header = state.latest_block_header;

//     let current_header = &current_snapshot.header;

//     // new sync committees from state object
//     let (current_sync_committee, next_sync_committee) = query_node::get_sync_committees(&state);

//     // new snapshot from new header and new sync comms
//     let snapshot = LightClientSnapshot{
//         header: new_header,
//         current_sync_committee: current_sync_committee,
//         next_sync_committee: next_sync_committee,
//     };

//     // get sync_aggregate from beacon block body
//     // parse to vector of u8s
//     let _sync_committee_bits = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_bits"].to_string();
//     let _trimmed = &_sync_committee_bits.replace("\"", "");
//     let sync_committee_bits: Vec<u8> = _trimmed.as_bytes().to_vec();


//     // THIS IS THE ROOT, BUT WE NEED THE MERKLE BRANCH CONNECTING IT TO BEACON STATE
//     let _finalized_branch = state["data"]["finalized_checkpoint"]["root"].to_string();
//     let _trimmed = &_finalized_branch.replace("\"", "");
//     let finalized_branch: Vec<u8> = _trimmed.as_bytes().to_vec();



//     // get sync committee signature 
//     let sync_committee_signature = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_signature"].to_string();

//     // other update vars from state obj
//     let branch = vec![0,1,2,3,4,5]; //PLACEHOLDER
//     let finality_header = current_header;
//     let finality_branch = finalized_branch;//PLACEHOLDER
//     let sync_committee_bits = sync_committee_bits;
//     let fork = state["data"]["fork"].to_string();
//     let sync_pubkeys = &snapshot.next_sync_committee.pubkeys.to_string();

//     // build update obj
//     let update =  LightClientUpdate{
//         header: snapshot.header,
//         next_sync_committee: snapshot.next_sync_committee,
//         next_sync_committee_branch: branch,
//         finality_header: finality_header,
//         finality_branch: finality_branch,
//         sync_committee_bits: sync_committee_bits,
//         sync_committee_signature: sync_committee_signature,
//         fork_version: fork,
//     };

//     return update

//}
