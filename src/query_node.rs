use std::format;
use std::fs;
use serde_json::{Value};
use reqwest::{
  header::{HeaderMap, HeaderValue}};
use crate::http_requests;



pub fn get_sync_committee_ids(api_key: &str, node_id: &str, state_id: &str)->Vec<u8>{

    // get list of validators included in sync commitee 
    let endpoint = format!("beacon/states/{}/sync_committees",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();
    
    // grab the validator ids only and parse them as u8s to vector validators_vec
    let validators = result["data"]["validators"].to_string();
    let _trimmed = &validators[1..validators.len() - 1].replace("\"", "");
    let validator_ids: Vec<u8> = _trimmed.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    assert_eq!(validator_ids.len(), 512);

    return validator_ids
}


pub fn get_sync_committee_pubkeys(api_key: &str, node_id: &str, state_id: &str, validator_ids: Vec<u8>)->Vec<Vec<u8>>{

    // grab the validator info from the /validators endpoint - includes validator's pubkeys
    let endpoint = format!("beacon/states/{}/validators",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();

    // for the validators included in the sync committee, get their pubkeys and parse as u8
    let mut pubkeys_str = Vec::new();
    
    
    for i in validator_ids{
        // if validator is in sync committee AND status is active
        // first traverse the json to find the right data, 
        // then remove quotation marks
        if result["data"][i as usize]["status"].to_string().contains("active"){
        pubkeys_str.push(result["data"][i as usize]["validator"]["pubkey"].to_string().replace("\"", ""));
        }

    }

    // now recast as u8 bytes and push to pubkeys vec
    let mut pubkeys = Vec::new();
    for i in pubkeys_str{
        pubkeys.push(i.into_bytes());
    }

    return pubkeys

}


pub fn get_block_header_info(api_key: &str, node_id: &str, state_id: &str)->(String, String, Vec<u8>){

    let endpoint = format!("beacon/headers/{}",state_id);
    let result: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();
    let header = result.to_string();
    let root = result["data"]["root"].to_string().replace("\"", "");
    let agg_sig = result["data"]["header"]["signature"].to_string().replace("\"", "");

    // now recast as u8 bytes and push to pubkeys vec
    let mut aggKey = agg_sig.into_bytes();

    return (header, root, aggKey)
}