use std::format;
use crate::http_requests;
use std::sync::Arc;
extern crate hex;
use eth2::types::*;
use crate::light_client_types::LightClientSnapshot;


pub fn get_state(api_key: &str, state_id: &str, endpoint_prefix: &str)->BeaconState<MainnetEthSpec>{

    let state_suffix: String = format!("v2/debug/beacon/states/{}", &state_id);

    let endpoint = String::from(endpoint_prefix)+&state_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
    .timeout(None)
      .build()
        .unwrap();
    let endpoint = String::from(endpoint);
    let req = client.get(endpoint).send().unwrap();
    let resp: GenericResponse<BeaconState<MainnetEthSpec>> = req.json().unwrap();
    let state = resp.data;
    
    return state
}




pub fn make_snapshot(state: &BeaconState<MainnetEthSpec>)-> LightClientSnapshot{

    let header = state.latest_block_header();
    let current_committee = state.current_sync_committee().unwrap();
    let next_committee = state.next_sync_committee().unwrap();


    let snapshot = LightClientSnapshot{
        header: header.to_owned(),
        current_sync_committee: current_committee.clone(),
        next_sync_committee: next_committee.clone(),
    };

    return snapshot
}



pub fn get_block(api_key: &str, state_id: &str, endpoint_prefix: &str)->SignedBeaconBlock<MainnetEthSpec>{

    use serde_json::json;
    let block_body_suffix: String = format!("v2/beacon/blocks/{}", &state_id);
    let endpoint = String::from(endpoint_prefix)+&block_body_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
    .timeout(None)
      .build()
        .unwrap();

    let req = client.get(endpoint).send().unwrap();
    let resp: ForkVersionedResponse<SignedBeaconBlock<MainnetEthSpec>> = req.json().unwrap();
    let block = resp.data;
    //dbg!(block);

    return block

}

pub fn get_header(api_key: &str, state_id: &str, endpoint_prefix: &str)->BlockHeaderData{
    
    use serde_json::json;
    let block_body_suffix: String = format!("v1/beacon/headers/{}", &state_id);
    let endpoint = String::from(endpoint_prefix)+&block_body_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
    .timeout(None)
      .build()
        .unwrap();

    let req = client.get(endpoint).send().unwrap();
    let resp: GenericResponse<BlockHeaderData> = req.json().unwrap();
    let header: BlockHeaderData = resp.data;

    return header
}




// pub fn initialize_store(snapshot: LightClientSnapshot)->LightClientStore{
    
//     // initialize with empty update vec
//     let empty_updates: Vec<LightClientUpdate> = vec![];

//     let store = LightClientStore{
//         snapshot: snapshot,
//         valid_updates: empty_updates,
//     };


//     return store
// }


// pub fn update_store(mut store: LightClientStore, snapshot: LightClientSnapshot, update: LightClientUpdate)->LightClientStore{

//     // call class method of LightClientStore to add update to vec and refresh snasphot
//     store.add_update(update);
//     store.refresh_snapshot(snapshot);
    
//     return store
// }





