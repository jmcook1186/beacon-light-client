use std::format;
extern crate hex;
use crate::constants::{FINALIZED_ROOT_INDEX, NEXT_SYNC_COMMITTEE_INDEX};
use crate::light_client_types::{LightClientSnapshot, LightClientUpdate};
use crate::serialize;
use eth2::types::*;
use ethereum_types::H256;

pub fn get_state(
    api_key: &str,
    state_id: &str,
    endpoint_prefix: &str,
) -> BeaconState<MainnetEthSpec> {
    let state_suffix: String = format!("v2/debug/beacon/states/{}", &state_id);

    let endpoint = String::from(endpoint_prefix) + &state_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()
        .unwrap();
    let endpoint = String::from(endpoint);
    let req = client.get(endpoint).send().unwrap();
    let resp: GenericResponse<BeaconState<MainnetEthSpec>> = req.json().unwrap();
    let state = resp.data;

    return state;
}

pub fn make_snapshot(state: &BeaconState<MainnetEthSpec>) -> LightClientSnapshot {
    let header = state.latest_block_header();
    let current_committee = state.current_sync_committee().unwrap();
    let next_committee = state.next_sync_committee().unwrap();

    let snapshot = LightClientSnapshot {
        header: header.to_owned(),
        current_sync_committee: current_committee.clone(),
        next_sync_committee: next_committee.clone(),
    };

    return snapshot;
}

pub fn get_block(
    api_key: &str,
    state_id: &str,
    endpoint_prefix: &str,
) -> SignedBeaconBlock<MainnetEthSpec> {
    let block_body_suffix: String = format!("v2/beacon/blocks/{}", &state_id);
    let endpoint = String::from(endpoint_prefix) + &block_body_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()
        .unwrap();

    let req = client.get(endpoint).send().unwrap();
    let resp: ForkVersionedResponse<SignedBeaconBlock<MainnetEthSpec>> = req.json().unwrap();
    let block = resp.data;
    //dbg!(block);

    return block;
}

pub fn get_header(api_key: &str, state_id: &str, endpoint_prefix: &str) -> BlockHeaderData {
    let block_body_suffix: String = format!("v1/beacon/headers/{}", &state_id);
    let endpoint = String::from(endpoint_prefix) + &block_body_suffix;
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()
        .unwrap();

    let req = client.get(endpoint).send().unwrap();
    let resp: GenericResponse<BlockHeaderData> = req.json().unwrap();
    let header: BlockHeaderData = resp.data;

    return header;
}

// pub fn get_update(
//     state: BeaconState<MainnetEthSpec>,
//     block: SignedBeaconBlock<MainnetEthSpec>,
//     finality_header: BlockHeaderData,
// ) -> LightClientUpdate {
//     // sync_aggregate comes straight from the block body - this is the source of sync_committee_bits
//     let aggregate: SyncAggregate<MainnetEthSpec> =
//         block.message().body().sync_aggregate().unwrap().to_owned();

//     // serialize the beacon_state and chunk it into 32 byte leaves.
//     // merklize the chunked vector, return the merkle tree and the depth of the tree
//     let leaves: Vec<H256> = serialize::to_h256_chunks(&state);
//     let (tree, tree_depth) = serialize::get_merkle_tree(&leaves);

//     // get branches (vectors of hashes at nodes connecting leaf to root)
//     let sync_comm_branch: Vec<H256> = serialize::get_branch(
//         &tree,
//         NEXT_SYNC_COMMITTEE_INDEX as usize,
//         tree_depth as usize,
//     );
//     let finality_branch: Vec<H256> =
//         serialize::get_branch(&tree, FINALIZED_ROOT_INDEX as usize, tree_depth as usize);

//     // build update object
//     let update = LightClientUpdate {
//         header: state.latest_block_header().to_owned(),
//         next_sync_committee: state.next_sync_committee().unwrap().to_owned(),
//         next_sync_committee_branch: sync_comm_branch,
//         finality_header: finality_header,
//         finality_branch: finality_branch,
//         sync_committee_bits: aggregate.sync_committee_bits,
//         fork_version: state.fork().current_version,
//     };

//     return update;
// }
