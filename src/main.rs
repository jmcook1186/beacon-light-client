use std::format;
use std::fs;
pub mod node_discovery;
pub mod http_requests;
pub mod build_objects;
pub mod light_client_types;
use crate::light_client_types::{LightClientUpdate, LightClientSnapshot};
use eth2::types::*;
use eth2_hashing::{hash};
use std::sync::Arc;
extern crate hex;
use swap_or_not_shuffle::compute_shuffled_index;
use bytes::{BufMut, BytesMut};
use ssz::{ssz_encode, Decode, DecodeError, Encode};
use ssz_types::{typenum::Unsigned, typenum::U32, BitVector, FixedVector, Bitfield};



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
    //let body = block.message().body();
    let finality_header = build_objects::get_header(&api_key, &state_id, &endpoint_prefix);

    //ssz serialize the state object
    //let serialized_state = state.as_ssz_bytes();
    
    // NOW MERKLEIZE IT!!
    //let state_tree_root = state.canonical_root();    

    let update = get_update(state, block, finality_header);

}


pub fn get_update(state: BeaconState<MainnetEthSpec>, block: SignedBeaconBlock<MainnetEthSpec>, finality_header: BlockHeaderData)->LightClientUpdate{

    let aggregate: SyncAggregate<MainnetEthSpec> = block.message().body().sync_aggregate().unwrap().to_owned();


    let update = LightClientUpdate{

        header: state.latest_block_header().to_owned(),
        next_sync_committee: state.next_sync_committee().unwrap().to_owned(),
        finality_header: finality_header,
        sync_committee_bits: aggregate.sync_committee_bits,

    };

    println!("{:?}",update.sync_committee_bits);

    return update
}


// pub struct LightClientUpdate{
    
//     header: BeaconBlockHeader  // comes from header struct
//     // Next sync committee corresponding to the header
//     next_sync_committee: SyncCommittee  //full syncCommittee struct
//     //next_sync_committee_branch: Vector[Bytes32, floorlog2(NEXT_SYNC_COMMITTEE_INDEX)] // vector of bytes32 with length equal to floorlog2(generalizedindex)
//     // # Finality proof for the update header
//     finality_header: BeaconBlockHeader  // comes from header struct
//     //finality_branch: Vector[Bytes32, floorlog2(FINALIZED_ROOT_INDEX)]    // vector of bytes32 with length equal to floorlog2(generalizedindex)
//     // Sync committee aggregate signature
//     sync_committee_bits: Bitvector[SYNC_COMMITTEE_SIZE]   // comes from syncAggregate struct
//     sync_committee_signature: BLSSignature  // comes from syncAggregate struct
//     // Fork version for the aggregate signature
//     fork_version: Version
// }