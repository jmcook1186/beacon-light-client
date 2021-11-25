use std::format;
use std::fs;
pub mod node_discovery;
pub mod http_requests;
pub mod build_objects;
pub mod light_client_types;
pub mod serialize_and_merkleize;
pub mod constants;
use crate::light_client_types::{LightClientUpdate, LightClientSnapshot};
use eth2::types::*;
use merkle_proof::MerkleTree;
use std::sync::Arc;
extern crate hex;
use swap_or_not_shuffle::compute_shuffled_index;
use bytes::{BufMut, BytesMut};
use ssz::{ssz_encode, Decode, DecodeError, Encode};
use ssz_types::{typenum::Unsigned, typenum::U32, BitVector, FixedVector, Bitfield};
use ethereum_types::H256;
use eth2_hashing::{hash};


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
    let finality_header = build_objects::get_header(&api_key, &state_id, &endpoint_prefix); //must have state_id == "finalized"

    // build update object
    // serialization, merkleization and branch extraction for beacon_state are in here
    let update = build_objects::get_update(state, block, finality_header);

}



