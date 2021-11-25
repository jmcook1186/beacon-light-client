use eth2::types::*;
use std::sync::Arc;
use ssz_types::{typenum::Unsigned, typenum::U512, length::Fixed, BitVector, FixedVector, Bitfield};
use ethereum_types::H256;



pub struct LightClientSnapshot{
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>
}


pub struct LightClientUpdate{

    pub header: BeaconBlockHeader,  // comes from header struct
    
    // Next sync committee corresponding to the header
    pub next_sync_committee: Arc<SyncCommittee<MainnetEthSpec>>,  //full syncCommittee struct
    
    // vector of bytes32 with length equal to floorlog2(generalizedindex)
    pub next_sync_committee_branch: Vec<H256>, 
    
    // # Finality proof for the update header
    pub finality_header: BlockHeaderData,  // comes from header struct
    
    // vector of bytes32 with length equal to floorlog2(generalizedindex)
    pub finality_branch: Vec<H256>,    
    
    // Sync committee aggregate signature
    pub sync_committee_bits: Bitfield<Fixed<U512>>,

    // Fork version for the aggregate signature
    pub fork_version: [u8; 4],
}
