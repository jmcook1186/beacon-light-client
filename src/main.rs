use std::format;
use std::fs;
pub mod node_discovery;
pub mod http_requests;
pub mod build_objects;
pub mod light_client_types;
pub mod serialize_and_merkleize;
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

// grab precomputed generalized indices and vec[root] lengths
// from lodestar. floor_log2 gives the length of the list
// of roots in the branch connecting these leaves to the state tree root
const NEXT_SYNC_COMMITTEE_INDEX: u64 = 55;
const NEXT_SYNC_COMMITTEE_INDEX_FLOORLOG2: u64 = 5;
const FINALIZED_ROOT_INDEX: u64 = 105;
const FINALIZED_ROOT_INDEX_FLOOR_LOG2: u64 = 6;


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

    // merklize beacon_state
    let leaves: Vec<H256> = serialize_and_merkleize::to_h256_chunks(&state);
    let tree: MerkleTree = serialize_and_merkleize::get_merkle_tree(&leaves);

    let branch_indices = serialize_and_merkleize::get_branch_indices(NEXT_SYNC_COMMITTEE_INDEX); 
    let branch = serialize_and_merkleize::get_branch(tree, branch_indices);

    // build update object
    let update = get_update(state, block, finality_header);

}


pub fn get_update(state: BeaconState<MainnetEthSpec>, block: SignedBeaconBlock<MainnetEthSpec>, finality_header: BlockHeaderData)->LightClientUpdate{

    let aggregate: SyncAggregate<MainnetEthSpec> = block.message().body().sync_aggregate().unwrap().to_owned();


    let update = LightClientUpdate{

        header: state.latest_block_header().to_owned(),
        next_sync_committee: state.next_sync_committee().unwrap().to_owned(),
        finality_header: finality_header,
        sync_committee_bits: aggregate.sync_committee_bits,
        fork_version: state.fork().current_version,

    };


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