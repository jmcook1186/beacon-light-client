// ALL THE FUNCS THAT WERE USEFUL ONCE BUT GOT DEPRECATED 
// MAYBE USEFUL AGAIN ONE DAY!

// use std::format;
// use std::fs;
// mod node_discovery;
// mod http_requests;
// mod build_objects;
// use eth2::types::*;
// use eth2_hashing::{hash};
// use std::sync::Arc;
// extern crate hex;
// use swap_or_not_shuffle::compute_shuffled_index;
// use bytes::{BufMut, BytesMut};


// A STORE OF MISCELLANEOUS FUNCS THAT I WROTE DURING DEV, GOT CHOPPED, BUT MIGHT
// BE USEFUL AGAIN LATER!



// pub fn u64_to_u8_32(var: u64) -> Vec<u8> {
//     let var_bytes = var.to_le_bytes();
//     let mut var_out: Vec<u8> = vec![];
//     // for positions 0-25 in count_vec, append zero (left pad vec)
//     for j in 0..(32 - var_bytes.len()) {
//         var_out.push(0u8);
//     }
//     // now append the 8 bytes of real data to the count vec
//     for j in var_bytes {
//         var_out.push(j);
//     }

//     assert_eq!(var_out.len(), 32);

//     return var_out;
// }

// pub struct LightClientUpdate(Container):
    
//     header: BeaconBlockHeader
//     # Next sync committee corresponding to the header
//     next_sync_committee: SyncCommittee
//     next_sync_committee_branch: Vector[Bytes32, floorlog2(NEXT_SYNC_COMMITTEE_INDEX)]
//     # Finality proof for the update header
//     finality_header: BeaconBlockHeader
//     finality_branch: Vector[Bytes32, floorlog2(FINALIZED_ROOT_INDEX)]
//     # Sync committee aggregate signature
//     sync_committee_bits: Bitvector[SYNC_COMMITTEE_SIZE]
//     sync_committee_signature: BLSSignature
//     # Fork version for the aggregate signature
//     fork_version: Version

///////////////////////////////////////////////////////////////////////
// HEAP OF FUNCS (TO BE ORGANISED INTO SENSIBLE PROJECT STRUCTURE LATER)


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


// pub fn get_seed(state: &BeaconState<MainnetEthSpec>, _epoch: &Epoch, domain_sync_committee: &String)->Vec<u8>{
    
//     const base: u64 = 2;
//     let EPOCHS_PER_HISTORICAL_VECTOR: u64 = base.pow(16); 
//     const MIN_SEED_LOOKAHEAD: u64 = 1;

//     let epoch = _epoch.as_u64();
//     let epoch_as_bytes = int_to_bytes32(epoch);
//     let idx = epoch % EPOCHS_PER_HISTORICAL_VECTOR - MIN_SEED_LOOKAHEAD -1;
//     let mix = state.randao_mixes()[idx as usize].as_bytes();
//     let mut domain_type: Vec<u8> = hex::decode(domain_sync_committee).unwrap();
    
//     domain_type.extend(epoch_as_bytes);
//     domain_type.extend(mix);

//     let seed = hash(&domain_type);

//     return seed
// }

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


// pub fn get_active_validators(state: &BeaconState<MainnetEthSpec>, epoch: &Epoch)->Vec<u64>{
//     // See consensus-spec:
//     //https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#get_active_validator_indices
//     let mut active_validator_indices: Vec<u64>= vec![];
    
//     let mut count:u64 = 0;
//     for i in 0..state.validators().len(){
//         if (state.validators()[i].activation_epoch <= epoch.to_owned()){
//         active_validator_indices.push(count);
//         }
//         count+=1;
//     }
//     return active_validator_indices
//   }



// pub struct LightClientSnapshot{
//     pub header: eth2::types::BeaconBlockHeader,
//     pub current_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
//     pub next_sync_committee: Arc<eth2::types::SyncCommittee<MainnetEthSpec>>,
// }


//Returns `int` as little-endian bytes with a length of 32.
// pub fn int_to_bytes32(int: u64) -> Vec<u8> {
//     let mut bytes = BytesMut::with_capacity(32);
//     bytes.put_u64_le(int);
//     bytes.resize(32, 0);
//     bytes.to_vec()
// }