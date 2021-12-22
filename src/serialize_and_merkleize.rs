use eth2::types::*;
use merkle_proof::MerkleTree;
extern crate hex;
use ethereum_types::H256;
use ssz::Encode;
use std::convert::From;
use std::mem::size_of_val;

pub fn serialize_beacon_state(state: &BeaconState<MainnetEthSpec>) -> Vec<u8>{
    
    let genesis_time = state.genesis_time().as_ssz_bytes();
    let genesis_validators_root = state.genesis_validators_root().as_ssz_bytes();
    let slot = state.slot().as_ssz_bytes();
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
    let eth1_data_votes = state.eth1_data_votes();
    let eth1_deposit_index: Vec<u8> = state.eth1_deposit_index().as_ssz_bytes();
    let validators: Vec<u8> = state.validators().as_ssz_bytes();
    let balances: Vec<u8> = state.balances().as_ssz_bytes();
    let randao_mixes: Vec<u8> = state.randao_mixes().as_ssz_bytes();
    let slashings: Vec<u8> = state.slashings().as_ssz_bytes();
    let previous_epoch_participation: Vec<u8> =
        state.previous_epoch_participation().unwrap().as_ssz_bytes();
    let current_epoch_participation: Vec<u8> =
        state.current_epoch_participation().unwrap().as_ssz_bytes();
    let justification_bits: Vec<u8> = state.justification_bits().as_ssz_bytes();
    let prev_just_check_epoch: Vec<u8> = state
        .previous_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    let prev_just_check_root: Vec<u8> = state.previous_justified_checkpoint().root.as_ssz_bytes();
    let curr_just_check_epoch: Vec<u8> = state
        .current_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    let curr_just_check_root: Vec<u8> = state.current_justified_checkpoint().root.as_ssz_bytes();
    let finalized_check_epoch: Vec<u8> = state.finalized_checkpoint().epoch.as_ssz_bytes();
    let finalized_checkpoint_root: Vec<u8> = state.finalized_checkpoint().root.as_ssz_bytes();
    let inactivity_scores: Vec<u8> = state.inactivity_scores().unwrap().as_ssz_bytes();
    let curr_sync_comm_pubkeys: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .pubkeys
        .as_ssz_bytes();
    let curr_sync_comm_agg_pubkey: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();
    let next_sync_comm_pubkeys: &Vec<u8> =
        &state.next_sync_committee().unwrap().pubkeys.as_ssz_bytes();
    let next_sync_comm_agg_pubkey: &Vec<u8> = &state
        .next_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();

    // eth1_data_votes is a bit complicated. It arrives from the state object as an array
    // of eth1_data containers, each of which has fields: deposit_root, count, block_hash.
    // The number of eth1_data containers inside eth1_data_votes will vary by slot.
    // To serialize this, we need to iterate through the object and append the raw byte
    // representation of the eth1_data containers to a simple byte array in the right order:

    // Vec[dep_root1, count1, blockhash1, dep_root2, count2, block_hash2, dep_root3, count3, block_hash3...]

    // A further complication is that "count" arrives as a u64 which needs to be cast as a 32 byte type
    // before serializing. The hashes are already 32 byte types. Here we go...

    // set up vec to hold raw eth1_data_votes data
    let mut eth1_data_votes_vec: Vec<u8> = vec![];

    // start looping through eth1_data_votes, one iteration per eth1_data container
    println!(
        "Iterating over {:?} vals in eth1_data_votes",
        eth1_data_votes.len()
    );

    for i in 0..eth1_data_votes.len() {

        //extract necessary values from  eth1_data object
        let dep_root: Vec<u8> = eth1_data_votes[i].deposit_root.as_ssz_bytes();
        let count = eth1_data_votes[i].deposit_count.as_ssz_bytes();
        let block_hash: Vec<u8> = eth1_data_votes[i].block_hash.as_ssz_bytes();

        // now for each byte in each field, push to eth1_data_votes_vec.
        // The ordering is critical - 32 bytes from dep_root first then
        // 32 bytes from count_vec, 32 bytes fromblock_hash
        for j in dep_root {
            eth1_data_votes_vec.push(j)
        }

        for j in count {
            eth1_data_votes_vec.push(j)
        }

        for j in block_hash {
            eth1_data_votes_vec.push(j)
        }
    }

    // assert that the length of the serialized dataset is equal to the number of eth1_data containers
    // multiplied by the sum of the lengths of each of their elements (32 bytes for hashes, 8 bytes for u64)
    assert_eq!(
        eth1_data_votes_vec.len(),
        eth1_data_votes.len() * (32 + 32 + 8)
    );

    // after loop has finished, eth1_data_votes_vec is the serialized form of eth1_data_votes ready to be merkleized
    // To avoid mistakes with var naming, we can overwrite eth1_data_votes (vec of containers) with eth1_data_votes_vec
    // (vec of bytes) and just use var eth1_data_votes from here on.
    let eth1_data_votes = eth1_data_votes_vec;

    // calculate length of fixed parts (required to calculate offsets later)
    // .len() is right for this as all vars have u8 type,
    // so N elements == N bytes
    // 4 bytes as placeholder for variable length offsets
    let mut fixed_parts: Vec<&u8> = vec![];
    let dummy_offset = vec![0u8, 0u8, 0u8, 0u8];

    for var in [
        &genesis_time,
        &genesis_validators_root,
        &slot,
        &fork_prev_ver,
        &fork_curr_ver,
        &fork_epoch,
        &header_slot,
        &header_proposer_index,
        &header_parent_root,
        &header_state_root,
        &header_body_root,
        &block_roots,
        &state_roots,
        &dummy_offset,
        &eth1_data_dep_root,
        &eth1_data_deposit_count,
        &eth1_data_block_hash,
        &dummy_offset,
        &eth1_deposit_index,
        &dummy_offset,
        &dummy_offset,
        &randao_mixes,
        &slashings,
        &previous_epoch_participation,
        &current_epoch_participation,
        &justification_bits,
        &prev_just_check_epoch,
        &prev_just_check_root,
        &curr_just_check_epoch,
        &curr_just_check_root,
        &finalized_check_epoch,
        &finalized_checkpoint_root,
        &dummy_offset,
        &curr_sync_comm_pubkeys,
        &curr_sync_comm_agg_pubkey,
        &next_sync_comm_pubkeys,
        &next_sync_comm_agg_pubkey,
    ] {
        for i in var {
            fixed_parts.push(i);
        }
    }

    let byte_len_fixed_parts = fixed_parts.len();

    println!("length of fixed part = {:?}", byte_len_fixed_parts);

    // CALCULATE VARIABLE LENGTH OFFSETS
    // TODO: MAKE ALL OFFSETS 4 BYTES LONG!!!!

    let historical_roots_offset: [u8; 8] = byte_len_fixed_parts.to_le_bytes();
    let historical_roots_offset: Vec<u8> = historical_roots_offset[0..4].to_vec();

    // offset starts after historical roots
    let eth1_data_votes_offset: [u8; 8] =
        ((byte_len_fixed_parts + historical_roots.len()).to_le_bytes());
    let eth1_data_votes_offset: Vec<u8> = eth1_data_votes_offset[0..4].to_vec();

    // // offset starts after eth1 data votes
    let validators_offset: [u8; 8] =
        ((byte_len_fixed_parts + historical_roots.len() + eth1_data_votes.len()).to_le_bytes());
    let validators_offset: Vec<u8> = validators_offset[0..4].to_vec();

    // // offset starts after validators
    let balances_offset: [u8; 8] = ((byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len())
    .to_le_bytes());
    let balances_offset: Vec<u8> = balances_offset[0..4].to_vec();

    // // offset starts after balances
    let previous_epoch_participation_offset: [u8; 8] = ((byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len())
    .to_le_bytes());
    let previous_epoch_participation_offset: Vec<u8> =
        previous_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let current_epoch_participation_offset: [u8; 8] = ((byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len())
    .to_le_bytes());
    let current_epoch_participation_offset: Vec<u8> =
        current_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let inactivity_scores_offset: [u8; 8] = ((byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len()
        + current_epoch_participation.len())
    .to_le_bytes());
    let inactivity_scores_offset: Vec<u8> = inactivity_scores_offset[0..4].to_vec();

    // for i in [
    //     historical_roots_offset,
    //     eth1_data_votes_offset,
    //     validators_offset,
    //     balances_offset,
    //     previous_epoch_participation_offset,
    //     current_epoch_participation_offset,
    //     inactivity_scores_offset,
    // ] {
    //     println!("{:?}", &i);
    // }

    // define serialized state object as empty vec
    let mut serialized_state: Vec<u8> = vec![];

    // add data to serialized state object
    for var in [
        genesis_validators_root,
        slot,
        fork_prev_ver,
        fork_curr_ver,
        fork_epoch,
        header_slot,
        header_proposer_index,
        header_parent_root,
        header_state_root,
        header_body_root,
        block_roots,
        state_roots,
        historical_roots_offset,
        eth1_data_dep_root,
        eth1_data_deposit_count,
        eth1_data_block_hash,
        eth1_data_votes_offset,
        eth1_deposit_index,
        validators_offset,
        balances_offset,
        randao_mixes,
        slashings,
        previous_epoch_participation_offset,
        current_epoch_participation_offset,
        justification_bits,
        prev_just_check_epoch,
        prev_just_check_root,
        curr_just_check_epoch,
        curr_just_check_root,
        finalized_check_epoch,
        finalized_checkpoint_root,
        inactivity_scores_offset,
        curr_sync_comm_pubkeys.to_owned(),
        curr_sync_comm_agg_pubkey.to_owned(),
        next_sync_comm_pubkeys.to_owned(),
        next_sync_comm_agg_pubkey.to_owned(),
        historical_roots,
        eth1_data_votes,
        validators,
        balances,
        previous_epoch_participation,
        current_epoch_participation,
        inactivity_scores,
    ] {
        for i in var {
            serialized_state.push(i)
        }
    }

    println!(
        "byte length of ssz serialized state object: {:?}",
        serialized_state.len()
    );

    println!(
        "implied byte length of variable length vars: {:?}",
        serialized_state.len()-byte_len_fixed_parts
    );

    return serialized_state;

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
