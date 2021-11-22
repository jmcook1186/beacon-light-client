use eth2::types::*;
use std::sync::Arc;
use ssz_types::{typenum::Unsigned, typenum::U512, length::Fixed, BitVector, FixedVector, Bitfield};



pub struct LightClientSnapshot{
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>
}


pub struct LightClientUpdate{

    pub header: BeaconBlockHeader,  // comes from header struct
    // Next sync committee corresponding to the header
    pub next_sync_committee: Arc<SyncCommittee<MainnetEthSpec>>,  //full syncCommittee struct
    //next_sync_committee_branch: Vector[Bytes32, floorlog2(NEXT_SYNC_COMMITTEE_INDEX)], // vector of bytes32 with length equal to floorlog2(generalizedindex)
    // # Finality proof for the update header
    pub finality_header: BlockHeaderData,  // comes from header struct
    //finality_branch: Vector[Bytes32, floorlog2(FINALIZED_ROOT_INDEX)],    // vector of bytes32 with length equal to floorlog2(generalizedindex)
    // Sync committee aggregate signature
    pub sync_committee_bits: Bitfield<Fixed<U512>>,

    // Fork version for the aggregate signature
    //pub fork_version: Version,
}
