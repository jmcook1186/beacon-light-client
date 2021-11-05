use std::format;
use crate::http_requests;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee};


// pub fn get_sync_committee_ids(api_key: &str, node_id: &str, state_id: &str)->Vec<u8>{

//     // get list of validators included in sync commitee 
//     let endpoint = format!("v1/beacon/states/{}/sync_committees",state_id);
//     let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();
    
//     // grab the validator ids only and parse them as u8s to vector validators_vec
//     let validators = result["data"]["validators"].to_string();
//     let _trimmed = &validators[1..validators.len() - 1].replace("\"", "");
//     let validator_ids: Vec<u8> = _trimmed.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
//     assert_eq!(validator_ids.len(), 512);

//     return validator_ids
// }


// pub fn get_sync_committee_pubkeys(api_key: &str, node_id: &str, state_id: &str, validator_ids: Vec<u8>)->Vec<Vec<u8>>{

//     // grab the validator info from the /validators endpoint - includes validator's pubkeys
//     let endpoint = format!("v1/beacon/states/{}/validators",state_id);
//     let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

//     // for the validators included in the sync committee, get their pubkeys and parse as u8
//     let mut pubkeys_str = Vec::new();
    
    
//     for i in validator_ids{
//         // if validator is in sync committee AND status is active
//         // first traverse the json to find the right data, 
//         // then remove quotation marks
//         if result["data"][i as usize]["status"].to_string().contains("active"){
//         pubkeys_str.push(result["data"][i as usize]["validator"]["pubkey"].to_string().replace("\"", ""));
//         }

//     }

//     // now recast as u8 bytes and push to pubkeys vec
//     let mut pubkeys = Vec::new();
//     for i in pubkeys_str{
//         pubkeys.push(i.into_bytes());
//     }

//     return pubkeys

// }





pub fn get_block_header(api_key: &str, node_id: &str, state_id: &str)->(BeaconBlockHeader){

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