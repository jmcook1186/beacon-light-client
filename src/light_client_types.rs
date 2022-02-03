use eth2::types::*;
use ssz_types::{length::Fixed, typenum::U512, Bitfield};
use std::sync::Arc;

pub struct LightClientSnapshot {
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
}

#[derive(Debug, Clone)]
pub struct LightClientUpdate {
    pub header: BeaconBlockHeader, // comes from header struct

    // Next sync committee corresponding to the header
    pub next_sync_committee: Arc<SyncCommittee<MainnetEthSpec>>, //full syncCommittee struct

    // vector of bytes32 with length equal to floorlog2(generalizedindex)
    pub next_sync_committee_branch: Vec<Vec<u8>>,

    // # Finality proof for the update header
    pub finality_header: BlockHeaderData, // comes from header struct

    // vector of bytes32 with length equal to floorlog2(generalizedindex)
    pub finality_branch: Vec<Vec<u8>>,

    // Sync committee aggregate signature
    pub sync_committee_bits: Bitfield<Fixed<U512>>,

    // Fork version for the aggregate signature
    pub fork_version: [u8; 4],
}

pub struct LightClientStore {
    pub snapshot: LightClientSnapshot,
    pub valid_updates: Vec<LightClientUpdate>,
}

impl LightClientStore {
    pub fn create(snapshot: LightClientSnapshot) -> LightClientStore {
        LightClientStore {
            snapshot: snapshot,
            valid_updates: vec![],
        }
    }
}
