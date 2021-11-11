// STRUCTS

use serde::{Deserialize, Serialize};

pub struct SyncCommittee {
    pub pubkeys: Vec<Vec<u8>>,
    pub aggregate_pubkey: Vec<u8>,
  }
  
pub struct SyncAggregate{
    pub sync_committee_bits: Vec<u8>,
    pub sync_committee_signature: String,
}

#[derive(Clone)]
pub struct BeaconBlockHeader{
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: Vec<u8>,
    pub state_root: Vec<u8>,
    pub body_root: Vec<u8>,
}
  
  
pub struct LightClientSnapshot{
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee
}


pub struct LightClientStore{

    pub snapshot: LightClientSnapshot,
    pub valid_updates: Vec<LightClientUpdate>,

}
impl LightClientStore{
    pub fn add_update(&mut self, update: LightClientUpdate) ->bool{
        self.valid_updates.push(update);
        true
    }
}
impl LightClientStore{
    pub fn refresh_snapshot(&mut self, snapshot: LightClientSnapshot) ->bool{
        self.snapshot = snapshot;
        true
    }
}


pub struct LightClientUpdate{
    pub header: BeaconBlockHeader,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<u8>,
    pub finality_header: BeaconBlockHeader,
    pub finality_branch: Vec<u8>,
    pub sync_committee_bits: Vec<u8>,
    pub sync_committee_signature: String,
    pub fork_version: String,
}

pub struct Checkpoint{
    pub epoch: u64,
    pub root: Vec<u8>,
}

pub struct Fork{
    pub current_version: Vec<u8>,
    pub epoch: u64,
    pub previous_version: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Eth1Data{
    pub deposit_root: Vec<u8>,
    pub deposit_count: u64,
    pub block_hash: Vec<u8>
}

pub struct BeaconState{

    pub genesis_time: u64,
    pub genesis_validators_root: Vec<u8>,
    pub slot: u64,
    pub fork: Fork,
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vec<u8>,
    pub state_roots: Vec<u8>,
    pub historical_roots: Vec<u8>,
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: Vec<Eth1Data>,
    pub eth1_deposit_index: u64,
    pub validators: Vec<Validator>,
    pub balances: Vec<u64>,
    pub randao_mixes: Vec<Vec<u8>>,
    pub slashings: Vec<Vec<u8>>,
    pub previous_epoch_participation: Vec<Vec<u8>>,
    pub current_epoch_participation: Vec<Vec<u8>>,
    pub justification_bits: Vec<u64>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
    pub inactivity_scores: Vec<u64>,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,
}


pub struct Validator{
    pub pubkey: Vec<u8>,
    pub withdrawal_credentials: Vec<u8>,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Vec<u8>,
    pub activation_epoch: u64,
    pub exit_epoch: u64,
    pub withdrawable_epoch: u64,
}