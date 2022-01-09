use eth2::types::*;
use merkle_proof::MerkleTree;
extern crate hex;
use crate::constants::{BYTES_PER_LENGTH_OFFSET, MAXIMUM_LENGTH, N_VARIABLE_LENGTH};
use ethereum_types::H256;
use ssz::Encode;
use std::collections::HashMap;
use std::convert::From;

pub fn serialize_beacon_state(
    state: &BeaconState<MainnetEthSpec>,
) -> (Vec<u8>, HashMap<&str, usize>, HashMap<&str, usize>) {
    // func takes state object as received from api endpoint and serializes it
    // according to the ssz specs

    // make hashmap of var lengths to pass to merklize
    let mut sizes = HashMap::new();
    let mut offsets = HashMap::new();

    let genesis_time = state.genesis_time().as_ssz_bytes();
    assert!(genesis_time.len() < MAXIMUM_LENGTH);
    sizes.insert("genesis_time", genesis_time.ssz_bytes_len());

    let genesis_validators_root = state.genesis_validators_root().as_ssz_bytes();
    assert!(genesis_validators_root.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "genesis_validators_root",
        genesis_validators_root.ssz_bytes_len(),
    );

    let slot = state.slot().as_ssz_bytes();
    assert!(slot.len() < MAXIMUM_LENGTH);
    sizes.insert("slot", slot.ssz_bytes_len());

    let fork_prev_ver: Vec<u8> = state.fork().previous_version.as_ssz_bytes();
    assert!(fork_prev_ver.len() < MAXIMUM_LENGTH);
    sizes.insert("fork_prev_ver", fork_prev_ver.ssz_bytes_len());

    let fork_curr_ver: Vec<u8> = state.fork().current_version.as_ssz_bytes();
    assert!(fork_curr_ver.len() < MAXIMUM_LENGTH);
    sizes.insert("fork_curr_ver", fork_curr_ver.ssz_bytes_len());

    let fork_epoch: Vec<u8> = state.fork().epoch.as_ssz_bytes();
    assert!(fork_epoch.len() < MAXIMUM_LENGTH);
    sizes.insert("fork_epoch", fork_epoch.ssz_bytes_len());

    let header_slot: Vec<u8> = state.latest_block_header().slot.as_ssz_bytes();
    assert!(header_slot.len() < MAXIMUM_LENGTH);
    sizes.insert("header_slot", header_slot.ssz_bytes_len());

    let header_proposer_index: Vec<u8> = state.latest_block_header().proposer_index.as_ssz_bytes();
    assert!(header_proposer_index.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "header_proposer_index",
        header_proposer_index.ssz_bytes_len(),
    );

    let header_parent_root: Vec<u8> = state.latest_block_header().parent_root.as_ssz_bytes();
    assert!(header_parent_root.len() < MAXIMUM_LENGTH);
    sizes.insert("header_parent_root", header_parent_root.ssz_bytes_len());

    let header_state_root: Vec<u8> = state.latest_block_header().state_root.as_ssz_bytes();
    assert!(header_state_root.len() < MAXIMUM_LENGTH);
    sizes.insert("header_state_root", header_state_root.ssz_bytes_len());

    let header_body_root: Vec<u8> = state.latest_block_header().body_root.as_ssz_bytes();
    assert!(header_body_root.len() < MAXIMUM_LENGTH);
    sizes.insert("header_body_root", header_body_root.ssz_bytes_len());

    let block_roots: Vec<u8> = state.block_roots().as_ssz_bytes();
    assert!(block_roots.len() < MAXIMUM_LENGTH);
    sizes.insert("block_roots", block_roots.ssz_bytes_len());

    let state_roots: Vec<u8> = state.state_roots().as_ssz_bytes();
    assert!(state_roots.len() < MAXIMUM_LENGTH);
    sizes.insert("state_roots", state_roots.ssz_bytes_len());

    let historical_roots: Vec<u8> = state.historical_roots().as_ssz_bytes();
    assert!(historical_roots.len() < MAXIMUM_LENGTH);
    sizes.insert("historical_roots", historical_roots.ssz_bytes_len());

    let eth1_data_dep_root: Vec<u8> = state.eth1_data().deposit_root.as_ssz_bytes();
    assert!(eth1_data_dep_root.len() < MAXIMUM_LENGTH);
    sizes.insert("eth1_data_dep_root", eth1_data_dep_root.ssz_bytes_len());

    let eth1_data_deposit_count: Vec<u8> = state.eth1_data().deposit_count.as_ssz_bytes();
    assert!(eth1_data_deposit_count.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "eth1_data_deposit_count",
        eth1_data_deposit_count.ssz_bytes_len(),
    );

    let eth1_data_block_hash: Vec<u8> = state.eth1_data().block_hash.as_ssz_bytes();
    assert!(eth1_data_block_hash.len() < MAXIMUM_LENGTH);
    sizes.insert("eth1_data_block_hash", eth1_data_block_hash.ssz_bytes_len());

    let eth1_data_votes = state.eth1_data_votes().as_ssz_bytes();
    assert!(eth1_data_votes.len() < MAXIMUM_LENGTH);
    sizes.insert("eth1_data_votes", eth1_data_votes.ssz_bytes_len());

    let eth1_deposit_index: Vec<u8> = state.eth1_deposit_index().as_ssz_bytes();
    assert!(eth1_deposit_index.len() < MAXIMUM_LENGTH);
    sizes.insert("eth1_deposit_index", eth1_deposit_index.ssz_bytes_len());

    let validators: Vec<u8> = state.validators().as_ssz_bytes();
    assert!(validators.len() < MAXIMUM_LENGTH);
    sizes.insert("validators", validators.ssz_bytes_len());

    let balances: Vec<u8> = state.balances().as_ssz_bytes();
    assert!(balances.len() < MAXIMUM_LENGTH);
    sizes.insert("balances", balances.ssz_bytes_len());

    let randao_mixes: Vec<u8> = state.randao_mixes().as_ssz_bytes();
    assert!(randao_mixes.len() < MAXIMUM_LENGTH);
    sizes.insert("randao_mixes", randao_mixes.ssz_bytes_len());

    let slashings: Vec<u8> = state.slashings().as_ssz_bytes();
    assert!(slashings.len() < MAXIMUM_LENGTH);
    sizes.insert("slashings", slashings.ssz_bytes_len());

    let previous_epoch_participation: Vec<u8> =
        state.previous_epoch_participation().unwrap().as_ssz_bytes();
    assert!(previous_epoch_participation.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "previous_epoch_participation",
        previous_epoch_participation.ssz_bytes_len(),
    );

    let current_epoch_participation: Vec<u8> =
        state.current_epoch_participation().unwrap().as_ssz_bytes();
    assert!(current_epoch_participation.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "current_epoch_participation",
        current_epoch_participation.ssz_bytes_len(),
    );

    let justification_bits: Vec<u8> = state.justification_bits().as_ssz_bytes();
    assert!(justification_bits.len() < MAXIMUM_LENGTH);
    sizes.insert("justification_bits", justification_bits.ssz_bytes_len());

    let prev_just_check_epoch: Vec<u8> = state
        .previous_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    assert!(prev_just_check_epoch.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "prev_just_check_epoch",
        prev_just_check_epoch.ssz_bytes_len(),
    );

    let prev_just_check_root: Vec<u8> = state.previous_justified_checkpoint().root.as_ssz_bytes();
    assert!(prev_just_check_root.len() < MAXIMUM_LENGTH);
    sizes.insert("prev_just_check_root", prev_just_check_root.ssz_bytes_len());

    let curr_just_check_epoch: Vec<u8> = state
        .current_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    assert!(curr_just_check_epoch.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "curr_just_check_epoch",
        curr_just_check_epoch.ssz_bytes_len(),
    );

    let curr_just_check_root: Vec<u8> = state.current_justified_checkpoint().root.as_ssz_bytes();
    assert!(curr_just_check_root.len() < MAXIMUM_LENGTH);
    sizes.insert("curr_just_check_root", curr_just_check_root.ssz_bytes_len());

    let finalized_check_epoch: Vec<u8> = state.finalized_checkpoint().epoch.as_ssz_bytes();
    assert!(finalized_check_epoch.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "finalized_check_epoch",
        finalized_check_epoch.ssz_bytes_len(),
    );

    let finalized_checkpoint_root: Vec<u8> = state.finalized_checkpoint().root.as_ssz_bytes();
    assert!(finalized_checkpoint_root.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "finalized_checkpoint_root",
        finalized_checkpoint_root.ssz_bytes_len(),
    );

    let inactivity_scores: Vec<u8> = state.inactivity_scores().unwrap().as_ssz_bytes();
    assert!(inactivity_scores.len() < MAXIMUM_LENGTH);
    sizes.insert("inactivity_scores", inactivity_scores.ssz_bytes_len());

    let curr_sync_comm_pubkeys: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .pubkeys
        .as_ssz_bytes();
    assert!(curr_sync_comm_pubkeys.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "curr_sync_comm_pubkeys",
        curr_sync_comm_pubkeys.ssz_bytes_len(),
    );

    let curr_sync_comm_agg_pubkey: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();
    assert!(curr_sync_comm_agg_pubkey.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "curr_sync_comm_agg_pubkey",
        curr_sync_comm_agg_pubkey.ssz_bytes_len(),
    );

    let next_sync_comm_pubkeys: &Vec<u8> =
        &state.next_sync_committee().unwrap().pubkeys.as_ssz_bytes();
    assert!(next_sync_comm_pubkeys.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "next_sync_comm_pubkeys",
        next_sync_comm_pubkeys.ssz_bytes_len(),
    );

    let next_sync_comm_agg_pubkey: &Vec<u8> = &state
        .next_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();
    assert!(next_sync_comm_agg_pubkey.len() < MAXIMUM_LENGTH);
    sizes.insert(
        "next_sync_comm_agg_pubkey",
        next_sync_comm_agg_pubkey.ssz_bytes_len(),
    );

    // calculate length of fixed parts (required to calculate offsets later)
    // .len() is right for this as all vars have u8 type,
    // so N elements == N bytes
    // 4 bytes as placeholder for variable length offsets
    let byte_len_fixed_parts = genesis_time.len()
        + genesis_validators_root.len()
        + slot.len()
        + fork_curr_ver.len()
        + fork_prev_ver.len()
        + fork_epoch.len()
        + header_slot.len()
        + header_proposer_index.len()
        + header_parent_root.len()
        + header_body_root.len()
        + header_state_root.len()
        + block_roots.len()
        + state_roots.len()
        + eth1_data_block_hash.len()
        + eth1_data_dep_root.len()
        + eth1_data_deposit_count.len()
        + eth1_deposit_index.len()
        + randao_mixes.len()
        + slashings.len()
        + prev_just_check_epoch.len()
        + prev_just_check_root.len()
        + curr_just_check_epoch.len()
        + curr_just_check_root.len()
        + finalized_check_epoch.len()
        + finalized_checkpoint_root.len()
        + curr_sync_comm_pubkeys.len()
        + curr_sync_comm_agg_pubkey.len()
        + next_sync_comm_pubkeys.len()
        + next_sync_comm_agg_pubkey.len()
        + (BYTES_PER_LENGTH_OFFSET * N_VARIABLE_LENGTH);

    sizes.insert("fixed_parts", byte_len_fixed_parts);

    // CALCULATE VARIABLE LENGTH OFFSETS
    // AND MAKE THEM 4 BYTES LONG AS PER SPEC.
    // (see LH ssz/encode.rs encode_length() func for alternative implementation)
    // is trimming the last 4 bytes off the offset ok? could there be a scenario
    // where the offset is represented in > 4bytes and the trim leads to information loss?
    // unlikely - max val in 4bytes is 4,294,967,295.
    let historical_roots_offset: usize = byte_len_fixed_parts;
    offsets.insert("historical_roots", historical_roots_offset);
    let historical_roots_offset: [u8; 8] = historical_roots_offset.to_le_bytes();
    let historical_roots_offset: Vec<u8> = historical_roots_offset[0..4].to_vec();

    // offset starts after historical roots
    let eth1_data_votes_offset: usize = byte_len_fixed_parts + historical_roots.len();
    offsets.insert("eth1_data_votes", eth1_data_votes_offset);
    let eth1_data_votes_offset: [u8; 8] = eth1_data_votes_offset.to_le_bytes();
    let eth1_data_votes_offset: Vec<u8> = eth1_data_votes_offset[0..4].to_vec();

    // // offset starts after eth1 data votes
    let validators_offset: usize =
        byte_len_fixed_parts + historical_roots.len() + eth1_data_votes.len();
    offsets.insert("validators", validators_offset);
    let validators_offset: [u8; 8] = validators_offset.to_le_bytes();
    let validators_offset: Vec<u8> = validators_offset[0..4].to_vec();

    // // offset starts after validators
    let balances_offset: usize =
        byte_len_fixed_parts + historical_roots.len() + eth1_data_votes.len() + validators.len();
    offsets.insert("balances", balances_offset);
    let balances_offset: [u8; 8] = balances_offset.to_le_bytes();
    let balances_offset: Vec<u8> = balances_offset[0..4].to_vec();

    // // offset starts after balances
    let previous_epoch_participation_offset: usize = byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len();
    offsets.insert(
        "previous_epoch_participation",
        previous_epoch_participation_offset,
    );
    let previous_epoch_participation_offset: [u8; 8] =
        previous_epoch_participation_offset.to_le_bytes();
    let previous_epoch_participation_offset: Vec<u8> =
        previous_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let current_epoch_participation_offset: usize = byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len();
    offsets.insert(
        "current_epoch_participation",
        current_epoch_participation_offset,
    );
    let current_epoch_participation_offset: [u8; 8] =
        current_epoch_participation_offset.to_le_bytes();
    let current_epoch_participation_offset: Vec<u8> =
        current_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let justification_bits_offset: usize = byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len()
        + current_epoch_participation.len();
    offsets.insert("justification_bits", justification_bits_offset);
    let justification_bits_offset: [u8; 8] = justification_bits_offset.to_le_bytes();
    let justification_bits_offset: Vec<u8> = justification_bits_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let inactivity_scores_offset: usize = byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len()
        + current_epoch_participation.len()
        + justification_bits.len();
    offsets.insert("inactivity_scores", inactivity_scores_offset);
    let inactivity_scores_offset: [u8; 8] = inactivity_scores_offset.to_le_bytes();
    let inactivity_scores_offset: Vec<u8> = inactivity_scores_offset[0..4].to_vec();

    // check all offsets are 4 bytes long
    for i in [
        historical_roots_offset.len(),
        eth1_data_votes_offset.len(),
        validators_offset.len(),
        previous_epoch_participation_offset.len(),
        justification_bits_offset.len(),
        current_epoch_participation_offset.len(),
        balances_offset.len(),
        inactivity_scores_offset.len(),
    ]
    .iter()
    {
        assert_eq!(i.to_owned(), 4 as usize);
    }

    // BUILD SERIALIZED STATE OBJECT
    // interleave offsets with fixed-length data then
    // append var-length data

    // define serialized state object as empty vec
    let mut serialized_state: Vec<u8> = vec![];

    // add data and offsets sequentially
    // to empty vec
    for var in [
        genesis_time,
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
        justification_bits_offset,
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
        justification_bits,
        inactivity_scores,
    ] {
        for i in var {
            serialized_state.push(i)
        }
    }

    // OPTIONALLY PRINT SERIALIZED OBJECT PROPERTIES
    // println!("\n*** SERIALIZED OBJECT PROPERTIES ***\n");
    // println!(
    //     "byte length of ssz serialized state object: {:?}",
    //     serialized_state.len()
    // );
    // println!(
    //     "\nimplied byte length of variable length vars: {:?}",
    //     serialized_state.len() - byte_len_fixed_parts
    // );
    // println!("\nSIZE (BYTES) OF EACH VAR:\n");
    // for (key, value) in sizes.iter() {
    //     println!("{:?}: {:?}", key, value);
    // }
    // println!("\nVARIABLE LENGTH OFFSETS:\n");
    // for (key, value) in offsets.iter() {
    //     println!("{:?}: {:?}", key, value);
    // }

    return (serialized_state, sizes, offsets);
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
