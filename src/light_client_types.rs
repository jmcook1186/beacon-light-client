use eth2::types::*;
use std::sync::Arc;

pub struct LightClientSnapshot{
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>
}


// pub struct LightClientUpdate{
    
//     header: BeaconBlockHeader  // comes from header struct
//     // Next sync committee corresponding to the header
//     next_sync_committee: SyncCommittee  //full syncCommittee struct
//     next_sync_committee_branch: Vector[Bytes32, floorlog2(NEXT_SYNC_COMMITTEE_INDEX)] // vector of bytes32 with length equal to floorlog2(generalizedindex)
//     // # Finality proof for the update header
//     finality_header: BeaconBlockHeader  // comes from header struct
//     finality_branch: Vector[Bytes32, floorlog2(FINALIZED_ROOT_INDEX)]    // vector of bytes32 with length equal to floorlog2(generalizedindex)
//     // Sync committee aggregate signature
//     sync_committee_bits: Bitvector[SYNC_COMMITTEE_SIZE]   // comes from syncAggregate struct
//     sync_committee_signature: BLSSignature  // comes from syncAggregate struct
//     // Fork version for the aggregate signature
//     fork_version: Version
// }
