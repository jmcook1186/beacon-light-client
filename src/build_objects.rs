use std::format;
use crate::http_requests;
use crate::query_node;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, LightClientStore, LightClientUpdate};


pub fn make_snapshot(state: &serde_json::Value)-> LightClientSnapshot{

    let _slot = state["data"]["latest_block_header"]["slot"].to_string();
    let _trimmed = &_slot.replace("\"", "");
    let slot = _trimmed.parse::<u32>().unwrap();
    let _proposer_index = state["data"]["latest_block_header"]["proposer_index"].to_string();
    let _trimmed = &_proposer_index.replace("\"", "");
    let proposer_index = _trimmed.parse::<u32>().unwrap();
    let parent_root = state["data"]["latest_block_header"]["parent_root"].to_string();
    let state_root = state["data"]["latest_block_header"]["state_root"].to_string();
    let body_root = state["data"]["latest_block_header"]["body_root"].to_string();

    let header = BeaconBlockHeader{
        slot: slot,
        proposer_index: proposer_index,
        parent_root: parent_root,
        state_root: state_root,
        body_root: body_root,
    };

    let (current_sync_committee, next_sync_committee) = query_node::get_sync_committees(&state);

    let snapshot = LightClientSnapshot{
        header: header,
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,
    };


    return snapshot
}


pub fn initialize_store(snapshot: LightClientSnapshot)->LightClientStore{
    // initialize with empty update vec
    let empty_updates: Vec<LightClientUpdate> = vec![];

    let store = LightClientStore{
        snapshot: snapshot,
        valid_updates: empty_updates,
    };


    return store
}


pub fn update_store(mut store: LightClientStore, snapshot: LightClientSnapshot, update: LightClientUpdate)->LightClientStore{

    // call class method of LightClientStore to add update to vec and refresh snasphot
    store.add_update(update);
    store.refresh_snapshot(snapshot);
    
    return store
}



pub fn get_update(state: &serde_json::Value, current_snapshot: &LightClientSnapshot, beacon_block_body: &serde_json::Value )->LightClientUpdate{

    // new header from state object
    let _slot = state["data"]["latest_block_header"]["slot"].to_string();
    let _trimmed = &_slot.replace("\"", "");
    let slot = _trimmed.parse::<u32>().unwrap();
    let _proposer_index = state["data"]["latest_block_header"]["proposer_index"].to_string();
    let _trimmed = &_proposer_index.replace("\"", "");
    let proposer_index = _trimmed.parse::<u32>().unwrap();
    let parent_root = state["data"]["latest_block_header"]["parent_root"].to_string();
    let state_root = state["data"]["latest_block_header"]["state_root"].to_string();
    let body_root = state["data"]["latest_block_header"]["body_root"].to_string();

    let new_header = BeaconBlockHeader{
        slot: slot,
        proposer_index: proposer_index,
        parent_root: parent_root,
        state_root: state_root,
        body_root: body_root,
    };


    //// current header from snapshot object
    let current_slot = &current_snapshot.header.slot;
    let current_proposer_index = &current_snapshot.header.proposer_index;
    let current_parent_root = &current_snapshot.header.parent_root;
    let current_state_root = &current_snapshot.header.state_root;
    let current_body_root = &current_snapshot.header.body_root;

    let current_header = BeaconBlockHeader{
        slot: current_slot.to_owned(),
        proposer_index: current_proposer_index.to_owned(),
        parent_root: current_parent_root.to_string(),
        state_root: current_state_root.to_string(),
        body_root: current_body_root.to_string(),
    };

    // new sync committees from state object
    let (current_sync_committee, next_sync_committee) = query_node::get_sync_committees(&state);

    // new snapshot from new header and new sync comms
    let snapshot = LightClientSnapshot{
        header: new_header,
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,
    };

    // get sync_aggregate from beacon block body
    // parse to vector of u8s
    let _sync_committee_bits = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_bits"].to_string();
    let _trimmed = &_sync_committee_bits.replace("\"", "");
    let sync_committee_bits: Vec<u8> = _trimmed.as_bytes().to_vec();
    println!("Vec<u8>:{:?}", sync_committee_bits);

    // get sync committee signature 
    let sync_committee_signature = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_signature"].to_string();
    println!("{}",sync_committee_signature);
    // other update vars from state obj
    let branch = vec![0,1,2,3,4,5]; //PLACEHOLDER
    let finality_header = current_header;
    let finality_branch =vec![0,1,2,3,4,5];//PLACEHOLDER
    let sync_committee_bits = sync_committee_bits;
    let fork = state["data"]["fork"].to_string();
    let sync_pubkeys = &snapshot.next_sync_committee.pubkeys.to_string();

    // build update obj
    let update =  LightClientUpdate{
        header: snapshot.header,
        next_sync_committee: snapshot.next_sync_committee,
        next_sync_committee_branch: branch,
        finality_header: finality_header,
        finality_branch: finality_branch,
        sync_committee_bits: sync_committee_bits,
        sync_committee_signature: sync_committee_signature,
        fork_version: fork,
    };

    return update

}


// class BeaconBlockBody(Container){
    
//     randao_reveal: BLSSignature
//     eth1_data: Eth1Data  # Eth1 data vote
//     graffiti: Bytes32  # Arbitrary data
//     # Operations
//     proposer_slashings: List[ProposerSlashing, MAX_PROPOSER_SLASHINGS]
//     attester_slashings: List[AttesterSlashing, MAX_ATTESTER_SLASHINGS]
//     attestations: List[Attestation, MAX_ATTESTATIONS]
//     deposits: List[Deposit, MAX_DEPOSITS]
//     voluntary_exits: List[SignedVoluntaryExit, MAX_VOLUNTARY_EXITS]
//     sync_aggregate: SyncAggregate

// }