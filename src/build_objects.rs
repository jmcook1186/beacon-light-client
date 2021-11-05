use std::format;
use crate::http_requests;
use crate::query_node;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee};


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



pub fn sync_finalized(api_key:&str, node_id: &str)->LightClientSnapshot{

    let state_id = "finalized".to_string();
    // get sync committee and snapshot objects

    let block_header = query_node::get_block_header(&api_key, &node_id, &state_id);
    let (current_sync_committee, next_sync_committee) = 
        query_node::get_sync_committees(&api_key, &node_id, &state_id);
    let snapshot = make_snapshot(block_header, current_sync_committee, next_sync_committee);
        
    return snapshot


}


// struct LightClientUpdate{
//     // Update beacon block header
//     header: BeaconBlockHeader,
//     // Next sync committee corresponding to the header
//     next_sync_committee: String, //SyncCommittee,
// //     next_sync_committee_branch: vec, //Vector[Bytes32, floorlog2(NEXT_SYNC_COMMITTEE_INDEX)],
// //     // Finality proof for the update header
// //     finality_header: String, //BeaconBlockHeader,
// //     finality_branch: vec, //Vector[Bytes32, floorlog2(FINALIZED_ROOT_INDEX)],
// //     // Sync committee aggregate signature
// //     sync_committee_bits: vec, //Bitvector[SYNC_COMMITTEE_SIZE],
// //     sync_committee_signature: String, //BLSSignature,
// //     // Fork version for the aggregate signature
// //     fork_version: String//Version
// }
