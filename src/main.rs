use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
mod query_node;
//mod build_objects;
use std::mem;
use std::option;
use eth2::types::{BeaconState, GenericResponse, MainnetEthSpec, SyncCommittee, BeaconBlockHeader, Epoch};
use std::sync::Arc;
use math::round;

fn main(){
    
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "head";
    let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope"); 
    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);

    let state: BeaconState<MainnetEthSpec> = get_state(&api_key, &state_id, &endpoint_prefix);
    let snapshot = make_snapshot(&state);
    
    
    let epoch: Epoch =state.slot().epoch(32); //32 slots per epoch https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_epoch_at_slot
    
    println!("{:?}",epoch);
    
}


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


// pub fn get_next_sync_committee_indices(state: BeaconState<MainnetEthSpec>) -> Something:

//     """
//     Return the sync committee indices, with possible duplicates, for the next sync committee.
//     """
//     use math::round;

//     slot = state.slot().parse::<u32>().unwrap();
//     epoch = round::floor(slot/32,0) + 1; //32 slots per epoch https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_epoch_at_slot
//     println!("{:?}",epoch);




    // MAX_RANDOM_BYTE = 2**8 - 1
    // active_validator_indices = get_active_validator_indices(state, epoch)
    // active_validator_count = uint64(len(active_validator_indices))
    // seed = get_seed(state, epoch, DOMAIN_SYNC_COMMITTEE)
    // i = 0
    // sync_committee_indices: List[ValidatorIndex] = []
    // while len(sync_committee_indices) < SYNC_COMMITTEE_SIZE:
    //     shuffled_index = compute_shuffled_index(uint64(i % active_validator_count), active_validator_count, seed)
    //     candidate_index = active_validator_indices[shuffled_index]
    //     random_byte = hash(seed + uint_to_bytes(uint64(i // 32)))[i % 32]
    //     effective_balance = state.validators[candidate_index].effective_balance
    //     if effective_balance * MAX_RANDOM_BYTE >= MAX_EFFECTIVE_BALANCE * random_byte:
    //         sync_committee_indices.append(candidate_index)
    //     i += 1
    // return sync_committee_indices



pub struct LightClientSnapshot{
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
}

