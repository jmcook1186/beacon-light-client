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


pub struct LightClientStore{

    pub snapshot: LightClientSnapshot,
    pub valid_updates: Option <LightClientUpdate>,

}



pub struct LightClientUpdate{
    pub header: BeaconBlockHeader,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<i32>,
    pub finality_header: BeaconBlockHeader,
    pub finality_branch: Vec<i32>,
    pub sync_committee_bits: Vec<i32>,
    pub sync_committee_signature: String,
    pub fork_version: String,
}