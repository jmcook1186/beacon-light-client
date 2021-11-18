use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
// use http_api::version::{fork_versioned_response, unsupported_version_rejection, V1};
use eth2::types::{BeaconState, GenericResponse, MainnetEthSpec, Epoch, SignedBeaconBlock, BeaconBlockBodyRef, ForkVersionedResponse};
use eth2_hashing::{hash};
use std::sync::Arc;
extern crate hex;
use swap_or_not_shuffle::compute_shuffled_index;
use bytes::{BufMut, BytesMut};

fn main(){
    
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let state_id = "finalized";
    let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope"); 
    let endpoint_prefix: String = format!("http://localhost:{}/eth/", &node_id);

    // download beacon_state and make a snapshot
    let state: BeaconState<MainnetEthSpec> = get_state(&api_key, &state_id, &endpoint_prefix);
    let snapshot = make_snapshot(&state);
    
    // download a beacon block and extract the body
    let block = get_block(&api_key, &state_id, &endpoint_prefix);
    let body: BeaconBlockBodyRef<MainnetEthSpec> = block.message().body();
    println!("{:?}",body.randao_reveal());
    
        
}




///////////////////////////////////////////////////////////////////////
// HEAP OF FUNCS (TO BE ORGANISED INTO SENSIBLE PROJECT STRUCTURE LATER)


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


// pub fn get_next_sync_committee_indices(state: &BeaconState<MainnetEthSpec>){

//     // """
//     // Return the sync committee indices, with possible duplicates, for the next sync committee.
//     // """

//     // divide slot by 32 slots per epoch using method of Slot type, see type definition:
//     // /home/joe/Code/lighthouse/consensus/types/src/slot_epoch.rs
//     // and in spec: https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_epoch_at_slot
    
//     const SYNC_COMMITTEE_SIZE:u64 = 512;
//     let current_epoch: Epoch =state.slot().epoch(32);
//     let next_epoch = current_epoch+1; 
    
//     //const MAX_RANDOM_BYTE: u64 = 2**8 - 1;

//     let active_validator_indices = get_active_validators(&state, &current_epoch);
//     let active_validator_count = active_validator_indices.len();
//     let domain_sync_committee = "07000000".to_string();

//     let seed = get_seed(&state, &current_epoch, &domain_sync_committee);
    
//     let idx: usize = 135;
//     let index_count: usize = 1;
    
//     let mut count = 0;
//     sync_committee_indices: List[u64] =[];
//     while len(sync_committee_indices< SYNC_COMMITTEE_SIZE){
        
//         let shuffled_index = compute_shuffled_index((count % active_validator_count).as usize, active_validator_count as size, seed);
//         let candidate_index = active_validator_indices[shuffled_index];
        
        
//         uint_to_bytes


//         let random_byte = hash(seed + uint_to_bytes(uint64(i // 32)))[i % 32]
//     }

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
//}




// SPEC: https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_shuffled_index

// def compute_shuffled_index(index: uint64, index_count: uint64, seed: Bytes32) -> uint64:
//     """
//     Return the shuffled index corresponding to ``seed`` (and ``index_count``).
//     """
//     assert index < index_count

//     # Swap or not (https://link.springer.com/content/pdf/10.1007%2F978-3-642-32009-5_1.pdf)
//     # See the 'generalized domain' algorithm on page 3
//     for current_round in range(SHUFFLE_ROUND_COUNT):
//         pivot = bytes_to_uint64(hash(seed + uint_to_bytes(uint8(current_round)))[0:8]) % index_count
//         flip = (pivot + index_count - index) % index_count
//         position = max(index, flip)
//         source = hash(
//             seed
//             + uint_to_bytes(uint8(current_round))
//             + uint_to_bytes(uint32(position // 256))
//         )
//         byte = uint8(source[(position % 256) // 8])
//         bit = (byte >> (position % 8)) % 2
//         index = flip if bit else index

//     return index





pub fn get_seed(state: &BeaconState<MainnetEthSpec>, _epoch: &Epoch, domain_sync_committee: &String)->Vec<u8>{
    
    const base: u64 = 2;
    let EPOCHS_PER_HISTORICAL_VECTOR: u64 = base.pow(16); 
    const MIN_SEED_LOOKAHEAD: u64 = 1;

    let epoch = _epoch.as_u64();
    let epoch_as_bytes = int_to_bytes32(epoch);
    let idx = epoch % EPOCHS_PER_HISTORICAL_VECTOR - MIN_SEED_LOOKAHEAD -1;
    let mix = state.randao_mixes()[idx as usize].as_bytes();
    let mut domain_type: Vec<u8> = hex::decode(domain_sync_committee).unwrap();
    
    domain_type.extend(epoch_as_bytes);
    domain_type.extend(mix);

    let seed = hash(&domain_type);

    return seed
}

    // let seed = hash(b'0x02000000' + epoch_as_bytes + mix);
    // need a merkle hash library
    
//     SPEC
//     def get_seed(state: BeaconState, epoch: Epoch, domain_type: DomainType) -> Bytes32:
//         """
//         Return the seed at ``epoch``.
//         """
//         mix = get_randao_mix(state, Epoch(epoch + EPOCHS_PER_HISTORICAL_VECTOR - MIN_SEED_LOOKAHEAD - 1))  # Avoid underflow
//         return hash(domain_type + uint_to_bytes(epoch) + mix)
// }


pub fn get_active_validators(state: &BeaconState<MainnetEthSpec>, epoch: &Epoch)->Vec<u64>{
    // See consensus-spec:
    //https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#get_active_validator_indices
    let mut active_validator_indices: Vec<u64>= vec![];
    
    let mut count:u64 = 0;
    for i in 0..state.validators().len(){
        if (state.validators()[i].activation_epoch <= epoch.to_owned()){
        active_validator_indices.push(count);
        }
        count+=1;
    }
    return active_validator_indices
  }



pub struct LightClientSnapshot{
    pub header: eth2::types::BeaconBlockHeader,
    pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
    pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
}


//Returns `int` as little-endian bytes with a length of 32.
pub fn int_to_bytes32(int: u64) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(32);
    bytes.put_u64_le(int);
    bytes.resize(32, 0);
    bytes.to_vec()
}