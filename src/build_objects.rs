use std::format;
use crate::http_requests;
use crate::query_node;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, LightClientStore, LightClientUpdate};


pub fn make_snapshot(block_header: BeaconBlockHeader, 
    current_sync_committee: SyncCommittee, 
    next_sync_committee: SyncCommittee)-> LightClientSnapshot{

    let snapshot = LightClientSnapshot{
        header: block_header,
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,
    };


    return snapshot
}



pub fn initial_snapshot(api_key:&str, node_id: &str)->LightClientSnapshot{

    let state_id = "finalized".to_string();
    // get sync committee and snapshot objects

    let block_header = query_node::get_block_header(&api_key, &node_id, &state_id);
    let (current_sync_committee, next_sync_committee) = 
        query_node::get_sync_committees(&api_key, &node_id, &state_id);
    let snapshot = make_snapshot(block_header, current_sync_committee, next_sync_committee);
        
    return snapshot


}

pub fn next_snapshot(current_store:LightClientStore, api_key:&str, node_id: &str)->LightClientSnapshot{
    

    let state_id = "head".to_string();
    // get sync committee and snapshot objects

    let block_header = query_node::get_block_header(&api_key, &node_id, &state_id);
    let (current_sync_committee, next_sync_committee) = 
        query_node::get_sync_committees(&api_key, &node_id, &state_id);
    let snapshot = make_snapshot(block_header, current_sync_committee, next_sync_committee);
        
    return snapshot
}




pub fn initialize_store(snapshot: LightClientSnapshot)->LightClientStore{


    
    let store = LightClientStore{
        snapshot: snapshot,
        valid_updates: None,
    };


    return store
}




