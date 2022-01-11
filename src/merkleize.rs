extern crate hex;
use bit_vec::BitVec;
use bitvec::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub fn calculate_leaves(
    serialized_state: &Vec<u8>,
    sizes: &HashMap<&str, usize>,
    offsets: &HashMap<&str, usize>,
) -> Vec<String> {
    // sha256 hashes vecs of bytes from serialized object
    // mixes in length data as per spec

    let mut leaves = vec![];
    let mut start_idx: usize = 0;

    pub fn hash(leaf: &Vec<u8>, length: &usize) -> String {
        // we need a bytes representation (length 32) of
        // the var length to "mix_in_length" later
        let length_bytes = length.to_le_bytes();
        let length_bytes = pad_to_32(&length_bytes, &length_bytes.len());

        assert_eq!(length_bytes.len(), 32);
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

            // mix in length data
            let mut hasher = Sha256::new();
            hasher.update(&chunks[0]);
            hasher.update(length_bytes);
            let hash = hasher.finalize_reset().to_vec();

            let root = hex::encode(hash);

            // 64 hex chars = 32 bytes
            assert_eq!(root.len(), 64);

            return root;
        }
    }

    pub fn pad_bytes(start: usize, length: usize, serialized_state: &Vec<u8>) -> Vec<u8> {
        // start and stop idxs for vars in ssz serialized object
        let stop = start + length;
        let var_as_bytes = &serialized_state[start..stop];

        //check lengths are consistent
        assert!(stop - start == length);
        assert!(
            stop <= serialized_state.len(),
            "stop {:?} exceeds end of ssz obj",
            stop
        );
        assert_eq!(length, stop - start);
        assert_eq!(length, var_as_bytes.len());
        println!("{:?}", start);

        if length == 32 {
            assert_eq!(var_as_bytes.len(), 32 as usize);
            let padded_var: Vec<u8> = var_as_bytes.to_vec();

            return padded_var;
        } else if length < 32 {
            assert!(var_as_bytes.len() < 32);
            let padded_var: Vec<u8> = pad_to_32(var_as_bytes, &length);
            assert_eq!(padded_var.len(), 32);
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

    let leaf_hash = hash(&genesis_time, &sizes["genesis_time"]);
    leaves.push(leaf_hash);

    let genesis_validators_root = pad_bytes(
        start_idx,
        sizes["genesis_validators_root"],
        &serialized_state,
    );
    start_idx += sizes["genesis_validators_root"] as usize;

    let leaf_hash = hash(&genesis_validators_root, &sizes["genesis_validators_root"]);
    leaves.push(leaf_hash);

    let slot = pad_bytes(start_idx, sizes["slot"], &serialized_state);
    start_idx += sizes["slot"] as usize;

    let leaf_hash = hash(&slot, &sizes["slot"]);
    leaves.push(leaf_hash);

    let fork_prev_ver = pad_bytes(start_idx, sizes["fork_prev_ver"], &serialized_state);
    start_idx += sizes["fork_prev_ver"] as usize;

    let leaf_hash = hash(&fork_prev_ver, &sizes["slot"]);
    leaves.push(leaf_hash);

    let fork_curr_ver = pad_bytes(start_idx, sizes["fork_curr_ver"], &serialized_state);
    start_idx += sizes["fork_curr_ver"] as usize;

    let leaf_hash = hash(&fork_curr_ver, &sizes["fork_curr_ver"]);
    leaves.push(leaf_hash);

    let fork_epoch = pad_bytes(start_idx, sizes["fork_epoch"], &serialized_state);
    start_idx += sizes["fork_epoch"] as usize;

    let leaf_hash = hash(&fork_epoch, &sizes["fork_epoch"]);
    leaves.push(leaf_hash);

    let header_slot = pad_bytes(start_idx, sizes["header_slot"], &serialized_state);
    start_idx += sizes["header_slot"] as usize;

    let leaf_hash = hash(&header_slot, &sizes["header_slot"]);
    leaves.push(leaf_hash);

    let header_proposer_index =
        pad_bytes(start_idx, sizes["header_proposer_index"], &serialized_state);
    start_idx += sizes["header_proposer_index"] as usize;

    let leaf_hash = hash(&header_proposer_index, &sizes["header_proposer_index"]);
    leaves.push(leaf_hash);

    let header_parent_root = pad_bytes(start_idx, sizes["header_parent_root"], &serialized_state);
    start_idx += sizes["header_parent_root"] as usize;

    let leaf_hash = hash(&header_parent_root, &sizes["header_parent_root"]);
    leaves.push(leaf_hash);

    let header_state_root = pad_bytes(start_idx, sizes["header_state_root"], &serialized_state);
    start_idx += sizes["header_state_root"] as usize;

    let leaf_hash = hash(&header_state_root, &sizes["header_state_root"]);
    leaves.push(leaf_hash);

    let header_body_root = pad_bytes(start_idx, sizes["header_body_root"], &serialized_state);
    start_idx += sizes["header_body_root"] as usize;

    let leaf_hash = hash(&header_body_root, &sizes["header_body_root"]);
    leaves.push(leaf_hash);

    let block_roots = pad_bytes(start_idx, sizes["block_roots"], &serialized_state);
    start_idx += sizes["block_roots"] as usize;

    let leaf_hash = hash(&block_roots, &sizes["block_roots"]);
    leaves.push(leaf_hash);

    let state_roots = pad_bytes(start_idx, sizes["state_roots"], &serialized_state);
    start_idx += sizes["state_roots"] as usize;

    let leaf_hash = hash(&state_roots, &sizes["state_roots"]);
    leaves.push(leaf_hash);

    let historical_roots = pad_bytes(
        offsets["historical_roots"],
        sizes["historical_roots"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(&historical_roots, &sizes["historical_roots"]);
    leaves.push(leaf_hash);

    let eth1_data_dep_root = pad_bytes(start_idx, sizes["eth1_data_dep_root"], &serialized_state);
    start_idx += sizes["eth1_data_dep_root"] as usize;
    let leaf_hash = hash(&eth1_data_dep_root, &sizes["eth1_data_dep_root"]);
    leaves.push(leaf_hash);

    let eth1_data_deposit_count = pad_bytes(
        start_idx,
        sizes["eth1_data_deposit_count"],
        &serialized_state,
    );
    start_idx += sizes["eth1_data_deposit_count"] as usize;

    let leaf_hash = hash(&eth1_data_deposit_count, &sizes["eth1_data_deposit_count"]);
    leaves.push(leaf_hash);

    let eth1_data_block_hash =
        pad_bytes(start_idx, sizes["eth1_data_block_hash"], &serialized_state);
    start_idx += sizes["eth1_data_block_hash"] as usize;

    let leaf_hash = hash(&eth1_data_block_hash, &sizes["eth1_data_block_hash"]);
    leaves.push(leaf_hash);

    let eth1_data_votes = pad_bytes(
        offsets["eth1_data_votes"],
        sizes["eth1_data_votes"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(&eth1_data_votes, &sizes["eth1_data_votes"]);
    leaves.push(leaf_hash);

    let eth1_deposit_index = pad_bytes(start_idx, sizes["eth1_deposit_index"], &serialized_state);
    start_idx += sizes["eth1_deposit_index"] as usize;

    let leaf_hash = hash(&eth1_deposit_index, &sizes["eth1_deposit_index"]);
    leaves.push(leaf_hash);

    let validators = pad_bytes(
        offsets["validators"],
        sizes["validators"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(&validators, &sizes["validators"]);
    leaves.push(leaf_hash);

    let balances = pad_bytes(offsets["balances"], sizes["balances"], &serialized_state);
    start_idx += 4;

    let leaf_hash = hash(&balances, &sizes["balances"]);
    leaves.push(leaf_hash);

    let randao_mixes = pad_bytes(start_idx, sizes["randao_mixes"], &serialized_state);
    start_idx += sizes["randao_mixes"] as usize;

    let leaf_hash = hash(&randao_mixes, &sizes["randao_mixes"]);
    leaves.push(leaf_hash);

    let slashings = pad_bytes(start_idx, sizes["slashings"], &serialized_state);
    start_idx += sizes["slashings"] as usize;

    let leaf_hash = hash(&slashings, &sizes["slashings"]);
    leaves.push(leaf_hash);

    let previous_epoch_participation = pad_bytes(
        offsets["previous_epoch_participation"],
        sizes["previous_epoch_participation"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(
        &previous_epoch_participation,
        &sizes["previous_epoch_participation"],
    );
    leaves.push(leaf_hash);

    let current_epoch_participation = pad_bytes(
        offsets["current_epoch_participation"],
        sizes["current_epoch_participation"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(
        &current_epoch_participation,
        &sizes["current_epoch_participation"],
    );
    leaves.push(leaf_hash);

    let justification_bits = pad_bytes(start_idx, sizes["justification_bits"], &serialized_state);
    start_idx += sizes["justification_bits"] as usize;

    let justification_bits = remove_cap_from_justification_bits(&justification_bits);
    let leaf_hash = hash(&justification_bits, &sizes["justification_bits"]);
    leaves.push(leaf_hash);

    let prev_just_check_epoch =
        pad_bytes(start_idx, sizes["prev_just_check_epoch"], &serialized_state);
    start_idx += sizes["prev_just_check_epoch"] as usize;

    let leaf_hash = hash(&prev_just_check_epoch, &sizes["prev_just_check_epoch"]);
    leaves.push(leaf_hash);

    let prev_just_check_root =
        pad_bytes(start_idx, sizes["prev_just_check_root"], &serialized_state);
    start_idx += sizes["prev_just_check_root"] as usize;

    let leaf_hash = hash(&prev_just_check_root, &sizes["prev_just_check_root"]);
    leaves.push(leaf_hash);

    let curr_just_check_epoch =
        pad_bytes(start_idx, sizes["curr_just_check_epoch"], &serialized_state);
    start_idx += sizes["curr_just_check_epoch"] as usize;

    let leaf_hash = hash(&curr_just_check_epoch, &sizes["curr_just_check_epoch"]);
    leaves.push(leaf_hash);

    let curr_just_check_root =
        pad_bytes(start_idx, sizes["curr_just_check_root"], &serialized_state);
    start_idx += sizes["curr_just_check_root"] as usize;

    let leaf_hash = hash(&curr_just_check_root, &sizes["curr_just_check_root"]);
    leaves.push(leaf_hash);

    let finalized_check_epoch =
        pad_bytes(start_idx, sizes["finalized_check_epoch"], &serialized_state);
    start_idx += sizes["finalized_check_epoch"] as usize;

    let leaf_hash = hash(&finalized_check_epoch, &sizes["finalized_check_epoch"]);
    leaves.push(leaf_hash);

    let finalized_checkpoint_root = pad_bytes(
        start_idx,
        sizes["finalized_checkpoint_root"],
        &serialized_state,
    );
    start_idx += sizes["finalized_checkpoint_root"] as usize;

    let leaf_hash = hash(
        &finalized_checkpoint_root,
        &sizes["finalized_checkpoint_root"],
    );
    leaves.push(leaf_hash);

    let inactivity_scores = pad_bytes(
        offsets["inactivity_scores"],
        sizes["inactivity_scores"],
        &serialized_state,
    );
    start_idx += 4;

    let leaf_hash = hash(&inactivity_scores, &sizes["inactivity_scores"]);
    leaves.push(leaf_hash);

    let curr_sync_comm_pubkeys = pad_bytes(
        start_idx,
        sizes["curr_sync_comm_pubkeys"],
        &serialized_state,
    );
    start_idx += sizes["curr_sync_comm_pubkeys"] as usize;

    let leaf_hash = hash(&curr_sync_comm_pubkeys, &sizes["curr_sync_comm_pubkeys"]);
    leaves.push(leaf_hash);

    let curr_sync_comm_agg_pubkey = pad_bytes(
        start_idx,
        sizes["curr_sync_comm_agg_pubkey"],
        &serialized_state,
    );
    start_idx += sizes["curr_sync_comm_agg_pubkey"] as usize;

    let leaf_hash = hash(
        &curr_sync_comm_agg_pubkey,
        &sizes["curr_sync_comm_agg_pubkey"],
    );
    leaves.push(leaf_hash);

    let next_sync_comm_pubkeys = pad_bytes(
        start_idx,
        sizes["next_sync_comm_pubkeys"],
        &serialized_state,
    );
    start_idx += sizes["next_sync_comm_pubkeys"] as usize;

    let leaf_hash = hash(&next_sync_comm_pubkeys, &sizes["next_sync_comm_pubkeys"]);
    leaves.push(leaf_hash);

    let next_sync_comm_agg_pubkey = pad_bytes(
        start_idx,
        sizes["next_sync_comm_agg_pubkey"],
        &serialized_state,
    );
    start_idx += sizes["next_sync_comm_agg_pubkey"] as usize;
    let leaf_hash = hash(
        &next_sync_comm_agg_pubkey,
        &sizes["next_sync_comm_agg_pubkey"],
    );
    leaves.push(leaf_hash);

    // there should always be 37 fields, so 37 hashes
    // the start_idx should, after tracking the fixed-length vars
    // equal the fixed-parts length calculated during serialization
    assert_eq!(leaves.len(), 37);
    assert_eq!(
        start_idx, sizes["fixed_parts"],
        "error: serialization and deserialization have not syncd correctly"
    );

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

pub fn build_tree(leaves: Vec<String>) -> Vec<Vec<String>> {
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

    // there should be 64 leaves, and the length
    // of erach leaf should eb 64 chars

    assert!(padded_leaves.len() == 64);
    assert!(padded_leaves[0].len() == 64);

    // build a tree that is a vector of vectors of strings
    // each sub-vector will be a layer in the tree
    // each string is a hex encoded hash (i.e. a node)
    let mut tree: Vec<Vec<String>> = vec![];
    let mut leaves_to_hash: Vec<String> = padded_leaves.clone();

    tree.push(padded_leaves);

    // take pairs of leaves and hash them together,
    // appending the resulting hash to the next layer
    // in the tree. Do this sequentially for each layer
    // until there is just one hash (==root)
    let mut layer: usize = 6; //just for logging to console
    println!("\n***BUILDING MERKLE TREE\n");
    println!("initial n leaves = {:?}", leaves.len());
    println!(
        "adding {:?} zero chunks to give {:?} leaves\n",
        leaves_to_add,
        leaves.len().next_power_of_two()
    );
    while leaves_to_hash.len() > 1 {
        let mut new_nodes: Vec<String> = vec![];
        //println!("\nHASHING IN LAYER {:?}\n", layer);
        // count through leaves in steps of 2
        for i in (0..leaves_to_hash.len()).step_by(2) {
            //println!("hashing leaves {:?} with {:?}", i, i + 1);
            let mut hasher = Sha256::new();
            hasher.update(&leaves_to_hash[i]);
            hasher.update(&leaves_to_hash[i + 1]);
            let result = hasher.finalize_reset();
            let result = hex::encode(result);
            new_nodes.push(result)
        }
        layer -= 1;
        leaves_to_hash = new_nodes.clone();
        tree.push(new_nodes);
    }
    // println!("\nFINISHED BUILDING MERKLE TREE");
    // println!("\n*** MERKLE TREE PROPERTIES ***\n");
    // println!("N LAYERS IN TREE: {:?}\n", tree.len());
    // println!("Lengths should decrease in powers of 2 from leaves to root");
    // println!("N LEAVES LAYER 7: {:?}", tree[0].len());
    // println!("N NODES LAYER 6: {:?}", tree[1].len());
    // println!("N NODES LAYER 5: {:?}", tree[2].len());
    // println!("N NODES LAYER 4: {:?}", tree[3].len());
    // println!("N NODES LAYER 3: {:?}", tree[4].len());
    // println!("N NODES LAYER 2: {:?}", tree[5].len());
    // println!("N NODES LAYER 1: {:?}", tree[6].len());
    println!("STATE_ROOT: {:?}\n", tree[6]);

    return tree;
}

pub fn remove_cap_from_justification_bits(justification_bits: &Vec<u8>) -> Vec<u8> {
    
    let mut bits: BitVec = BitVec::from_bytes(&justification_bits);

    let mut counter: usize = 0;

    for i in (0..bits.len()).rev() {
        if bits[i] == true {
            println!("erasing length cap from justification bits idx:{:?}", i);
            bits.set(i, false);
            break;
        } else if i == 0 && bits[i] == false {
            bits.set(i, true);
            // if we get to the end of the bits
            // without finding a 1, exit loops
            break;
        }
    }

    let bytes: Vec<u8> = bits.to_bytes();
    println!("{:?}", bytes);
    return bytes;
}
