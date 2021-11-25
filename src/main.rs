use std::format;
use std::fs;
pub mod node_discovery;
pub mod http_requests;
pub mod build_objects;
pub mod light_client_types;
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
// from lodestar
const NEXT_SYNC_COMMITTEE_INDEX: u64 = 55;
const NEXT_SYNC_COMMITTEE_INDEX_FLOORLOG2: u64 = 55;
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
    //let body = block.message().body();
    let finality_header = build_objects::get_header(&api_key, &state_id, &endpoint_prefix);

    let leaves: Vec<H256> = chunkify_state_and_to_H256(&state);

    // check content of leaves vec
    println!("{:?}",leaves[0]);
    let tree: MerkleTree = get_merkle_tree(&leaves);
    //println!("{:?}",tree);

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

pub fn chunkify_state_and_to_H256(state: &BeaconState<MainnetEthSpec>)->Vec<H256>{

    // small inner func for converting vec<u8> to vecArray<u8>
    // i.e. make vec length fixed
    fn vector_as_u8_32_array(vector: Vec<u8>) -> [u8;32] {
        let mut arr = [0u8;32];
        for (place, element) in arr.iter_mut().zip(vector.iter()) {
            *place = *element;
        }
        arr
    }

            
    //ssz serialize the state object
    let serialized_state = state.as_ssz_bytes();
    
    // each element in serialized_state is a u8, i.e. 1 byte
    // chunks of 32 elements = 32 bytes as expected for merkleization
    let chunked = serialized_state.chunks(32);
    println!("chunked length: {:?}",chunked.len());

    // convert each 32 byte chunk of the serialized object into H256 type
    // and append each to vec leaves
    let mut leaves: Vec<H256> = vec![];
    for chunk in chunked{
        let chunk_vec = chunk.to_vec();
        let chunk_fixed: [u8; 32] = vector_as_u8_32_array(chunk_vec);
        let leaf = H256::from(chunk_fixed);
        leaves.push(leaf);
        }
        return leaves
}

pub fn get_merkle_tree(leaves: &Vec<H256>)-> MerkleTree{

    // // get tree depth and number of leaves to pass to merkle func
    let n_leaves: f64 = leaves.len() as f64;
    let tree_depth:usize = n_leaves.floor().log2() as usize;

    println!("n leaves: {:?}, tree_depth: {:?}", n_leaves, tree_depth);
    let tree_depth:usize = n_leaves.floor().log2() as usize;

    let mut merkle_tree = MerkleTree::create(&leaves, 49);
    
    return merkle_tree
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