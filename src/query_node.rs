use std::format;
use crate::http_requests;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, LightClientUpdate};



pub fn get_block_header(api_key: &str, node_id: &str, state_id: &str)->BeaconBlockHeader{

    let endpoint = format!("v1/beacon/headers/{}",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

    let _slot = result["data"]["header"]["message"]["slot"].to_string();
    let _trimmed = &_slot.replace("\"", "");
    let slot = _trimmed.parse::<u32>().unwrap();


    let _proposer_index = result["data"]["header"]["message"]["proposer_index"].to_string();
    let _trimmed = &_proposer_index.replace("\"", "");
    let proposer_index = _trimmed.parse::<u32>().unwrap();

    println!("{}", proposer_index);

    let parent_root = result["data"]["header"]["message"]["parent_root"].to_string();
    let body_root = result["data"]["header"]["message"]["body_root"].to_string();
    let state_root =result["data"]["header"]["message"]["state_root"].to_string();

    let beacon_block_header = BeaconBlockHeader{slot: slot, proposer_index: proposer_index,
        parent_root: parent_root, state_root: state_root, body_root: body_root};

    return beacon_block_header
}


pub fn get_sync_committees(api_key: &str, node_id: &str, state_id: &str)->(SyncCommittee, SyncCommittee){

    let endpoint = format!("v2/debug/beacon/states/{}",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();
    let current_sync_committee_pubkeys = result["data"]["current_sync_committee"]["pubkeys"].to_string();
    let current_aggregate_pubkey = result["data"]["current_sync_committee"]["aggregate_pubkey"].to_string();
    let next_sync_committee_pubkeys = result["data"]["next_sync_committee"]["pubkeys"].to_string();
    let next_aggregate_pubkey = result["data"]["aggregate_pubkey"].to_string();

    let current_sync_committee = SyncCommittee{pubkeys: current_sync_committee_pubkeys, aggregate_pubkey: current_aggregate_pubkey};
    let next_sync_committee = SyncCommittee{pubkeys: next_sync_committee_pubkeys, aggregate_pubkey: next_aggregate_pubkey};

    return (current_sync_committee, next_sync_committee)
}


pub fn get_update(api_key: &str, node_id: &str)->LightClientUpdate{

    let state_id = "head";
    let current_header = get_block_header(&api_key, &node_id, &"finalized".to_string());
    let next_header = get_block_header(&api_key, &node_id, &state_id);
    let (current_sync_committee, next_sync_committee) = get_sync_committees(&api_key, &node_id, &state_id);

    let endpoint = format!("v2/debug/beacon/states/{}",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

    let branch = vec![0,1,2,3,4,5];
    let finality_header = current_header;
    let finality_branch =vec![0,1,2,3,4,5];
    let sync_committee_bits = vec![0,1,2,3,4,5];
    let fork = result["data"]["fork"].to_string();
    let sync_sig = &next_sync_committee.aggregate_pubkey.to_string();
    let sync_pubkeys = &next_sync_committee.pubkeys.to_string();

    let update =  LightClientUpdate{
        header: next_header,
        next_sync_committee: next_sync_committee,
        next_sync_committee_branch: branch,
        finality_header: finality_header,
        finality_branch: finality_branch,
        sync_committee_bits: sync_committee_bits,
        sync_committee_signature: sync_sig.to_string(),
        fork_version: fork,
    };

    return update

}