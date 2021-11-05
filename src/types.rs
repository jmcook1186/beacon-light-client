// STRUCTS

pub struct SyncCommittee {
    pub pubkeys: String,
    pub aggregate_pubkey: String,
  }
  
pub struct BeaconBlockHeader{
    pub slot: u32,
    pub proposer_index: u32,
    pub parent_root: String,
    pub state_root: String,
    pub body_root: String,
}
  
  
pub struct LightClientSnapshot{
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee
}


  // struct LightClientStore{

//     snapshot: LightClientSnapshot,
//     valid_updates: Option <LightClientUpdate>

// }


// struct LightClientUpdate{
//     // Update beacon block header
//     header: String, //BeaconBlockHeader,
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
