use eth2::types::*;
use merkle_proof::MerkleTree;
extern crate hex;
use ethereum_types::H256;
use ssz::Encode;
use std::convert::From;
use std::mem::size_of_val;



pub fn serialize_beacon_state(state: &BeaconState<MainnetEthSpec>){

    // FIRST ACCESS ALL LEAVES
    // an H256 is already a [u8; 32]: 
    // https://docs.rs/ethereum-types/0.3.2/ethereum_types/struct.H256.html

    let genesis_time = state.genesis_time().as_ssz_bytes();
    let genesis_validators_root = state.genesis_validators_root().as_ssz_bytes();
    let slot = state.slot().as_ssz_bytes();

    // println!("{:?}",genesis_time);
    // println!("{:?}",genesis_time.len());
    // println!("{:?}",std::mem::size_of_val(&genesis_time));

    // println!("{:?}",genesis_validators_root);
    // println!("{:?}",genesis_validators_root.len());
    // println!("{:?}",std::mem::size_of_val(&genesis_validators_root));

    // println!("{:?}",slot);
    // println!("{:?}",slot.len());
    // println!("{:?}",std::mem::size_of_val(&slot));

    let fork_prev_ver: Vec<u8> = state.fork().previous_version.as_ssz_bytes();
    let fork_curr_ver: Vec<u8> = state.fork().current_version.as_ssz_bytes();
    let fork_epoch: Vec<u8> = state.fork().epoch.as_ssz_bytes();
    let header_slot: Vec<u8> = state.latest_block_header().slot.as_ssz_bytes();
    let header_proposer_index: Vec<u8> = state.latest_block_header().proposer_index.as_ssz_bytes();
    let header_parent_root: Vec<u8> = state.latest_block_header().parent_root.as_ssz_bytes();
    let header_state_root: Vec<u8> = state.latest_block_header().state_root.as_ssz_bytes();
    let header_body_root: Vec<u8> = state.latest_block_header().body_root.as_ssz_bytes();
    let block_roots: Vec<u8> = state.block_roots().as_ssz_bytes();
    let state_roots: Vec<u8> = state.state_roots().as_ssz_bytes();
    let historical_roots: Vec<u8> = state.historical_roots().as_ssz_bytes();
    let eth1_data_dep_root: Vec<u8> = state.eth1_data().deposit_root.as_ssz_bytes();
    let eth1_data_deposit_count: Vec<u8> = state.eth1_data().deposit_count.as_ssz_bytes();
    let eth1_data_block_hash: Vec<u8> = state.eth1_data().block_hash.as_ssz_bytes();
    let eth1_data_votes: Vec<u8> = state.eth1_data_votes().as_ssz_bytes();
    let eth1_deposit_index: Vec<u8> = state.eth1_deposit_index().as_ssz_bytes();
    let validators: Vec<u8> = state.validators().as_ssz_bytes();
    let balances: Vec<u8> = state.balances().as_ssz_bytes();
    let randao_mixes: Vec<u8> = state.randao_mixes().as_ssz_bytes();
    let slashings: Vec<u8> = state.slashings().as_ssz_bytes();
    let previous_epoch_participation: Vec<u8> = state.previous_epoch_participation().unwrap().as_ssz_bytes();
    let current_epoch_participation: Vec<u8> = state.current_epoch_participation().unwrap().as_ssz_bytes();
    let justification_bits: Vec<u8> = state.justification_bits().as_ssz_bytes();
    let prev_just_check_epoch: Vec<u8> = state.previous_justified_checkpoint().epoch.as_u64().as_ssz_bytes();
    let prev_just_check_root: Vec<u8> = state.previous_justified_checkpoint().root.as_ssz_bytes();
    let curr_just_check_epoch: Vec<u8> = state.current_justified_checkpoint().epoch.as_u64().as_ssz_bytes();
    let curr_just_check_root: Vec<u8> = state.current_justified_checkpoint().root.as_ssz_bytes();
    let finalized_check_epoch: Vec<u8> = state.finalized_checkpoint().epoch.as_ssz_bytes();
    let finalized_checkpoint_root: Vec<u8> = state.finalized_checkpoint().root.as_ssz_bytes();
    let inactivity_scores: Vec<u8> = state.inactivity_scores().unwrap().as_ssz_bytes();
    let curr_sync_comm_pubkeys: &Vec<u8> = &state.current_sync_committee().unwrap().pubkeys.as_ssz_bytes();
    let curr_sync_comm_agg_pubkey: &Vec<u8> = &state.current_sync_committee().unwrap().aggregate_pubkey.as_ssz_bytes();
    let next_sync_comm_pubkeys: &Vec<u8> = &state.next_sync_committee().unwrap().pubkeys.as_ssz_bytes();
    let next_sync_comm_agg_pubkey: &Vec<u8> = &state.next_sync_committee().unwrap().aggregate_pubkey.as_ssz_bytes();

    balances.whatami();
    println!("{:?}", fork_prev_ver);
    println!("{:?}", fork_prev_ver.len());
    println!("{:?}", std::mem::size_of_val(&fork_prev_ver[0]));

    // all of these fields need to be cast as 32 byte values so each value becomes a leaf.
    // for fields with >1 element, each element must be a 32 byte value and there must be an
    // even number of them so that the top level field can be represented as a root. If 
    // there is not naturally an even number of elements, we add 32 bytes of zeros.

    pub fn u64_to_u8_32(var: u64)->Vec<u8>{
        
        let var_bytes = var.to_le_bytes();
        let mut var_out: Vec<u8>= vec![];
        // for positions 0-25 in count_vec, append zero (left pad vec)
        for j in 0..(32-var_bytes.len()){
            var_out.push(0u8);
        }
        // now append the 8 bytes of real data to the count vec
        for j in var_bytes{
            var_out.push(j);
        }

        assert_eq!(var_out.len(), 32);
        

        
        return var_out

    }




    // eth1_data_votes is a bit complicated. It arrives from the state object as an array
    // of eth1_data containers, each of which has fields: deposit_root, count, block_hash. 
    // The number of eth1_data containers inside eth1_data_votes will vary by slot.
    // To serialize this, we need to iterate through the object and append the raw byte
    // representation of the eth1_data containers to a simple byte array in the right order:

    // Vec[dep_root1, count1, blockhash1, dep_root2, count2, block_hash2, dep_root3, count3, block_hash3...]

    // A further complication is that "count" arrives as a u64 which needs to be cast as a 32 byte type
    // before serializing. The hashes are already 32 byte types. Here we go...

    // set up vec to hold raw eth1_data_votes data
    // let mut eth1_data_votes_vec: Vec<u8> = vec![];
    
    // start looping through eth1_data_votes, one iteration per eth1_data container
    // for i in 0..eth1_data_votes.len(){
    //     // set up temp vec that exists to pad count: u64 to [u8; 32]
    //     let mut count_vec: Vec<u8> = vec![];
    //     //extract necessary values from  eth1_data object
    //     let dep_root: &[u8; 32] = eth1_data_votes[i].deposit_root.as_fixed_bytes();
    //     let count = eth1_data_votes[i].deposit_count;
    //     let block_hash: &[u8; 32] = eth1_data_votes[i].block_hash.as_fixed_bytes(); 

    //     let deposit_count: Vec<u8> = u64_to_u8_32(count);

    //     // for positions 0-25 in count_vec, append zero (left pad vec)
    //     for j in 0..(32-count.len()){
            
    //         count_vec.push(0u8);

    //     }
    //     // now append the 8 bytes of real data to the count vec
    //     for j in count{
    //         count_vec.push(j);
    //     }

    //     now for each byte in each field, push to eth1_data_votes_vec.
    //     The ordering is critical - 32 bytes from dep_root first then
    //     32 bytes from count_vec, 32 bytes fromblock_hash
    //     for j in dep_root{
    //         eth1_data_votes_vec.push(*j)
    //     }

    //     for j in deposit_count{
    //         eth1_data_votes_vec.push(j)
    //     }

    //     for j in block_hash{
    //         eth1_data_votes_vec.push(*j)
    //     }

    // }
    // after loop has finished, eth1_data_votes_vec is the serialized form of eth1_data_votes ready to be merkleized
    // To avoid mistakes with var naming, we can overwrite eth1_data_votes (vec of containers) with eth1_data_votes_vec
    // (vec of bytes) and just use var eth1_data_votes from here on.

   // let eth1_data_votes = eth1_data_votes_vec;

  //  println!("{:?}",historical_roots);
}





pub fn to_h256_chunks(state: &BeaconState<MainnetEthSpec>) -> Vec<H256> {
    // small inner func for converting vec<u8> to vecArray<u8>
    // i.e. make vec length fixed
    fn vector_as_u8_32_array(vector: Vec<u8>) -> [u8; 32] {
        let mut arr = [0u8; 32];
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
    println!("chunked length: {:?}", chunked.len());

    // convert each 32 byte chunk of the serialized object into H256 type
    // and append each to vec leaves
    let mut leaves: Vec<H256> = vec![];
    for chunk in chunked {
        let chunk_vec = chunk.to_vec();
        let chunk_fixed: [u8; 32] = vector_as_u8_32_array(chunk_vec);
        let leaf = H256::from(chunk_fixed);
        leaves.push(leaf);
    }
    return leaves;
}

pub fn get_merkle_tree(leaves: &Vec<H256>) -> (MerkleTree, usize) {
    // // get tree depth and number of leaves to pass to merkle func
    let n_leaves: f64 = leaves.len() as f64;

    let tree_depth: usize = ((n_leaves.floor().log2()) + 1.0) as usize;

    let merkle_tree = MerkleTree::create(&leaves, tree_depth);

    return (merkle_tree, tree_depth);
}

pub fn get_branch_indices(leaf_index: usize) -> Vec<usize> {
    // function takes leaf index and returns
    // the indexes for all sibling and parent roots
    // required for a merkle proof for the leaf
    // NB not actually implemented in main() bc
    // superseded by Lighthouse's get_proof() func

    let mut branch_indices: Vec<usize> = vec![];

    // initialize branch with the leaf
    branch_indices.push(leaf_index as usize);

    // while the last item in the list is not the state root
    // sequence of pushes is: leaf, sibling, parent, sibling, parent...
    // i.e. up a lovel, get hash partner, up a level, get hash partner...
    while branch_indices.last_mut().unwrap().to_owned() as u64 > 1 {
        // index of the leaf and its left and right neighbours
        let leaf = branch_indices.last_mut().unwrap().to_owned() as u64;
        let left = branch_indices.last_mut().unwrap().to_owned() as u64 - 1;
        let right = branch_indices.last_mut().unwrap().to_owned() as u64 + 1;

        // if the index is even we always want its right neighbour
        // to hash with. If odd, always left neighbour.
        if branch_indices.last_mut().unwrap().to_owned() as u64 % 2 == 0 {
            branch_indices.push(right as usize)
        } else {
            branch_indices.push(left as usize)
        }

        // the parent is always floor of index/2.
        branch_indices.push(math::round::floor((leaf / 2) as f64, 0) as usize);
    }

    return branch_indices;
}

pub fn get_branch(tree: &MerkleTree, leaf_index: usize, tree_depth: usize) -> Vec<H256> {
    let (_leaf, branch) = tree.generate_proof(leaf_index, tree_depth);

    return branch;
}
