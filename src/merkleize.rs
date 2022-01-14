extern crate hex;
use bit_vec::BitVec;
use bitvec::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;



pub fn get_hash_root(leaf: &Vec<u8>) -> String {
    
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

        let root = hex::encode(&chunks[0]);
        // 64 hex chars = 32 bytes
        assert_eq!(root.len(), 64);

        return root;
    }
}

pub fn mix_in_length_data(root: &str, length: &usize) -> String {
    // mix in length data
    // we need a bytes representation (length 32) of
    // the var length to "mix_in_length" later
    let length_bytes = length.to_le_bytes();
    let length_bytes = pad_to_32(&length_bytes, &length_bytes.len());

    // make sure the length that is mixed in is a 32 byte vec
    // and that the leaf is at least 32 bytes and always a multiple
    // of 32 bytes
    assert_eq!(length_bytes.len(), 32);
    let mut hasher = Sha256::new();
    hasher.update(root);
    hasher.update(length_bytes);
    let hash = hasher.finalize_reset().to_vec();

    let root = hex::encode(hash);
    root
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

pub fn calculate_leaves(
    serialized_state: &Vec<u8>,
    sizes: &HashMap<&str, usize>,
    offsets: &HashMap<&str, usize>,
) -> Vec<String> {
    // sha256 hashes vecs of bytes from serialized object
    // mixes in length data as per spec

    let mut leaves = vec![];
    let mut start_idx: usize = 0;

    let keys: Vec<&str> = vec![
        "genesis_time",
        "genesis_validators_root",
        "slot",
        "fork_prev_ver",
        "fork_curr_ver",
        "fork_epoch",
        "header_slot",
        "header_proposer_index",
        "header_parent_root",
        "header_state_root",
        "header_body_root",
        "block_roots",
        "state_roots",
        "historical_roots",
        "eth1_data_dep_root",
        "eth1_data_deposit_count",
        "eth1_data_block_hash",
        "eth1_data_votes",
        "eth1_deposit_index",
        "validators",
        "balances",
        "randao_mixes",
        "slashings",
        "previous_epoch_participation",
        "current_epoch_participation",
        "justification_bits",
        "prev_just_check_epoch",
        "prev_just_check_root",
        "curr_just_check_epoch",
        "curr_just_check_root",
        "finalized_check_epoch",
        "finalized_checkpoint_root",
        "inactivity_scores",
        "curr_sync_comm_pubkeys",
        "curr_sync_comm_agg_pubkey",
        "next_sync_comm_pubkeys",
        "next_sync_comm_agg_pubkey",
    ];

    for key in keys.iter() {
        let var = pad_bytes(start_idx, sizes[key], &serialized_state);
        if offsets.contains_key(key) {
            start_idx += 4;
        } else {
            start_idx += sizes[key];
        };

        let root = get_hash_root(&var);
        let root = mix_in_length_data(&root, &sizes[key]);
        leaves.push(root);
    }

        return leaves
    
}

pub fn build_tree(leaves: Vec<String>) -> Vec<Vec<String>> {
    // first add data leaves, then pad with
    // zero leaves until N leaves is a power of 2
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
    // of each leaf should be 64 chars

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
        leaves_to_hash = new_nodes.clone();

        tree.push(new_nodes);
    }
    // println!("\nFINISHED BUILDING MERKLE TREE");
    // println!("\n*** MERKLE TREE PROPERTIES ***\n");
    // println!("N LAYERS IN TREE: {:?}\n", tree.len());

    println!("CALCULATED STATE_ROOT: {:?}\n", tree[6]);

    return tree;
}

pub fn remove_cap_from_justification_bits(justification_bits: &Vec<u8>) -> Vec<u8> {
    let mut bits: BitVec = BitVec::from_bytes(&justification_bits);
    println!("\njustification bits with length-cap\n{:?}\n", bits);
    for i in (0..bits.len()).rev() {
        if bits[i] == true {
            println!(
                "\nremoving length cap from justification bits idx:{:?}\n",
                i
            );
            bits.set(i, false);
            break;
        }
    }
    println!("\njustification bits without length-cap\n{:?}\n", bits);
    let bytes: Vec<u8> = bits.to_bytes();
    println!("\njustification bits as bytes\n{:?}\n", bytes);
    return bytes;
}
