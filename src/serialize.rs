use eth2::types::*;
extern crate hex;
use crate::constants::*;
use bit_vec::BitVec;
use ssz::Encode;
use std::collections::HashMap;

pub fn serialize_beacon_state(
    state: &BeaconState<MainnetEthSpec>,
) -> (Vec<u8>, HashMap<&str, usize>, HashMap<&str, usize>) {
    // func takes state object as received from api endpoint and serializes it
    // according to the ssz specs

    let mut fixed_parts: Vec<u8> = vec![];
    let mut variable_parts: Vec<u8> = vec![];
    let mut variable_lengths = HashMap::new();
    let mut sizes = HashMap::new();
    let mut offsets = HashMap::new();

    println!("*** SSZ SERIALIZING STATE OBJECT ***");

    // make hashmap of var lengths to pass to merklize

    for i in state.genesis_time().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "genesis_time",
        state.genesis_time().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.genesis_validators_root().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "genesis_validators_root",
        state
            .genesis_validators_root()
            .as_ssz_bytes()
            .ssz_bytes_len(),
    );

    for i in state.slot().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert("slot", state.slot().as_ssz_bytes().ssz_bytes_len());

    // fork is a container, so serialize all fields
    let mut fork: Vec<u8> = vec![];
    fork.append(&mut state.fork().previous_version.as_ssz_bytes());
    fork.append(&mut state.fork().current_version.as_ssz_bytes());
    fork.append(&mut state.fork().epoch.as_ssz_bytes());

    for i in fork.iter() {
        fixed_parts.push(*i);
    }

    // we will still need the length of the fields for deserializing
    sizes.insert("fork", fork.len());
    sizes.insert(
        "fork_previous_version",
        state.fork().previous_version.as_ssz_bytes().len(),
    );
    sizes.insert(
        "fork_current_version",
        state.fork().current_version.as_ssz_bytes().len(),
    );
    sizes.insert("fork_epoch", state.fork().epoch.as_ssz_bytes().len());

    // latest_block_header is also a container
    let mut latest_block_header: Vec<u8> = vec![];
    latest_block_header.append(&mut state.latest_block_header().slot.as_ssz_bytes());
    latest_block_header.append(&mut state.latest_block_header().proposer_index.as_ssz_bytes());
    latest_block_header.append(&mut state.latest_block_header().parent_root.as_ssz_bytes());
    latest_block_header.append(&mut state.latest_block_header().state_root.as_ssz_bytes());
    latest_block_header.append(&mut state.latest_block_header().body_root.as_ssz_bytes());

    for i in latest_block_header.iter() {
        fixed_parts.push(*i);
    }

    // need sizes of each field for deserialization
    sizes.insert("latest_block_header", latest_block_header.len());
    sizes.insert(
        "header_proposer_index",
        state
            .latest_block_header()
            .proposer_index
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "header_slot",
        state.latest_block_header().slot.as_ssz_bytes().len(),
    );
    sizes.insert(
        "header_parent_root",
        state.latest_block_header().parent_root.as_ssz_bytes().len(),
    );
    sizes.insert(
        "header_state_root",
        state.latest_block_header().state_root.as_ssz_bytes().len(),
    );
    sizes.insert(
        "header_body_root",
        state.latest_block_header().body_root.as_ssz_bytes().len(),
    );

    for i in state.block_roots().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "block_roots",
        state.block_roots().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.state_roots().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "state_roots",
        state.state_roots().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.historical_roots().as_ssz_bytes().iter() {
        variable_parts.push(*i);
    }
    sizes.insert(
        "historical_roots",
        state.historical_roots().as_ssz_bytes().ssz_bytes_len(),
    );
    variable_lengths.insert("historical_roots", sizes["historical_roots"]);
    let offset_bytes: [u8; 8] = variable_parts.len().to_le_bytes();
    for i in offset_bytes[0..4].to_vec() {
        fixed_parts.push(i);
    }

    // eth1_data is also a container
    let mut eth1_data: Vec<u8> = vec![];
    eth1_data.append(&mut state.eth1_data().deposit_root.as_ssz_bytes());
    eth1_data.append(&mut state.eth1_data().deposit_count.as_ssz_bytes());
    eth1_data.append(&mut state.eth1_data().block_hash.as_ssz_bytes());

    for i in eth1_data.iter() {
        fixed_parts.push(*i);
    }

    // need sizes of each field for deserialization
    sizes.insert("eth1_data", eth1_data.len());
    sizes.insert(
        "eth1_deposit_count",
        state.eth1_data().deposit_count.as_ssz_bytes().len(),
    );
    sizes.insert(
        "eth1_deposit_root",
        state.eth1_data().deposit_root.as_ssz_bytes().len(),
    );
    sizes.insert(
        "eth1_block_hash",
        state.eth1_data().block_hash.as_ssz_bytes().len(),
    );

    for i in state.eth1_data_votes().as_ssz_bytes().iter() {
        variable_parts.push(*i);
    }
    sizes.insert(
        "eth1_data_votes",
        state.eth1_data_votes().as_ssz_bytes().ssz_bytes_len(),
    );

    variable_lengths.insert("eth1_data_votes", sizes["eth1_data_votes"]);
    let offset_bytes: [u8; 8] = variable_parts.len().to_le_bytes();
    for i in offset_bytes[0..4].to_vec() {
        fixed_parts.push(i);
    }

    for i in state.eth1_deposit_index().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "eth1_deposit_index",
        state.eth1_deposit_index().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.eth1_deposit_index().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "eth1_deposit_index",
        state.eth1_deposit_index().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.validators().as_ssz_bytes().iter() {
        variable_parts.push(*i);
    }
    sizes.insert(
        "validators",
        state.validators().as_ssz_bytes().ssz_bytes_len(),
    );
    variable_lengths.insert("validators", sizes["validators"]);

    for i in state.balances().as_ssz_bytes().iter() {
        variable_parts.push(*i);
    }
    sizes.insert("balances", state.balances().as_ssz_bytes().ssz_bytes_len());
    variable_lengths.insert("balances", sizes["balances"]);

    for i in state.randao_mixes().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "randao_mixes",
        state.randao_mixes().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state.slashings().as_ssz_bytes().iter() {
        fixed_parts.push(*i);
    }
    sizes.insert(
        "slashings",
        state.slashings().as_ssz_bytes().ssz_bytes_len(),
    );

    for i in state
        .previous_epoch_participation()
        .unwrap()
        .as_ssz_bytes()
        .iter()
    {
        variable_parts.push(*i);
    }
    sizes.insert(
        "previous_epoch_participation",
        state
            .previous_epoch_participation()
            .unwrap()
            .as_ssz_bytes()
            .ssz_bytes_len(),
    );
    variable_lengths.insert(
        "previous_epoch_participation",
        sizes["previous_epoch_participation"],
    );
    let offset_bytes: [u8; 8] = variable_parts.len().to_le_bytes();
    for i in offset_bytes[0..4].to_vec() {
        fixed_parts.push(i);
    }

    for i in state
        .current_epoch_participation()
        .unwrap()
        .as_ssz_bytes()
        .iter()
    {
        variable_parts.push(*i);
    }
    sizes.insert(
        "current_epoch_participation",
        state
            .previous_epoch_participation()
            .unwrap()
            .as_ssz_bytes()
            .ssz_bytes_len(),
    );
    variable_lengths.insert(
        "current_epoch_participation",
        sizes["current_epoch_participation"],
    );
    let offset_bytes: [u8; 8] = variable_parts.len().to_le_bytes();
    for i in offset_bytes[0..4].to_vec() {
        fixed_parts.push(i);
    }

    // append a length cap to justification bits before serializing
    let justification_bits: Vec<u8> =
        length_cap_to_bitvector(&state.justification_bits().as_ssz_bytes());
    for i in justification_bits.iter() {
        fixed_parts.push(*i);
    }
    sizes.insert("justification_bits", justification_bits.len());

    // previous_justified_checkpoint is also a container
    let mut previous_justified_checkpoint: Vec<u8> = vec![];
    previous_justified_checkpoint.append(
        &mut state
            .previous_justified_checkpoint()
            .epoch
            .as_u64()
            .as_ssz_bytes(),
    );

    previous_justified_checkpoint
        .append(&mut state.previous_justified_checkpoint().root.as_ssz_bytes());

    for i in previous_justified_checkpoint.iter() {
        fixed_parts.push(*i);
    }

    // and we need the sizes of the fields for deserialization
    sizes.insert(
        "previous_justified_checkpoint",
        previous_justified_checkpoint.len(),
    );
    sizes.insert(
        "previous_checkpoint_epoch",
        state
            .previous_justified_checkpoint()
            .epoch
            .as_u64()
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "previous_checkpoint_root",
        state
            .previous_justified_checkpoint()
            .root
            .as_ssz_bytes()
            .len(),
    );

    // current_justified_checkpoint is also a container
    let mut current_justified_checkpoint: Vec<u8> = vec![];
    current_justified_checkpoint.append(
        &mut state
            .current_justified_checkpoint()
            .epoch
            .as_u64()
            .as_ssz_bytes(),
    );

    current_justified_checkpoint
        .append(&mut state.current_justified_checkpoint().root.as_ssz_bytes());
    for i in current_justified_checkpoint.iter() {
        fixed_parts.push(*i);
    }

    // and we need the sizes of the fields for deserialization
    sizes.insert(
        "current_justified_checkpoint",
        current_justified_checkpoint.len(),
    );

    sizes.insert(
        "current_checkpoint_epoch",
        state
            .current_justified_checkpoint()
            .epoch
            .as_u64()
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "current_checkpoint_root",
        state
            .current_justified_checkpoint()
            .root
            .as_ssz_bytes()
            .len(),
    );

    // finalized checkpoint is a container
    let mut finalized_checkpoint: Vec<u8> = vec![];
    finalized_checkpoint.append(&mut state.finalized_checkpoint().epoch.as_ssz_bytes());
    finalized_checkpoint.append(&mut state.finalized_checkpoint().root.as_ssz_bytes());

    for i in finalized_checkpoint.iter() {
        fixed_parts.push(*i);
    }
    // and we need the sizes of the fields for deserialization
    sizes.insert("finalized_checkpoint", finalized_checkpoint.len());
    sizes.insert(
        "finalized_checkpoint_epoch",
        state
            .finalized_checkpoint()
            .epoch
            .as_u64()
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "finalized_checkpoint_root",
        state.finalized_checkpoint().root.as_ssz_bytes().len(),
    );

    for i in state.inactivity_scores().unwrap().as_ssz_bytes().iter() {
        variable_parts.push(*i);
    }
    sizes.insert(
        "inactivity_scores",
        state
            .inactivity_scores()
            .unwrap()
            .as_ssz_bytes()
            .ssz_bytes_len(),
    );
    variable_lengths.insert("inactivity_scores", sizes["inactivity_scores"]);
    let offset_bytes: [u8; 8] = variable_parts.len().to_le_bytes();
    for i in offset_bytes[0..4].to_vec() {
        fixed_parts.push(i);
    }

    // current_sync_committee is a container
    let mut current_sync_committee: Vec<u8> = vec![];
    current_sync_committee.append(
        &mut state
            .current_sync_committee()
            .unwrap()
            .pubkeys
            .as_ssz_bytes(),
    );
    current_sync_committee.append(
        &mut state
            .current_sync_committee()
            .unwrap()
            .aggregate_pubkey
            .as_ssz_bytes(),
    );

    for i in current_sync_committee.iter() {
        fixed_parts.push(*i);
    }
    // and we need the sizes for dseserialization
    sizes.insert("current_sync_committee", current_sync_committee.len());
    sizes.insert(
        "current_sync_committee_pubkeys",
        state
            .current_sync_committee()
            .unwrap()
            .pubkeys
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "current_sync_committee_agg_pubkey",
        state
            .current_sync_committee()
            .unwrap()
            .aggregate_pubkey
            .as_ssz_bytes()
            .len(),
    );

    // next_sync_comittee is a container
    let mut next_sync_committee: Vec<u8> = vec![];
    next_sync_committee.append(&mut state.next_sync_committee().unwrap().pubkeys.as_ssz_bytes());
    next_sync_committee.append(
        &mut state
            .next_sync_committee()
            .unwrap()
            .aggregate_pubkey
            .as_ssz_bytes(),
    );

    for i in next_sync_committee.iter() {
        fixed_parts.push(*i);
    }
    // and we need the sizes for deserialization
    sizes.insert("next_sync_committee", next_sync_committee.len());
    sizes.insert(
        "next_sync_committee_pubkeys",
        state
            .next_sync_committee()
            .unwrap()
            .pubkeys
            .as_ssz_bytes()
            .len(),
    );
    sizes.insert(
        "next_sync_committee_agg_pubkey",
        state
            .next_sync_committee()
            .unwrap()
            .aggregate_pubkey
            .as_ssz_bytes()
            .len(),
    );

    // insert total size into size hashmap
    // also assert that the total serialized size equals the last offset + last var size
    assert!(
        fixed_parts.len() + variable_parts.len()
            < 2usize.pow((BYTES_PER_LENGTH_OFFSET * BITS_PER_BYTE) as u32)
    );
    sizes.insert("fixed_parts", fixed_parts.len());
    sizes.insert("variable_parts", variable_parts.len());
    sizes.insert("total_length", fixed_parts.len() + variable_parts.len());

    // calculate offsets and add to hashmap
    // offset is previous offset plus previous var length
    offsets.insert("historical_roots", sizes["fixed_parts"]);
    offsets.insert(
        "eth1_data_votes",
        offsets["historical_roots"] + sizes["historical_roots"],
    );
    offsets.insert(
        "validators",
        offsets["eth1_data_votes"] + sizes["eth1_data_votes"],
    );
    offsets.insert("balances", offsets["validators"] + sizes["validators"]);
    offsets.insert(
        "previous_epoch_participation",
        offsets["balances"] + sizes["balances"],
    );
    offsets.insert(
        "current_epoch_participation",
        offsets["previous_epoch_participation"] + sizes["previous_epoch_participation"],
    );
    offsets.insert(
        "inactivity_scores",
        offsets["current_epoch_participation"] + sizes["current_epoch_participation"],
    );

    // BUILD SERIALIZED STATE OBJECT
    // define serialized state object as empty vec
    // append fixed and variable parts to it
    // offsets are already interleaved in fixed_parts
    let mut serialized_state: Vec<u8> = vec![];
    serialized_state.append(&mut fixed_parts);
    serialized_state.append(&mut variable_parts);

    assert_eq!(
        serialized_state.len(),
        sizes["total_length"],
        "NEW TEST 1 FAILS: SERIALIZE LINE 522"
    );
    assert_eq!(
        serialized_state.len(),
        state.as_ssz_bytes().len(),
        "NEW TEST 2 FAILS: SERIALIZE LINE 523"
    );

    // println!("\nSIZE (IN BYTES) OF EACH VAR:\n");
    // for (key, value) in sizes.iter() {
    //     println!("{:?}: {:?}", key, value);
    // }
    // println!("\nVARIABLE-LENGTH VAR OFFSETS:\n");
    // for (key, value) in offsets.iter() {
    //     println!("{:?}: {:?}", key, value);
    // }

    
    return (serialized_state, sizes, offsets);
}

pub fn length_cap_to_bitvector(var: &Vec<u8>) -> Vec<u8> {
    // JUSTIFICATION BITS
    // BITVECTOR REQUIRES AN ADDITIONAL 1 APPENDED TO THE END AS LENGTH CAP
    let mut bits = BitVec::from_bytes(var);
    assert!(bits.len() % 4 == 0);

    // go backwards through justification bits
    // until we find a "true". Then flip
    // the next bit to true. [00010] -> [00011]
    // if there are no true's make 0 index true.
    for i in (0..bits.len()).rev() {
        if bits[i] == true {
            let idx = bits.len() - i;

            if i < bits.len() {
                bits.set(idx + 1, true);
            } else {
                bits.push(true);
            }
            // break so we only ever
            // add one length cap
            break;
        } else if i == 0 {
            // if we reach the 0 index
            bits.set(0, true);
        }
    }

    // convert to bytes
    let bytes: Vec<u8> = bits.to_bytes();

    // justification bit length should be 4 bytes
    // zero vector is illegal (should never occur here bc of length cap)
    assert!(bytes.iter().sum::<u8>() > 0);

    return bytes;
}
