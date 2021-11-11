use std::format;
use crate::http_requests;
use crate::types::{BeaconBlockHeader,LightClientSnapshot, SyncCommittee, LightClientUpdate,
Fork, Eth1Data, Validator, Checkpoint, BeaconState};


// pub fn get_sync_committees(state: &serde_json::Value)->(SyncCommittee, SyncCommittee){

//     let current_sync_committee_pubkeys = state["data"]["current_sync_committee"]["pubkeys"].to_string();
//     let current_aggregate_pubkey = state["data"]["current_sync_committee"]["aggregate_pubkey"].to_string();
//     let next_sync_committee_pubkeys = state["data"]["next_sync_committee"]["pubkeys"].to_string();
//     let next_aggregate_pubkey = state["data"]["aggregate_pubkey"].to_string();

//     let current_sync_committee = SyncCommittee{pubkeys: current_sync_committee_pubkeys, aggregate_pubkey: current_aggregate_pubkey};
//     let next_sync_committee = SyncCommittee{pubkeys: next_sync_committee_pubkeys, aggregate_pubkey: next_aggregate_pubkey};

//     return (current_sync_committee, next_sync_committee)
// }


pub fn get_full_state_object(api_key: &str, node_id: &str, state_id: &str)->(serde_json::Value, BeaconState){
    
    let endpoint = format!("v2/debug/beacon/states/{}",state_id);
    let state_json: serde_json::Value = http_requests::generic_request(&api_key, &endpoint, &node_id).unwrap();


    fn string_to_vec_u8(string: String)->Vec<u8>{
        let _trimmed = &string.replace("\"", "");
        let result: Vec<u8> = _trimmed.as_bytes().to_vec();
        return result
    }

    fn string_to_bool(string: String)->bool{
        
        let _trimmed = &string.replace("\"", "");
        let result: bool = false;

        if _trimmed == "false"{
            return result
        }
        else if _trimmed == "true"{
            let result = true;
            return result
        }

        else{
          
          let s = _trimmed.parse::<u64>().unwrap();
        
          if s > 0{
            let result = true;
            return result
          }
        
          else{
            return result
          }
       }
    }
    

    fn string_to_u64(string: String)->u64{
        let _trimmed = &string.replace("\"", "");
        let result: u64 = _trimmed.parse::<u64>().unwrap();
        return result
    }

    fn string_to_vec_u64(string: String)->Vec<u64>{
        let _trimmed = &string.replace("\"", "").replace("[","").replace("]","");
        let mut result = Vec::new();
        let strings: Vec<String> = _trimmed.split(",").map(|s| s.to_string()).collect();
        for i in strings{
            let s = i.parse::<u64>().unwrap();
            result.push(s);
        }
        return result
    }

    let _genesis_time = state_json["data"]["genesis_time"].to_string();
    let genesis_time = string_to_u64(_genesis_time);

    let _genesis_validators_root = state_json["data"]["genesis_validators_root"].to_string();
    let genesis_validators_root = string_to_vec_u8(_genesis_validators_root);
    
    let _slot = state_json["data"]["slot"].to_string();
    let slot = string_to_u64(_slot);

    let _current_version = state_json["data"]["fork"]["current_version"].to_string();
    let current_version = string_to_vec_u8(_current_version);
    let _epoch = state_json["data"]["fork"]["epoch"].to_string();
    let epoch = string_to_u64(_epoch);
    let _previous_version = state_json["data"]["fork"]["previous_version"].to_string();
    let previous_version = string_to_vec_u8(_previous_version);

    let fork = Fork{
        current_version: current_version,
        epoch: epoch,
        previous_version: previous_version,
    };
    

    let _slot = state_json["data"]["latest_block_header"]["slot"].to_string();
    let slot = string_to_u64(_slot);
    let _proposer_index = state_json["data"]["latest_block_header"]["proposer_index"].to_string();
    let proposer_index = string_to_u64(_proposer_index);
    let _parent_root = state_json["data"]["latest_block_header"]["parent_root"].to_string();
    let parent_root = string_to_vec_u8(_parent_root);
    let _state_root = state_json["data"]["latest_block_header"]["state_root"].to_string();
    let state_root = string_to_vec_u8(_state_root);
    let _body_root = state_json["data"]["latest_block_header"]["body_root"].to_string();
    let body_root = string_to_vec_u8(_body_root);
    let block_header = BeaconBlockHeader{
        slot: slot,
        proposer_index: proposer_index,
        parent_root: parent_root,
        state_root: state_root,
        body_root: body_root,
    };

    let _block_roots = state_json["data"]["block_roots"].to_string();
    let block_roots = string_to_vec_u8(_block_roots);

    let _state_roots = state_json["data"]["state_roots"].to_string();
    let state_roots = string_to_vec_u8(_state_roots);

    let _historical_roots = state_json["data"]["historical_roots"].to_string();
    let historical_roots = string_to_vec_u8(_historical_roots);

    let _deposit_root = state_json["data"]["eth1_data"]["deposit_root"].to_string();
    let deposit_root = string_to_vec_u8(_deposit_root);
    let _deposit_count = state_json["data"]["eth1_data"]["deposit_count"].to_string();
    let deposit_count = string_to_u64(_deposit_count);
    let _block_hash = state_json["data"]["eth1_data"]["block_hash"].to_string();
    let block_hash = string_to_vec_u8(_block_hash);

    let eth1_data =  Eth1Data{
        deposit_root: deposit_root,
        deposit_count: deposit_count,
        block_hash: block_hash,
    };



    // begin awful process of constructing list of Eth1Data objects from json
    ////////////////////////////////////////////////////////////////////////
    let temp = state_json["data"]["eth1_data_votes"].to_string();
    
    let s: Vec<String> = temp.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    
    let mut eth1_data_votes =  Vec::new();

    let mut deproot_vec = Vec::new();
    let mut depcount_vec = Vec::new();
    let mut blockhash_vec = Vec::new();

    for i in 0..s.len()-1{

        if i %3==0{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_vec_u8(sss);
          deproot_vec.push(val);
        }
        else if i%3==1{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_u64(sss);
          depcount_vec.push(val);
          }
        else if i%3==2{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_vec_u8(sss);
          blockhash_vec.push(val);
        }
    }

    for i in 0..blockhash_vec.len()-1{

        let b = Eth1Data{
            deposit_root: deproot_vec[i].to_vec(),
            deposit_count: depcount_vec[i],
            block_hash: blockhash_vec[i].to_vec(),
        };

        eth1_data_votes.push(b)

    }
    /////////////////////////////////////////////
    /// end eth1_data_votes construction. breathe.


    let _eth1_deposit_index = state_json["data"]["eth1_deposit_index"].to_string();
    let eth1_deposit_index = string_to_u64(_eth1_deposit_index);


    // start construction of validators (vec of Validator structs)
    let temp = state_json["data"]["validators"].to_string();
    let s: Vec<String> = temp.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    
    
    //////////////////////////////////////////////////////
    // start equally awful construction of validators

    let mut validators =  Vec::new();
    let mut pubkey_vec = Vec::new();
    let mut withdrawal_cred_vec = Vec::new();
    let mut bal_vec = Vec::new();
    let mut slashed_vec = Vec::new();
    let mut eligibility_vec = Vec::new();
    let mut activation_vec = Vec::new();
    let mut exit_vec = Vec::new();
    let mut withdrawal_epoch_vec = Vec::new();

    for i in 0..s.len(){
        
        if i %8==0{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_vec_u8(sss);
          eligibility_vec.push(val);
        }

        else if i%8==1{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_u64(sss);
          activation_vec.push(val);

        }

        else if i%8==2{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_u64(sss);
          bal_vec.push(val);
        }

        else if i%8==3{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_u64(sss);
          exit_vec.push(val);
        }

        else if i%8==4{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_vec_u8(sss);
          pubkey_vec.push(val);
        }
        
        else if i%8==5{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_bool(sss);
          slashed_vec.push(val);
        }

        
        else if i %8==6{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_u64(sss);
          withdrawal_epoch_vec.push(val);
        }
        
        else if i%8==7{
          let ss:Vec<String> =s[i].split(":").map(|s| s.to_string()).collect();
          let sss =ss[1].to_owned();
          let val = string_to_vec_u8(sss);
          withdrawal_cred_vec.push(val);
        }
    }
    
    for i in 0..pubkey_vec.len()-1{
        
        let b = Validator{
          pubkey: pubkey_vec[i].to_owned(),
          withdrawal_credentials: withdrawal_cred_vec[i].to_owned(),
          effective_balance: bal_vec[i],
          slashed: slashed_vec[i],
          activation_eligibility_epoch: eligibility_vec[i].to_owned(),
          activation_epoch: activation_vec[i],
          exit_epoch: exit_vec[i],
          withdrawable_epoch: withdrawal_epoch_vec[i].to_owned(),
        };

        validators.push(b)
    }

    ////////////////////////////////////////
    /// end construction of validators

    let _balances = state_json["data"]["balances"].to_string();
    let balances = string_to_vec_u64(_balances);

    let _randao = state_json["data"]["randao_mixes"].to_string();
    let ss:Vec<String> =_randao.split(",").map(|s| s.to_string()).collect();
    let mut randao_mixes = Vec::new();
    for i in ss{
        let out = string_to_vec_u8(i);
        randao_mixes.push(out)
    }

    let _slashings = state_json["data"]["slashings"].to_string();
    let ss: Vec<String> = _slashings.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut slashings = Vec::new();
    for i in ss{
        let out = string_to_vec_u8(i);
        slashings.push(out)
    }

    let _prev_part = state_json["data"]["previous_epoch_participation"].to_string();
    let ss: Vec<String> = _slashings.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut previous_epoch_participation = Vec::new();
    for i in ss{
        let out = string_to_vec_u8(i);
        previous_epoch_participation.push(out)
    }

    let _current_part = state_json["data"]["previous_epoch_participation"].to_string();
    let ss: Vec<String> = _slashings.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut current_epoch_participation = Vec::new();
    
    for i in ss{
        let out = string_to_vec_u8(i);
        current_epoch_participation.push(out)
    }

    let _justification_bits = state_json["data"]["justification_bits"].to_string();
    let ss: Vec<String> = _slashings.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut justification_bits = Vec::new();
    for i in ss{
        let out = u64::from_str_radix(i.trim_start_matches("0x"), 16).unwrap();
        justification_bits.push(out)
    }

    let _prev_checkpoint = state_json["data"]["previous_justified_checkpoint"].to_string();
    let ss: Vec<String> = _prev_checkpoint.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let _epoch:Vec<String> = ss[0].to_string().split(":").map(|s| s.to_string()).collect();
    let epoch = _epoch[1].parse::<u64>().unwrap();
    let _root:Vec<String> = ss[1].to_string().split(":").map(|s| s.to_string()).collect();
    let root = _root[1].as_bytes().to_owned();
    
    let previous_justified_checkpoint = Checkpoint{
        epoch: epoch,
        root: root
    };

    let _curr_checkpoint = state_json["data"]["current_justified_checkpoint"].to_string();
    let ss: Vec<String> = _curr_checkpoint.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let _epoch:Vec<String> = ss[0].to_string().split(":").map(|s| s.to_string()).collect();
    let epoch = _epoch[1].parse::<u64>().unwrap();
    let _root:Vec<String> = ss[1].to_string().split(":").map(|s| s.to_string()).collect();
    let root = _root[1].as_bytes().to_owned();
    
    let current_justified_checkpoint = Checkpoint{
        epoch: epoch,
        root: root
    };

    let _finalized_checkpoint = state_json["data"]["finalized_checkpoint"].to_string();
    let ss: Vec<String> = _finalized_checkpoint.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let _epoch:Vec<String> = ss[0].to_string().split(":").map(|s| s.to_string()).collect();
    let epoch = _epoch[1].parse::<u64>().unwrap();
    let _root:Vec<String> = ss[1].to_string().split(":").map(|s| s.to_string()).collect();
    let root = _root[1].as_bytes().to_owned();
    
    let finalized_checkpoint = Checkpoint{
        epoch: epoch,
        root: root
    };

    
    let _inactivity = state_json["data"]["inactivity_scores"].to_string();
    let ss: Vec<String> = _inactivity.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut inactivity_scores = Vec::new();
    for i in ss{
        let out = i.parse::<u64>().unwrap();
        inactivity_scores.push(out);
    } 


    let _current_sync_comm_keys = state_json["data"]["current_sync_committee"]["pubkeys"].to_string();
    let ss: Vec<String> = _current_sync_comm_keys.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut current_sync_com_keys = Vec::new();
    for i in ss{
        let out: Vec<u8> = i.as_bytes().to_vec();
        current_sync_com_keys.push(out);
    }
    let _current_aggregate_pubkey = state_json["data"]["current_sync_committee"]["aggregate_pubkey"].to_string();
    let aggregate_pubkey: Vec<u8> = _current_aggregate_pubkey.as_bytes().to_vec();

    let current_sync_committee = SyncCommittee{
        pubkeys: current_sync_com_keys,
        aggregate_pubkey: aggregate_pubkey,
    };

    let _next_sync_comm_keys = state_json["data"]["next_sync_committee"]["pubkeys"].to_string();
    let ss: Vec<String> = _next_sync_comm_keys.replace(&['[', ']', '{', '\"', '}'][..], "").split(",").map(|s| s.to_string()).collect();
    let mut next_sync_com_keys = Vec::new();
    for i in ss{
        let out: Vec<u8> = i.as_bytes().to_vec();
        next_sync_com_keys.push(out);
    }
    let _next_aggregate_pubkey = state_json["data"]["next_sync_committee"]["aggregate_pubkey"].to_string();
    let aggregate_pubkey: Vec<u8> = _current_aggregate_pubkey.as_bytes().to_vec();

    let next_sync_committee = SyncCommittee{
        pubkeys: next_sync_com_keys,
        aggregate_pubkey: aggregate_pubkey,
    };


    let state = BeaconState{
        genesis_time: genesis_time,
        genesis_validators_root: genesis_validators_root,
        slot: slot,
        fork: fork,
        latest_block_header: block_header,
        block_roots: block_roots,
        state_roots: state_roots,
        historical_roots: historical_roots,
        eth1_data: eth1_data,
        eth1_data_votes: eth1_data_votes,
        eth1_deposit_index: eth1_deposit_index,
        validators: validators,
        balances: balances,
        randao_mixes: randao_mixes,
        slashings: slashings,
        previous_epoch_participation: previous_epoch_participation,
        current_epoch_participation: current_epoch_participation,
        justification_bits: justification_bits,
        previous_justified_checkpoint: previous_justified_checkpoint,
        current_justified_checkpoint: current_justified_checkpoint,
        finalized_checkpoint: finalized_checkpoint,
        inactivity_scores: inactivity_scores,
        current_sync_committee: current_sync_committee,
        next_sync_committee: next_sync_committee,

    };




    return (state_json, state)
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


