

// pub fn calculate_leaves(
//     serialized_state: &Vec<u8>,
//     sizes: &HashMap<&str, usize>,
//     offsets: &HashMap<&str, usize>,
// ) -> Vec<String> {
//     // sha256 hashes vecs of bytes from serialized object
//     // mixes in length data as per spec

//     let mut leaves = vec![];
//     let mut start_idx: usize = 0;

//     keys: Vec<&str> = vec!["genesis_time", "genesis_validators_root", "slot",
//     "fork_prev_ver", "fork_curr_ver", "fork_epoch", "header_slot",
//     "header_proposer_index", "header_parent_root", "header_state_root",
//     "header_body_root", "block_roots", "state_roots", "historical_roots",
//     "eth1_data_dep_root", "eth1_data_deposit_count", "eth1_data_block_hash",
//     "eth1_data_votes", "eth1_deposit_index", "validators", "balances",
//     "randao_mixes", "slashings", "previous_epoch_participation", 
//     "current_epoch_participation", "justification_bits", "prev_just_check_epoch",
//     "prev_just_check_root", "curr_just_check_epoch", "curr_just_check_root",
//     "finalized_check_epoch", "finalized_checkpoint_root", "inactivity_scores",
//     "curr_sync_comm_pubkeys", "curr_sync_comm_agg_pubkey", "next_sync_comm_pubkeys",
//     "next_sync_comm_agg_pubkey"
//     ]

//     for key in keys.iter(){

//         let var = pad_bytes(start_idx, sizes[key], &serialized_state)
//         if offsets.contains_key(key){
//             start_idx+=4
//         }else{
//             start_idx += sizes[key]}

//         let root = get_hash_root(&var, &sizes[key])
//         let root = mix_in_length_data(&root, &sizes[key])
//         leaves.push(root)

//         return leaves
//     }
