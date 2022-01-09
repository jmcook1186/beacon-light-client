extern crate hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub fn calculate_leaves(
    serialized_state: &Vec<u8>,
    sizes: &HashMap<&str, usize>,
    offsets: &HashMap<&str, usize>,
) -> Vec<String> {
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
        assert!(leaf.len() >= 32);
        assert_eq!(leaf.len() % 32, 0);

        if leaf.len() == 32 {
            let mut hasher = Sha256::new();
            hasher.update(leaf);
            let result = hasher.finalize_reset();
            return hex::encode(result);
        } else {
            // here we deal with vars that have multiple chunks
            // by recursively hashing pairs and returning the root

            let chunked_leaf: Vec<Vec<u8>> = leaf.chunks(32).map(|s| s.into()).collect();

            assert!(leaf.len() > 32);
            assert!(leaf.len() % 32 == 0);
            assert!(chunked_leaf.len() == leaf.len() / 32);

            let root: String;
            let mut chunks = chunked_leaf.clone();
            while chunks.len() != 1 {
                // while there are multiple nodes to hash
                let mut temp: Vec<Vec<u8>> = vec![];
                for i in (0..chunks.len()).step_by(2) {
                    // step through nodes in pairs
                    let mut hasher = Sha256::new();
                    hasher.update(&chunks[i]);
                    hasher.update(&chunks[i + 1]);
                    temp.push(hasher.finalize_reset().to_vec());
                }
                chunks = temp;
            }
            assert!(chunks.len() == 1);
            assert_eq!(chunks[0].len(), 32);

            root = hex::encode(&chunks[0]);
            // 64 hex chars = 32 bytes
            assert_eq!(root.len(), 64);

            return root;
        }
    }

    pub fn pad_bytes(start: usize, length: usize, serialized_state: &Vec<u8>) -> Vec<u8> {
        // start and stop idxs for vars in ssz serialized object
        let stop = start + length;
        assert!(stop < serialized_state.len(), "stop exceeds end of ssz obj");
        let var_as_bytes = &serialized_state[start..stop];

        if length == 32 {
            assert_eq!(var_as_bytes.len(), 32 as usize);
            let padded_var: Vec<u8> = var_as_bytes.to_vec();
            return padded_var;
        } else if length < 32 {
            assert!(var_as_bytes.len() < 32);
            let padded_var: Vec<u8> = pad_to_32(var_as_bytes, &length);
            return padded_var;
        } else {
            //else length > 32
            assert!(length > 32, "unexpected condition: length <= 32");
            assert!(var_as_bytes.len() > 32);
            assert_eq!(var_as_bytes.len(), length);

            if length % 32 == 0 {
                // if length > 32 and is multiple of 32
                let n_chunks: usize = length / 32;

                if n_chunks.is_power_of_two() {
                    let padded_var: Vec<u8> = var_as_bytes.to_vec();
                    assert!(padded_var.len().is_power_of_two());
                    assert!(padded_var.len() % 32 == 0);

                    return padded_var;
                } else {
                    // if length > 32 and multiple of 32
                    // but N chunks not a power of 2

                    let padded_var: Vec<u8> = pad_chunks_to_power2(var_as_bytes);
                    assert!(padded_var.len().is_power_of_two());
                    assert!(padded_var.len() % 32 == 0);
                    return padded_var;
                }
            } else {
                //length > 32 but not a multiple of 32
                let intermediate_var = pad_to_multiple_of_32(var_as_bytes, &length);

                if intermediate_var.len().is_power_of_two() {
                    return intermediate_var;
                } else {
                    let padded_var: Vec<u8> = pad_chunks_to_power2(&intermediate_var);
                    assert!(padded_var.len().is_power_of_two());
                    assert!(padded_var.len() % 32 == 0);
                    return padded_var;
                }
            }
        }
    }

    pub fn pad_to_32(var: &[u8], length: &usize) -> Vec<u8> {
        // takes ssz bytes and pads with zeros to 32 byte length

        let mut padded_var: Vec<u8> = vec![];
        let n_pad = 32 - length;
        let pad = vec![0u8; n_pad];

        for i in var.iter() {
            padded_var.push(*i);
        }

        padded_var.extend_from_slice(&pad);

        assert_eq!(padded_var.len(), 32);

        return padded_var;
    }

    pub fn pad_to_multiple_of_32(var: &[u8], length: &usize) -> Vec<u8> {
        // for vars with >1 chunk, pads with zeros to next multiple of 32 bytes

        let mut padded_var: Vec<u8> = vec![];
        let pad = vec![0u8; 1];
        let mut length_mut = length.clone();

        // add the var bytes to the new vec
        for i in var.iter() {
            padded_var.push(*i);
        }

        // add 0's to the vec until the length is a multiple of 32
        while length_mut % 32 != 0 {
            padded_var.extend_from_slice(&pad);
            length_mut += 1;
        }

        // make sure the length var == actual var length
        assert_eq!(padded_var.len(), length_mut.to_owned());

        return padded_var;
    }

    pub fn pad_chunks_to_power2(var: &[u8]) -> Vec<u8> {
        // for vars with multiple chunks, ensures number
        // of chunks is a power of 2

        assert!(var.len() % 32 == 0);
        let n_chunks: usize = var.len() / 32;

        // if N chunks is already a power of 2
        if n_chunks.is_power_of_two() {
            assert_eq!(var.len() / 32 % 2, 0);
            assert_eq!(var.len() % 32, 0);
            return var.to_vec();
        } else {
            // if N chunks is not a power of 2
            let chunks_to_add = n_chunks.next_power_of_two() - n_chunks;

            assert!(chunks_to_add != 0);
            assert!(var.len() % 32 == 0);

            let mut padded_var: Vec<u8> = vec![];

            let pad = vec![0u8; 32 * chunks_to_add];

            for i in var.iter() {
                padded_var.push(*i);
            }

            padded_var.extend_from_slice(&pad);

            let new_n_chunks: usize = padded_var.len() / 32;

            assert_eq!(padded_var.len() % 32, 0);
            assert!(new_n_chunks.is_power_of_two());
            assert!(padded_var.len().is_power_of_two());

            return padded_var;
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
    start_idx += sizes["genesis_validators_root"] - 1 as usize;
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

    let historical_roots = pad_bytes(
        offsets["historical_roots"],
        sizes["historical_roots"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&historical_roots);
    leaves.push(leaf_hash);

    let eth1_data_dep_root = pad_bytes(start_idx, sizes["eth1_data_dep_root"], &serialized_state);
    start_idx += sizes["eth1_data_dep_root"] as usize;
    let leaf_hash = hash(&eth1_data_dep_root);
    leaves.push(leaf_hash);

    let eth1_data_deposit_count = pad_bytes(
        start_idx,
        sizes["eth1_data_deposit_count"],
        &serialized_state,
    );
    start_idx += sizes["eth1_data_deposit_count"] as usize;
    let leaf_hash = hash(&eth1_data_deposit_count);
    leaves.push(leaf_hash);

    let eth1_data_block_hash =
        pad_bytes(start_idx, sizes["eth1_data_block_hash"], &serialized_state);
    start_idx += sizes["eth1_data_block_hash"] as usize;
    let leaf_hash = hash(&eth1_data_block_hash);
    leaves.push(leaf_hash);

    let eth1_data_votes = pad_bytes(
        offsets["eth1_data_votes"],
        sizes["eth1_data_votes"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&eth1_data_votes);
    leaves.push(leaf_hash);

    let eth1_deposit_index = pad_bytes(start_idx, sizes["eth1_deposit_index"], &serialized_state);
    start_idx += sizes["eth1_deposit_index"] as usize;
    let leaf_hash = hash(&eth1_deposit_index);
    leaves.push(leaf_hash);

    let validators = pad_bytes(
        offsets["validators"],
        sizes["validators"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&validators);
    leaves.push(leaf_hash);

    let balances = pad_bytes(offsets["balances"], sizes["balances"], &serialized_state);
    start_idx += 4;
    let leaf_hash = hash(&balances);
    leaves.push(leaf_hash);

    let randao_mixes = pad_bytes(start_idx, sizes["randao_mixes"], &serialized_state);
    start_idx += sizes["randao_mixes"] as usize;
    let leaf_hash = hash(&randao_mixes);
    leaves.push(leaf_hash);

    let slashings = pad_bytes(start_idx, sizes["slashings"], &serialized_state);
    start_idx += sizes["slashings"] as usize;
    let leaf_hash = hash(&slashings);
    leaves.push(leaf_hash);

    let previous_epoch_participation = pad_bytes(
        offsets["previous_epoch_participation"],
        sizes["previous_epoch_participation"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&previous_epoch_participation);
    leaves.push(leaf_hash);

    let current_epoch_participation = pad_bytes(
        offsets["current_epoch_participation"],
        sizes["current_epoch_participation"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&current_epoch_participation);
    leaves.push(leaf_hash);

    let justification_bits = pad_bytes(
        offsets["justification_bits"],
        sizes["justification_bits"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&justification_bits);
    leaves.push(leaf_hash);

    let prev_just_check_epoch =
        pad_bytes(start_idx, sizes["prev_just_check_epoch"], &serialized_state);
    start_idx += sizes["prev_just_check_epoch"] as usize;
    let leaf_hash = hash(&prev_just_check_epoch);
    leaves.push(leaf_hash);

    let prev_just_check_root =
        pad_bytes(start_idx, sizes["prev_just_check_root"], &serialized_state);
    start_idx += sizes["prev_just_check_root"] as usize;
    let leaf_hash = hash(&prev_just_check_root);
    leaves.push(leaf_hash);

    let curr_just_check_epoch =
        pad_bytes(start_idx, sizes["curr_just_check_epoch"], &serialized_state);
    start_idx += sizes["curr_just_check_epoch"] as usize;
    let leaf_hash = hash(&curr_just_check_epoch);
    leaves.push(leaf_hash);

    let curr_just_check_root =
        pad_bytes(start_idx, sizes["curr_just_check_root"], &serialized_state);
    start_idx += sizes["curr_just_check_root"] as usize;
    let leaf_hash = hash(&curr_just_check_root);
    leaves.push(leaf_hash);

    let finalized_check_epoch =
        pad_bytes(start_idx, sizes["finalized_check_epoch"], &serialized_state);
    start_idx += sizes["finalized_check_epoch"] as usize;
    let leaf_hash = hash(&finalized_check_epoch);
    leaves.push(leaf_hash);

    let finalized_checkpoint_root = pad_bytes(
        start_idx,
        sizes["finalized_checkpoint_root"],
        &serialized_state,
    );
    start_idx += sizes["finalized_checkpoint_root"] as usize;
    let leaf_hash = hash(&finalized_checkpoint_root);
    leaves.push(leaf_hash);

    let inactivity_scores = pad_bytes(
        offsets["inactivity_scores"],
        sizes["inactivity_scores"],
        &serialized_state,
    );
    start_idx += 4;
    let leaf_hash = hash(&inactivity_scores);
    leaves.push(leaf_hash);

    let curr_sync_comm_pubkeys = pad_bytes(
        start_idx,
        sizes["curr_sync_comm_pubkeys"],
        &serialized_state,
    );
    start_idx += sizes["curr_sync_comm_pubkeys"] as usize;
    let leaf_hash = hash(&curr_sync_comm_pubkeys);
    leaves.push(leaf_hash);

    let curr_sync_comm_agg_pubkey = pad_bytes(
        start_idx,
        sizes["curr_sync_comm_agg_pubkey"],
        &serialized_state,
    );
    start_idx += sizes["curr_sync_comm_agg_pubkey"] as usize;
    let leaf_hash = hash(&curr_sync_comm_agg_pubkey);
    leaves.push(leaf_hash);

    let next_sync_comm_pubkeys = pad_bytes(
        start_idx,
        sizes["next_sync_comm_pubkeys"],
        &serialized_state,
    );
    start_idx += sizes["next_sync_comm_pubkeys"] as usize;
    let leaf_hash = hash(&next_sync_comm_pubkeys);
    leaves.push(leaf_hash);

    let next_sync_comm_agg_pubkey = pad_bytes(
        start_idx,
        sizes["next_sync_comm_agg_pubkey"],
        &serialized_state,
    );
    let leaf_hash = hash(&next_sync_comm_agg_pubkey);
    leaves.push(leaf_hash);

    // there should always be 37 fields, so 37 hashes
    assert_eq!(leaves.len(), 37);

    // OPTIONAL PRINT EACH LEAF HASH (ROOTS FOR MULTICHUNK VARS)
    // println!("\nHASHES FOR EACH FIELD IN STATE:\n");
    // for i in leaves.iter() {
    //     if i == "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925" {
    //         println!(" {:?} (zero hash)", i);
    //     } else {
    //         println!("{:?}", i);
    //     }
    // }
    return leaves;
}

pub fn build_tree(leaves: Vec<String>) -> Vec<Vec<String>>{
    // firt add zero leaves until N leaves is a power of 2
    let mut padded_leaves: Vec<String> = vec![];
    for i in leaves.iter() {
        padded_leaves.push(i.to_string());
    }

    let leaves_to_add: usize = leaves.len().next_power_of_two() - leaves.len();

    let zeros: Vec<u8> = vec![0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(zeros);
    let result = hasher.finalize_reset();
    let result = hex::encode(result);

    for i in (0..leaves_to_add) {
        padded_leaves.push(result.clone());
    }

    assert!(padded_leaves.len() == 64);
    assert!(padded_leaves[0].len() == 64);

    // build a tree that is a vector of vectors of strings
    // each sub-vector will be a layer in the tree
    // each string is a hex encoded hash (i.e. a node)
    let mut tree: Vec<Vec<String>> = vec![];
    let mut leaves_to_hash: Vec<String> = padded_leaves.clone();

    tree.push(padded_leaves);
    while leaves_to_hash.len() > 1 {
        let mut new_nodes: Vec<String> = vec![];
        for i in (0..leaves_to_hash.len()).step_by(2) {
            let mut hasher = Sha256::new();
            hasher.update(&leaves_to_hash[i]);
            hasher.update(&leaves_to_hash[i + 1]);
            let result = hasher.finalize_reset();
            let result = hex::encode(result);
            new_nodes.push(result)
        }
        
        leaves_to_hash = new_nodes.clone();
        tree.push(new_nodes);
    }

    println!("\n*** MERKLE TREE PROPERTIES ***\n");
    println!("N LAYERS: {:?}\n",tree.len());
    println!("Lengths should decrease in powers of 2 from leaves to root");
    println!("LAYER 1 LEN: {:?}",tree[0].len());
    println!("LAYER 2 LEN: {:?}",tree[1].len());
    println!("LAYER 3 LEN: {:?}",tree[2].len());
    println!("LAYER 4 LEN: {:?}",tree[3].len());
    println!("LAYER 5 LEN: {:?}",tree[4].len());
    println!("LAYER 6 LEN: {:?}",tree[5].len());
    println!("LAYER 7 LEN: {:?}",tree[6].len());
    println!("STATE_ROOT: {:?}\n", tree[6]);

    return tree;
}
