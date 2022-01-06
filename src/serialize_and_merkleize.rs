use eth2::types::*;
use merkle_proof::MerkleTree;
extern crate hex;
use ethereum_types::H256;
use sha2::{Digest, Sha256};
use ssz::Encode;
use std::collections::HashMap;
use std::convert::From;
use std::mem::size_of_val;

pub fn serialize_beacon_state(
    state: &BeaconState<MainnetEthSpec>,
) -> (Vec<u8>, HashMap<&str, usize>) {
    // func takes state object as received from api endpoint and serializes it
    // according to the ssz specs

    // make hashmap of var lengths to pass to merklize
    let mut sizes = HashMap::new();

    let genesis_time = state.genesis_time().as_ssz_bytes();
    sizes.insert("genesis_time", genesis_time.ssz_bytes_len());

    let genesis_validators_root = state.genesis_validators_root().as_ssz_bytes();
    sizes.insert(
        "genesis_validators_root",
        genesis_validators_root.ssz_bytes_len(),
    );

    let slot = state.slot().as_ssz_bytes();
    sizes.insert("slot", slot.ssz_bytes_len());

    let fork_prev_ver: Vec<u8> = state.fork().previous_version.as_ssz_bytes();
    sizes.insert("fork_prev_ver", fork_prev_ver.ssz_bytes_len());

    let fork_curr_ver: Vec<u8> = state.fork().current_version.as_ssz_bytes();
    sizes.insert("fork_curr_ver", fork_curr_ver.ssz_bytes_len());

    let fork_epoch: Vec<u8> = state.fork().epoch.as_ssz_bytes();
    sizes.insert("fork_epoch", fork_epoch.ssz_bytes_len());

    let header_slot: Vec<u8> = state.latest_block_header().slot.as_ssz_bytes();
    sizes.insert("header_slot", header_slot.ssz_bytes_len());

    let header_proposer_index: Vec<u8> = state.latest_block_header().proposer_index.as_ssz_bytes();
    sizes.insert(
        "header_proposer_index",
        header_proposer_index.ssz_bytes_len(),
    );

    let header_parent_root: Vec<u8> = state.latest_block_header().parent_root.as_ssz_bytes();
    sizes.insert("header_parent_root", header_parent_root.ssz_bytes_len());

    let header_state_root: Vec<u8> = state.latest_block_header().state_root.as_ssz_bytes();
    sizes.insert("header_state_root", header_state_root.ssz_bytes_len());

    let header_body_root: Vec<u8> = state.latest_block_header().body_root.as_ssz_bytes();
    sizes.insert("header_body_root", header_body_root.ssz_bytes_len());

    let block_roots: Vec<u8> = state.block_roots().as_ssz_bytes();
    sizes.insert("block_roots", block_roots.ssz_bytes_len());

    let state_roots: Vec<u8> = state.state_roots().as_ssz_bytes();
    sizes.insert("state_roots", state_roots.ssz_bytes_len());

    let historical_roots: Vec<u8> = state.historical_roots().as_ssz_bytes();
    sizes.insert("historical_roots", historical_roots.ssz_bytes_len());

    let eth1_data_dep_root: Vec<u8> = state.eth1_data().deposit_root.as_ssz_bytes();
    sizes.insert("eth1_data_dep_root", eth1_data_dep_root.ssz_bytes_len());

    let eth1_data_deposit_count: Vec<u8> = state.eth1_data().deposit_count.as_ssz_bytes();
    sizes.insert(
        "eth1_data_deposit_count",
        eth1_data_deposit_count.ssz_bytes_len(),
    );

    let eth1_data_block_hash: Vec<u8> = state.eth1_data().block_hash.as_ssz_bytes();
    sizes.insert("eth1_data_block_hash", eth1_data_block_hash.ssz_bytes_len());

    let eth1_data_votes = state.eth1_data_votes().as_ssz_bytes();
    sizes.insert("eth1_data_votes", eth1_data_votes.ssz_bytes_len());

    let eth1_deposit_index: Vec<u8> = state.eth1_deposit_index().as_ssz_bytes();
    sizes.insert("eth1_deposit_index", eth1_deposit_index.ssz_bytes_len());

    let validators: Vec<u8> = state.validators().as_ssz_bytes();
    sizes.insert("validators", validators.ssz_bytes_len());

    let balances: Vec<u8> = state.balances().as_ssz_bytes();
    sizes.insert("balances", balances.ssz_bytes_len());

    let randao_mixes: Vec<u8> = state.randao_mixes().as_ssz_bytes();
    sizes.insert("randao_mixes", randao_mixes.ssz_bytes_len());

    let slashings: Vec<u8> = state.slashings().as_ssz_bytes();
    sizes.insert("slashings", slashings.ssz_bytes_len());

    let previous_epoch_participation: Vec<u8> =
        state.previous_epoch_participation().unwrap().as_ssz_bytes();
    sizes.insert(
        "previous_epoch_participation",
        previous_epoch_participation.ssz_bytes_len(),
    );

    let current_epoch_participation: Vec<u8> =
        state.current_epoch_participation().unwrap().as_ssz_bytes();
    sizes.insert(
        "current_epoch_participation",
        current_epoch_participation.ssz_bytes_len(),
    );

    let justification_bits: Vec<u8> = state.justification_bits().as_ssz_bytes();
    sizes.insert("justification_bits", justification_bits.ssz_bytes_len());

    let prev_just_check_epoch: Vec<u8> = state
        .previous_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    sizes.insert(
        "prev_just_check_epoch",
        prev_just_check_epoch.ssz_bytes_len(),
    );

    let prev_just_check_root: Vec<u8> = state.previous_justified_checkpoint().root.as_ssz_bytes();
    sizes.insert("prev_just_check_root", prev_just_check_root.ssz_bytes_len());

    let curr_just_check_epoch: Vec<u8> = state
        .current_justified_checkpoint()
        .epoch
        .as_u64()
        .as_ssz_bytes();
    sizes.insert(
        "curr_just_check_epoch",
        curr_just_check_epoch.ssz_bytes_len(),
    );

    let curr_just_check_root: Vec<u8> = state.current_justified_checkpoint().root.as_ssz_bytes();
    sizes.insert("curr_just_check_root", curr_just_check_root.ssz_bytes_len());

    let finalized_check_epoch: Vec<u8> = state.finalized_checkpoint().epoch.as_ssz_bytes();
    sizes.insert(
        "finalized_check_epoch",
        finalized_check_epoch.ssz_bytes_len(),
    );

    let finalized_checkpoint_root: Vec<u8> = state.finalized_checkpoint().root.as_ssz_bytes();
    sizes.insert(
        "finalized_checkpoint_root",
        finalized_checkpoint_root.ssz_bytes_len(),
    );

    let inactivity_scores: Vec<u8> = state.inactivity_scores().unwrap().as_ssz_bytes();
    sizes.insert("inactivity_scores", inactivity_scores.ssz_bytes_len());

    let curr_sync_comm_pubkeys: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .pubkeys
        .as_ssz_bytes();
    sizes.insert(
        "curr_sync_comm_pubkeys",
        curr_sync_comm_pubkeys.ssz_bytes_len(),
    );

    let curr_sync_comm_agg_pubkey: &Vec<u8> = &state
        .current_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();
    sizes.insert(
        "curr_sync_comm_agg_pubkey",
        curr_sync_comm_agg_pubkey.ssz_bytes_len(),
    );

    let next_sync_comm_pubkeys: &Vec<u8> =
        &state.next_sync_committee().unwrap().pubkeys.as_ssz_bytes();
    sizes.insert(
        "next_sync_comm_pubkeys",
        next_sync_comm_pubkeys.ssz_bytes_len(),
    );

    let next_sync_comm_agg_pubkey: &Vec<u8> = &state
        .next_sync_committee()
        .unwrap()
        .aggregate_pubkey
        .as_ssz_bytes();
    sizes.insert(
        "next_sync_comm_agg_pubkey",
        next_sync_comm_agg_pubkey.ssz_bytes_len(),
    );

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
        &dummy_offset,
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
    sizes.insert("fixed_parts", byte_len_fixed_parts);
    println!("length of fixed part = {:?}", byte_len_fixed_parts);

    // CALCULATE VARIABLE LENGTH OFFSETS
    // AND MAKE THEM 4 BYTES LONG AS PER SPEC.
    // (see LH ssz/encode.rs encode_length() func for alternative implementation)
    // is trimming the last 4 bytes off the offset ok? could there be a scenario
    // where the offset is represented in > 4bytes and the trim leads to information loss?
    // unlikely - max val in 4bytes is 4,294,967,295.
    let historical_roots_offset: [u8; 8] = byte_len_fixed_parts.to_le_bytes();
    let historical_roots_offset: Vec<u8> = historical_roots_offset[0..4].to_vec();

    // offset starts after historical roots
    let eth1_data_votes_offset: [u8; 8] =
        (byte_len_fixed_parts + historical_roots.ssz_bytes_len()).to_le_bytes();
    let eth1_data_votes_offset: Vec<u8> = eth1_data_votes_offset[0..4].to_vec();

    // // offset starts after eth1 data votes
    let validators_offset: [u8; 8] =
        (byte_len_fixed_parts + historical_roots.len() + eth1_data_votes.len()).to_le_bytes();
    let validators_offset: Vec<u8> = validators_offset[0..4].to_vec();

    // // offset starts after validators
    let balances_offset: [u8; 8] =
        (byte_len_fixed_parts + historical_roots.len() + eth1_data_votes.len() + validators.len())
            .to_le_bytes();
    let balances_offset: Vec<u8> = balances_offset[0..4].to_vec();

    // // offset starts after balances
    let previous_epoch_participation_offset: [u8; 8] = (byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len())
    .to_le_bytes();
    let previous_epoch_participation_offset: Vec<u8> =
        previous_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let current_epoch_participation_offset: [u8; 8] = (byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len())
    .to_le_bytes();
    let current_epoch_participation_offset: Vec<u8> =
        current_epoch_participation_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let justification_bits_offset: [u8; 8] = (byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len()
        + current_epoch_participation.len())
    .to_le_bytes();
    let justification_bits_offset: Vec<u8> = justification_bits_offset[0..4].to_vec();

    // // offset starts after previous_epoch
    let inactivity_scores_offset: [u8; 8] = (byte_len_fixed_parts
        + historical_roots.len()
        + eth1_data_votes.len()
        + validators.len()
        + balances.len()
        + previous_epoch_participation.len()
        + current_epoch_participation.len()
        + justification_bits.len())
    .to_le_bytes();
    let inactivity_scores_offset: Vec<u8> = inactivity_scores_offset[0..4].to_vec();

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

    println!(
        "byte length of ssz serialized state object: {:?}",
        serialized_state.len()
    );

    println!(
        "implied byte length of variable length vars: {:?}",
        serialized_state.len() - byte_len_fixed_parts
    );

    for (key, value) in sizes.iter() {
        println!("{:?}: {:?}", key, value);
    }

    return (serialized_state, sizes);
}

pub fn merkleize_state(serialized_state: &Vec<u8>, sizes: &HashMap<&str, usize>) {
    // takes vec<u8> of bytes - this is the actual serialized data
    // also takes Hashmap of <str, usize> - this is the byte length
    // of each field (actual length not offset for variable length fields)

    // 1) need to know size in bytes of every element in state (from hashmap)
    // object so we can retrieve their bytes from the serialized state. Also
    // need a hasher that can take 32 bytes and return a valid hash

    // 2) Need to examine each element to ensure each leaf is exactly 32 bytes
    // for those leaves that are not 32 bytes long, right pad them

    // 3) The number of leaves must be a power of 2 to form a tree, so
    // where the leaves feeding a root are not power of 2, add zero vectors

    // 4) for containers, hash leaves together sequentially to produce a
    // container hash

    // 5) Now the tree should have one 32 byte element for each field in the
    // state object, so we can start to hash adjacent leaves to form the merkle tree
    // and veentually compute the state root

    // 6) We should then be able to verify that the serialization and merkleization
    // was successful by comparing the computed root to the state root in the block header

    // then on to generalized indices - can we verify that the objects in the
    // positions defined in our constants file definitely contain the right data?
    // If so we need to extract branches, meaning hashes of all nodes connecting
    // leaf to root.

    let mut leaves = vec![];
    let mut start_idx: usize = 0;

    pub fn hash(leaf: &Vec<u8>) -> String {
        assert_eq!(leaf.len() % 32, 0);

        let mut hasher = Sha256::new();
        hasher.update(leaf);
        let result = hasher.finalize_reset();
        return hex::encode(result);
    }

    pub fn pad_bytes(start: usize, mut length: usize, serialized_state: &Vec<u8>) -> Vec<u8> {
        // start and stop idxs for vars in ssz serialized object
        let stop = start + length;
        let var_as_bytes = &serialized_state[start..stop];

        // if there are exactly 32 bytes in var
        if length == 32 {
            assert_eq!(var_as_bytes.len(), 32);
            return var_as_bytes.to_vec();
        } else if length < 32 {
            let mut padded_var: Vec<u8> = vec![];
            let n_pad = 32 - length;
            let pad = vec![0u8; n_pad];

            for i in var_as_bytes.iter() {
                padded_var.push(*i);
            }
            padded_var.extend_from_slice(&pad);

            assert_eq!(padded_var.len(), 32);

            return padded_var;
        }
        // else if there are more than 32 bytes in var
        else {
            if length % 32 == 0 {
                let n_chunks: usize = length / 32;

                // if N chunks is already a power of 2
                if n_chunks.is_power_of_two() {
                    assert_eq!(length / 32 % 2, 0);
                    assert_eq!(length % 32, 0);
                    return var_as_bytes.to_vec();
                }
                // if N chunks is not a power of 2,
                // calculate how many chunks are needed and
                // add that many [0u8; 32] to the serialized var
                else {
                    let chunks_to_add = n_chunks.next_power_of_two() - n_chunks;

                    let mut padded_var: Vec<u8> = vec![];
                    let pad = vec![0u8; 32 * chunks_to_add];
                    for i in var_as_bytes.iter() {
                        padded_var.push(*i);
                    }
                    padded_var.extend_from_slice(&pad);

                    assert_eq!(length / 32 % 2, 0);
                    assert_eq!(length % 32, 0);

                    return padded_var;
                }
            } else {
                // make vec containing existing var bytes
                // make a single 0 pad
                let mut padded_var: Vec<u8> = vec![];
                let pad = vec![0u8; 1];
                // add the var bytes to the new vec
                for i in var_as_bytes.iter() {
                    padded_var.push(*i);
                }
                // add 0's to the vec until the length is a multiple of 32
                while (length % 32 != 0) {
                    padded_var.extend_from_slice(&pad);
                    length += 1;
                }

                // make sure the length var == actual var length
                assert_eq!(padded_var.len(), length);

                // count 32 byte chunks in length
                // update var_as_bytes to the new padded_var
                // and reuse the variable names in next round
                // of padding
                let n_chunks: usize = length / 32;

                // if N chunks is already a power of 2
                // just return the var
                if n_chunks.is_power_of_two() {
                    assert_eq!(length / 32 % 2, 0);
                    assert_eq!(length % 32, 0);
                    return padded_var;
                }
                // if N chunks is not a power of 2,
                // calculate how many chunks are needed and
                // add that many [0u8; 32] to the serialized var
                else {
                    let chunks_to_add = n_chunks.next_power_of_two() - n_chunks;

                    let pad = vec![0u8; 32 * chunks_to_add];

                    padded_var.extend_from_slice(&pad);

                    assert_eq!(length / 32 % 2, 0);
                    assert_eq!(length % 32, 0);
                    assert_eq!(length, n_chunks * 32);

                    return padded_var;
                }
            }
        }
    }

    // APPLY PAD AND HASH FUNCS TO EACH VAR
    let genesis_time = pad_bytes(start_idx, sizes["genesis_time"], &serialized_state);
    start_idx += sizes["genesis_time"] as usize;
    let leaf_hash = hash(&genesis_time);
    leaves.push(leaf_hash);

    let genesis_validators_root = pad_bytes(
        start_idx,
        sizes["genesis_validators_root"],
        &serialized_state,
    );
    start_idx += sizes["genesis_validators_root"] as usize;
    let leaf_hash = hash(&genesis_validators_root);
    leaves.push(leaf_hash);

    let slot = pad_bytes(start_idx, sizes["slot"], &serialized_state);
    start_idx += sizes["slot"] as usize;
    let leaf_hash = hash(&slot);
    leaves.push(leaf_hash);

    let fork_prev_ver = pad_bytes(start_idx, sizes["fork_prev_ver"], &serialized_state);
    start_idx += sizes["fork_prev_ver"] as usize;
    let leaf_hash = hash(&fork_prev_ver);
    leaves.push(leaf_hash);

    let fork_curr_ver = pad_bytes(start_idx, sizes["fork_curr_ver"], &serialized_state);
    start_idx += sizes["fork_curr_ver"] as usize;
    let leaf_hash = hash(&fork_curr_ver);
    leaves.push(leaf_hash);

    let fork_epoch = pad_bytes(start_idx, sizes["fork_epoch"], &serialized_state);
    start_idx += sizes["fork_epoch"] as usize;
    let leaf_hash = hash(&fork_epoch);
    leaves.push(leaf_hash);

    let header_slot = pad_bytes(start_idx, sizes["header_slot"], &serialized_state);
    start_idx += sizes["header_slot"] as usize;
    let leaf_hash = hash(&header_slot);
    leaves.push(leaf_hash);

    let header_proposer_index =
        pad_bytes(start_idx, sizes["header_proposer_index"], &serialized_state);
    start_idx += sizes["header_proposer_index"] as usize;
    let leaf_hash = hash(&header_proposer_index);
    leaves.push(leaf_hash);

    let header_parent_root = pad_bytes(start_idx, sizes["header_parent_root"], &serialized_state);
    start_idx += sizes["header_parent_root"] as usize;
    let leaf_hash = hash(&header_parent_root);
    leaves.push(leaf_hash);

    let header_state_root = pad_bytes(start_idx, sizes["header_state_root"], &serialized_state);
    start_idx += sizes["header_state_root"] as usize;
    let leaf_hash = hash(&header_state_root);
    leaves.push(leaf_hash);

    let header_body_root = pad_bytes(start_idx, sizes["header_body_root"], &serialized_state);
    start_idx += sizes["header_body_root"] as usize;
    let leaf_hash = hash(&header_body_root);
    leaves.push(leaf_hash);

    let block_roots = pad_bytes(start_idx, sizes["block_roots"], &serialized_state);
    start_idx += sizes["block_roots"] as usize;
    let leaf_hash = hash(&block_roots);
    leaves.push(leaf_hash);

    let state_roots = pad_bytes(start_idx, sizes["state_roots"], &serialized_state);
    start_idx += sizes["state_roots"] as usize;
    let leaf_hash = hash(&state_roots);
    leaves.push(leaf_hash);

    for i in leaves.iter() {
        println!("{:?}", i);
    }
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
