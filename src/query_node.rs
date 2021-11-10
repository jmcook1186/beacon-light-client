use std::format;
use crate::http_requests;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, LightClientUpdate};


pub fn get_sync_committees(state: &serde_json::Value)->(SyncCommittee, SyncCommittee){

    let current_sync_committee_pubkeys = state["data"]["current_sync_committee"]["pubkeys"].to_string();
    let current_aggregate_pubkey = state["data"]["current_sync_committee"]["aggregate_pubkey"].to_string();
    let next_sync_committee_pubkeys = state["data"]["next_sync_committee"]["pubkeys"].to_string();
    let next_aggregate_pubkey = state["data"]["aggregate_pubkey"].to_string();

    let current_sync_committee = SyncCommittee{pubkeys: current_sync_committee_pubkeys, aggregate_pubkey: current_aggregate_pubkey};
    let next_sync_committee = SyncCommittee{pubkeys: next_sync_committee_pubkeys, aggregate_pubkey: next_aggregate_pubkey};

    return (current_sync_committee, next_sync_committee)
}


pub fn get_full_state_object(api_key: &str, node_id: &str, state_id: &str)->serde_json::Value{
    
    let endpoint = format!("v2/debug/beacon/states/{}",state_id);
    let state: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

    for (key, value) in state["data"].as_object().unwrap(){
      println!("{}", key);
    }

    return state
}

pub fn get_block_body(api_key: &str, node_id: &str, state_id: &str)->serde_json::Value{
    
    let endpoint = format!("v2/beacon/blocks/{}",state_id);
    let blockbody: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();
    
    return blockbody
}

// #[tokio::main]
// pub async fn get_state_as_ssz_bytes(api_key: &str, node_id: &str, state_id: &str)->Vec<u8>{


//     let endpoint = format!("lighthouse/beacon/states/{}/ssz",state_id);

//     let prefix: String = format!("http://localhost:{}/eth/",node_id);
//     let url: String = prefix+&endpoint;
//     let client = reqwest::Client::new();
//     let _headers: HeaderMap = get_request_auth_header(api_key).unwrap();
  
//     let response = 
//       client.get(&url).headers(_headers).send().await;
      
//     let out = response.map(|bytes| BeaconState::from_ssz_bytes(&bytes, spec).map_err(Error::InvalidSsz))
//       .transpose();

//     return out
// }

// pub fn get_block_header(api_key: &str, node_id: &str, state_id: &str)->BeaconBlockHeader{

//     let endpoint = format!("v1/beacon/headers/{}",state_id);
//     let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

//     let _slot = result["data"]["header"]["message"]["slot"].to_string();
//     let _trimmed = &_slot.replace("\"", "");
//     let slot = _trimmed.parse::<u32>().unwrap();

//     let _proposer_index = result["data"]["header"]["message"]["proposer_index"].to_string();
//     let _trimmed = &_proposer_index.replace("\"", "");
//     let proposer_index = _trimmed.parse::<u32>().unwrap();

//     let parent_root = result["data"]["header"]["message"]["parent_root"].to_string();
//     let body_root = result["data"]["header"]["message"]["body_root"].to_string();
//     let state_root =result["data"]["header"]["message"]["state_root"].to_string();

//     let beacon_block_header = BeaconBlockHeader{slot: slot, proposer_index: proposer_index,
//         parent_root: parent_root, state_root: state_root, body_root: body_root};

//     return beacon_block_header
// }


