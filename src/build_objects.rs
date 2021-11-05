use std::format;
use crate::http_requests;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee};


pub fn get_snapshot(block_header: BeaconBlockHeader, 
    current_sync_committee: SyncCommittee, 
    next_sync_committee: SyncCommittee)-> LightClientSnapshot{

    let snapshot = LightClientSnapshot{
        header: block_header,
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,
    };


    return snapshot
}
