use std::format;
use std::fs;
use eth2::types::*;
use merkle_proof::MerkleTree;
use std::sync::Arc;
extern crate hex;
use bytes::{BufMut, BytesMut};
use ssz::{ssz_encode, Decode, DecodeError, Encode};
use ssz_types::{typenum::Unsigned, typenum::U32, BitVector, FixedVector, Bitfield};
use ethereum_types::H256;
use eth2_hashing::{hash};


pub fn to_h256_chunks(state: &BeaconState<MainnetEthSpec>)->Vec<H256>{

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
    let tree_depth:usize = ((n_leaves.floor().log2())+1.0) as usize;

    let mut merkle_tree = MerkleTree::create(&leaves, tree_depth);
    
    return merkle_tree
}

pub fn get_branch_indices(leaf_index: u64)->Vec<usize>{
    // function takes leaf index and returns 
    // the indexes for all sibling and parent roots
    // required for a merkle proof for the leaf

    let mut branch: Vec<usize> = vec![];

    // initialize branch with the leaf
    branch.push(leaf_index as usize);
    
    // while the last item in the list is not the state root
    // sequence of pushes is: leaf, sibling, parent, sibling, parent...
    // i.e. up a lovel, get hash partner, up a level, get hash partner...
    while branch.last_mut().unwrap().to_owned() as u64 > 1{
        
        // index of the leaf and its left and right neighbours
        let leaf = branch.last_mut().unwrap().to_owned() as u64;
        let left = branch.last_mut().unwrap().to_owned() as u64 -1;
        let right = branch.last_mut().unwrap().to_owned() as u64 +1;
        
        // if the index is even we always want its right neighbour 
        // to hash with. If odd, always left neighbour.
        if branch.last_mut().unwrap().to_owned() as u64 % 2 ==0{
            branch.push(right as usize)
        }
        else{
            branch.push(left as usize)
        }
        
        // the parent is always floor of index/2.
        branch.push(math::round::floor((leaf/2) as f64,0) as usize);

        };

        println!("{:?}",branch);

        return branch
    }


    pub fn get_branch(tree: MerkleTree, indices: Vec<usize>){

        //let branch = indices.iter().map( |i| tree.Leaf[*i] ).collect::<Vec<_>>();
        //fs::write("./tree_test.txt",tree.to_string());
        println!("{:?}",tree[0]);
        //return branch
    }