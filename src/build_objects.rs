use std::format;
use crate::http_requests;
use crate::query_node;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, 
    LightClientStore, LightClientUpdate, BeaconState};


pub fn make_snapshot(state: &BeaconState)-> LightClientSnapshot{

    let (current_sync_committee, next_sync_committee) = query_node::get_sync_committees(&state);

    let snapshot = LightClientSnapshot{
        header: state.latest_block_header.to_owned(),
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,
    };


    return snapshot
}


// pub fn initialize_store(snapshot: LightClientSnapshot)->LightClientStore{
    
//     // initialize with empty update vec
//     let empty_updates: Vec<LightClientUpdate> = vec![];

//     let store = LightClientStore{
//         snapshot: snapshot,
//         valid_updates: empty_updates,
//     };


//     return store
// }


// pub fn update_store(mut store: LightClientStore, snapshot: LightClientSnapshot, update: LightClientUpdate)->LightClientStore{

//     // call class method of LightClientStore to add update to vec and refresh snasphot
//     store.add_update(update);
//     store.refresh_snapshot(snapshot);
    
//     return store
// }


// pub fn get_update(state: &serde_json::Value, current_snapshot: &LightClientSnapshot, beacon_block_body: &serde_json::Value )->LightClientUpdate{


//     let new_header = state.latest_block_header;

//     let current_header = &current_snapshot.header;

//     // new sync committees from state object
//     let (current_sync_committee, next_sync_committee) = query_node::get_sync_committees(&state);

//     // new snapshot from new header and new sync comms
//     let snapshot = LightClientSnapshot{
//         header: new_header,
//         current_sync_committee: current_sync_committee,
//         next_sync_committee: next_sync_committee,
//     };

//     // get sync_aggregate from beacon block body
//     // parse to vector of u8s
//     let _sync_committee_bits = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_bits"].to_string();
//     let _trimmed = &_sync_committee_bits.replace("\"", "");
//     let sync_committee_bits: Vec<u8> = _trimmed.as_bytes().to_vec();


//     // THIS IS THE ROOT, BUT WE NEED THE MERKLE BRANCH CONNECTING IT TO BEACON STATE
//     let _finalized_branch = state["data"]["finalized_checkpoint"]["root"].to_string();
//     let _trimmed = &_finalized_branch.replace("\"", "");
//     let finalized_branch: Vec<u8> = _trimmed.as_bytes().to_vec();



//     // get sync committee signature 
//     let sync_committee_signature = beacon_block_body["data"]["message"]["body"]["sync_aggregate"]["sync_committee_signature"].to_string();

//     // other update vars from state obj
//     let branch = vec![0,1,2,3,4,5]; //PLACEHOLDER
//     let finality_header = current_header;
//     let finality_branch = finalized_branch;//PLACEHOLDER
//     let sync_committee_bits = sync_committee_bits;
//     let fork = state["data"]["fork"].to_string();
//     let sync_pubkeys = &snapshot.next_sync_committee.pubkeys.to_string();

//     // build update obj
//     let update =  LightClientUpdate{
//         header: snapshot.header,
//         next_sync_committee: snapshot.next_sync_committee,
//         next_sync_committee_branch: branch,
//         finality_header: finality_header,
//         finality_branch: finality_branch,
//         sync_committee_bits: sync_committee_bits,
//         sync_committee_signature: sync_committee_signature,
//         fork_version: fork,
//     };

//     return update

// }


