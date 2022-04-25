extern crate hex;
use crate::constants::{
    BYTES_PER_LENGTH_OFFSET, EPOCHS_PER_ETH1_VOTING_PERIOD, EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR, HISTORICAL_ROOTS_LIMIT, SLOTS_PER_EPOCH,
    SLOTS_PER_HISTORICAL_ROOT, VALIDATOR_REGISTRY_LIMIT,
};
use bit_vec::BitVec;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// control flow here is:
// 1) generate chunks() (returns vec of root hashes, one per leaf)
//       calls out to get_var_bytes() which grabs the bytes representing each var from the serialized state
//       calls out to pack (N chunks of 32 bytes where N == power of 2)
//           if var is justification_bits the length cap is removed first
//           calls out to pad_to_32 (if single chunk needs padding)
//           calls out to pad_to_multiple_of_32 (if multi-chunk var needs padding)
//           calls out to chunks_to_power_of_two (if N chunks is not power of 2)
//       calls out to get_hash_root() (returns root hash of single or multichunk var)
//       calls out to mix_in_length_data() (hashes root with le_bytes representation of the var length)
// 2) merkle_tree() (returns merkle tree as Vec<Vec<String>> where vec[i+1].len() = vec[i].len()/2)

// 1) and 2) called in sequence returns a merkle tree representation of beacon_state where
// tree[6] is the state root.

pub fn generate_chunks(
    serialized_state: &Vec<u8>,
    sizes: &HashMap<&str, usize>,
    offsets: &HashMap<&str, usize>,
) -> Vec<u8> {
    // sha256 hashes vecs of bytes from serialized object
    // mixes in length data as per spec and
    // returns vec of leaves

    let mut chunks: Vec<u8> = vec![];
    let mut start_idx: usize = 0;

    let keys: Vec<&str> = vec![
        "genesis_time",
        "genesis_validators_root",
        "slot",
        "fork",
        "latest_block_header",
        "block_roots",
        "state_roots",
        "historical_roots",
        "eth1_data",
        "eth1_data_votes",
        "eth1_deposit_index",
        "validators",
        "balances",
        "randao_mixes",
        "slashings",
        "previous_epoch_participation",
        "current_epoch_participation",
        "justification_bits",
        "previous_justified_checkpoint",
        "current_justified_checkpoint",
        "finalized_checkpoint",
        "inactivity_scores",
        "current_sync_committee",
        "next_sync_committee",
    ];

    let containers: Vec<&str> = vec![
        "fork",
        "eth1_data",
        "latest_block_header",
        "previous_justified_checkpoint",
        "current_justified_checkpoint",
        "finalized_checkpoint",
        "current_sync_committee",
        "next_sync_committee",
    ];

    for key in keys.iter() {
        // if var is justification bits remove the end cap
        let mut bit_flag = false;

        if key == &"justification_bits" {
            bit_flag = true;
        } else {
            bit_flag = false;
        }

       
       
        let var = get_var_bytes(start_idx, sizes[key], &serialized_state, &bit_flag);
        assert_eq!(var.len(), sizes[key]);


        let var: Vec<u8> = pack(var);
        let mut root = hash_tree_root(&var);
        //if var is a container then get the container root
        if containers.contains(key) {
            let var: Vec<u8> = pack(var);
            root = hash_tree_root_container(key, var, sizes);
        } else {
            let var: Vec<u8> = pack(var);
            root = hash_tree_root(&var);
        }


        // mix in length data, push root to chunks vec
        // only if the var type is list
        if (key == &"historical_roots")
            | (key == &"validators")
            | (key == &"balances")
            | (key == &"eth1_data_votes")
            | (key == &"previous_epoch_participation")
            | (key == &"current_epoch_participation")
        {
            root = mix_in_length_data(&root, &sizes[key]);
        }

        chunks.append(&mut root);

        // advance start_idx to the end of the var just deserialized
        // or end of offset for a variable-length var
        if offsets.contains_key(key) {
            start_idx += BYTES_PER_LENGTH_OFFSET;
        } else {
            start_idx += sizes[key];
        };
    }

    return chunks;
}

pub fn hash_tree_root_container(key: &str, var: Vec<u8>, sizes: &HashMap<&str, usize>) -> Vec<u8> {
    if key == "fork" {
        println!("HASHING FORK");
        let mut fork_previous_version = pack(var[0..sizes["fork_previous_version"]].to_vec());
        let mut fork_current_version = pack(
            var[sizes["fork_previous_version"]
                ..sizes["fork_previous_version"] + sizes["fork_current_version"]]
                .to_vec(),
        );
        let mut fork_epoch =
            pack(var[sizes["fork_previous_version"] + sizes["fork_current_version"]..].to_vec());

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut fork_previous_version);
        chunks.append(&mut fork_current_version);
        chunks.append(&mut fork_epoch);

        let chunks: Vec<u8> = pack(chunks);
        let root: Vec<u8> = hash_tree_root(&chunks);

        return root;
    } else if key == "eth1_data" {
        println!("HASHING ETH1DATA");
        let mut deposit_root = pack(var[0..sizes["eth1_deposit_root"]].to_vec());
        let mut deposit_count = pack(
            var[sizes["eth1_deposit_count"]
                ..sizes["eth1_deposit_count"] + sizes["eth1_deposit_root"]]
                .to_vec(),
        );
        let mut block_hash =
            pack(var[sizes["eth1_deposit_count"] + sizes["eth1_deposit_root"]..].to_vec());

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut deposit_root);
        chunks.append(&mut deposit_count);
        chunks.append(&mut block_hash);

        let chunks = pack(chunks);

        let root = hash_tree_root(&chunks);

        return root;
    } else if key == "latest_block_header" {
        println!("HASHING LATEST BLOCK HEADER");
        let mut slot = pack(var[0..sizes["header_slot"]].to_vec());
        let mut proposer_index = pack(
            var[sizes["header_slot"]..sizes["header_slot"] + sizes["header_proposer_index"]]
                .to_vec(),
        );
        let mut parent_root = pack(
            var[sizes["header_slot"] + sizes["header_proposer_index"]
                ..sizes["header_slot"]
                    + sizes["header_proposer_index"]
                    + sizes["header_parent_root"]]
                .to_vec(),
        );
        let mut state_root = pack(
            var[sizes["header_slot"] + sizes["header_proposer_index"] + sizes["header_parent_root"]
                ..sizes["header_slot"]
                    + sizes["header_proposer_index"]
                    + sizes["header_parent_root"]
                    + sizes["header_state_root"]]
                .to_vec(),
        );
        let mut body_root = pack(
            var[sizes["header_slot"]
                + sizes["header_proposer_index"]
                + sizes["header_parent_root"]
                + sizes["header_state_root"]
                ..sizes["header_slot"]
                    + sizes["header_proposer_index"]
                    + sizes["header_parent_root"]
                    + sizes["header_state_root"]
                    + sizes["header_body_root"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut slot);
        chunks.append(&mut proposer_index);
        chunks.append(&mut parent_root);
        chunks.append(&mut state_root);
        chunks.append(&mut body_root);
        chunks.append(&mut parent_root);
        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    } else if key == "previous_justified_checkpoint" {
        println!("HASHING PREVIOUS CHECKPOINT");
        let mut epoch = pack(var[0..sizes["previous_checkpoint_epoch"]].to_vec());
        let mut _root = pack(
            var[sizes["previous_checkpoint_epoch"]
                ..sizes["previous_checkpoint_epoch"] + sizes["previous_checkpoint_root"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut epoch);
        chunks.append(&mut _root);

        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    } else if key == "current_justified_checkpoint" {
        println!("HASHING CURRENT CHECKPOINT");
        let mut epoch = pack(var[0..sizes["current_checkpoint_epoch"]].to_vec());
        let mut _root = pack(
            var[sizes["current_checkpoint_epoch"]
                ..sizes["current_checkpoint_epoch"] + sizes["current_checkpoint_root"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut epoch);
        chunks.append(&mut _root);

        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    } else if key == "finalized_checkpoint" {
        println!("HASHING FINALIZED CHECKPOINT");

        let mut epoch = pack(var[0..sizes["finalized_checkpoint_epoch"]].to_vec());
        let mut _root = pack(
            var[sizes["finalized_checkpoint_epoch"]
                ..sizes["finalized_checkpoint_epoch"] + sizes["finalized_checkpoint_root"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut epoch);
        chunks.append(&mut _root);

        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    } else if key == "current_sync_committee" {
        let mut pubkeys = pack(var[0..sizes["current_sync_committee_pubkeys"]].to_vec());
        let mut agg_pubkey = pack(
            var[sizes["current_sync_committee_pubkeys"]
                ..sizes["current_sync_committee_pubkeys"]
                    + sizes["current_sync_committee_agg_pubkey"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut pubkeys);
        chunks.append(&mut agg_pubkey);

        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    } else {
        assert!(key == "next_sync_committee");
        let mut pubkeys = pack(var[0..sizes["next_sync_committee_pubkeys"]].to_vec());
        let mut agg_pubkey = pack(
            var[sizes["next_sync_committee_pubkeys"]
                ..sizes["next_sync_committee_pubkeys"] + sizes["next_sync_committee_agg_pubkey"]]
                .to_vec(),
        );

        let mut chunks: Vec<u8> = vec![];
        chunks.append(&mut pubkeys);
        chunks.append(&mut agg_pubkey);

        let chunks = pack(chunks);
        let root = hash_tree_root(&chunks);

        return root;
    }
}

pub fn merkle_tree(chunks: Vec<u8>) -> Vec<Vec<u8>> {
    // take vec<u8> which is organised into one long
    // continuous vec of bytes, where every 32 bytes is a chunk
    // take these 32 bytes, hash them together and add them
    // in order to the tree
    // the tree will be a continuous vector too, with 32 bytes
    // per node

    let chunks = pack(chunks);
    let chunks: Vec<Vec<u8>> = chunks.chunks(32).map(|x| x.to_vec()).collect();

    let mut chunks_temp: Vec<Vec<u8>> = chunks.clone();
    let mut tree: Vec<Vec<u8>> = vec![];

    for i in chunks_temp.iter() {
        tree.push(i.to_vec());
    }

    while chunks_temp.len() > 1 {
        let mut new_nodes: Vec<Vec<u8>> = vec![];

        for i in (0..chunks_temp.len()).step_by(2) {
            let mut hasher = Sha256::new();
            hasher.update(&chunks_temp[i]);
            hasher.update(&chunks_temp[i + 1]);
            let result = hasher.finalize_reset();
            new_nodes.push(result.to_vec());
        }

        chunks_temp = new_nodes;
        for node in chunks_temp.iter() {
            tree.push(node.to_vec());
        }
    }

    // want root at start of vec to
    // enable generalized index calcs
    tree.reverse();
    println!("{:?}", hex::encode(&tree[0]));
    return tree;
}

pub fn hash_tree_root(leaf: &Vec<u8>) -> Vec<u8> {
    assert!(leaf.len() >= 32);
    assert_eq!(leaf.len() % 32, 0);

    // first, if single leaf just return it
    if leaf.len() == 32 {
        return leaf.to_vec();
    } else {
        // here we deal with multiple chunks
        // by recursively hashing pairs
        // and returning the root

        let chunked_leaf: Vec<Vec<u8>> = leaf.chunks(32).map(|s| s.into()).collect();
        assert!(leaf.len() > 32);
        assert!(leaf.len() % 32 == 0);
        assert!(chunked_leaf.len() == leaf.len() / 32);

        let mut chunks = chunked_leaf;

        // iterate through pairs of chunks
        // creating new vec of parent nodes
        // hash those together and repeat until
        // one root left
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

        let root = &chunks[chunks.len() - 1];
        // 64 hex chars = 32 bytes
        assert_eq!(root.len(), 32);

        return root.to_owned();
    }
}

pub fn mix_in_length_data(root: &Vec<u8>, length: &usize) -> Vec<u8> {
    // mix in length data
    // we need a bytes representation (length 32) of
    // the var length to "mix_in_length" later
    let length_bytes = length.to_le_bytes();
    let length_bytes = pad_to_32(&length_bytes);

    // make sure the length that is mixed in is a 32 byte vec
    // and that the leaf is at least 32 bytes and always a multiple
    // of 32 bytes
    assert_eq!(length_bytes.len(), 32);
    let mut hasher = Sha256::new();
    hasher.update(root);
    hasher.update(length_bytes);
    let root = hasher.finalize_reset().to_vec();

    root
}

// gets the bytes representing a specific var out of the
// ssz serialized state object
pub fn get_var_bytes(
    start: usize,
    length: usize,
    serialized_state: &Vec<u8>,
    bit_flag: &bool,
) -> Vec<u8> {
    // start and stop idxs for vars in ssz serialized object
    let stop = start + length;
    let var_as_bytes = &serialized_state[start..stop];

    // if the var is justification_bits then remove the end cap before continuing
    if *bit_flag {
        return remove_cap_from_justification_bits(&var_as_bytes.to_vec());
    } else {
        //check lengths are consistent
        assert!(stop - start == length);
        assert!(
            stop <= serialized_state.len(),
            "stop {:?} exceeds end of ssz obj",
            stop
        );
        assert_eq!(length, stop - start);
        assert_eq!(length, var_as_bytes.len());

        return var_as_bytes.to_vec();
    }
}

pub fn pack(var_as_bytes: Vec<u8>) -> Vec<u8> {
    if var_as_bytes.len() == 32 {
        assert_eq!(var_as_bytes.len(), 32 as usize);
        let padded_var: Vec<u8> = var_as_bytes.to_vec();
        return padded_var;
    } else if var_as_bytes.len() < 32 {
        assert!(var_as_bytes.len() < 32);
        let padded_var: Vec<u8> = pad_to_32(&var_as_bytes);
        assert_eq!(padded_var.len(), 32);
        return padded_var;
    } else {
        if var_as_bytes.len() % 32 == 0 {
            // if length > 32 and is multiple of 32
            let n_chunks: usize = var_as_bytes.len() / 32;

            if n_chunks.is_power_of_two() {
                let padded_var: Vec<u8> = var_as_bytes.to_vec();
                assert!(padded_var.len().is_power_of_two());
                assert!(padded_var.len() % 32 == 0);

                return padded_var;
            } else {
                // if length > 32 and multiple of 32
                // but N chunks not a power of 2

                let padded_var: Vec<u8> = pad_chunks_to_power2(&var_as_bytes);
                assert!(padded_var.len().is_power_of_two());
                assert!(padded_var.len() % 32 == 0);
                return padded_var;
            }
        } else {
            //length > 32 but not a multiple of 32
            let intermediate_var = pad_to_multiple_of_32(&var_as_bytes);

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

// accessory funcs for pack()
pub fn pad_to_32(var: &[u8]) -> Vec<u8> {
    // takes ssz bytes and pads with zeros to 32 byte length

    let mut padded_var: Vec<u8> = vec![];
    let n_pad = 32 - var.len();
    let pad = vec![0u8; n_pad];

    for i in var.iter() {
        padded_var.push(*i);
    }

    padded_var.extend_from_slice(&pad);

    assert_eq!(padded_var.len(), 32);

    return padded_var;
}

pub fn pad_to_multiple_of_32(var: &[u8]) -> Vec<u8> {
    // for vars with >1 chunk, pads with zeros to next multiple of 32 bytes

    let mut padded_var: Vec<u8> = vec![];
    let pad = vec![0u8; 1];
    let mut length_mut = var.len();

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

pub fn remove_cap_from_justification_bits(justification_bits: &Vec<u8>) -> Vec<u8> {
    let mut bits: BitVec = BitVec::from_bytes(&justification_bits);
    println!("\njustification bits with length-cap\n{:?}\n", bits);
    // iterate backwards and remove 1st "true"
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

