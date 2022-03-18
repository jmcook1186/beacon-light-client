use std::format;
extern crate hex;
use crate::constants;
use crate::light_client_types::{LightClientSnapshot, LightClientUpdate};
use crate::merkle_proofs;
use crate::merkleize;
use crate::serialize;
use eth2::types::*;

pub fn get_state(state_id: &str, endpoint_prefix: &str) -> BeaconState<MainnetEthSpec> {
    let state_suffix: String = format!("v2/debug/beacon/states/{}", &state_id);

    let endpoint = String::from(endpoint_prefix) + &state_suffix;

    println!("{:?}", endpoint);
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

pub fn get_block(state_id: &str, endpoint_prefix: &str) -> SignedBeaconBlock<MainnetEthSpec> {
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

pub fn get_header(state_id: &str, endpoint_prefix: &str) -> BlockHeaderData {
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

pub fn build_update(
    state: BeaconState<MainnetEthSpec>,
    block: SignedBeaconBlock<MainnetEthSpec>,
    finality_header: BlockHeaderData,
) -> LightClientUpdate {
    // ssz serialize the state object, pad and hash each field, build merkle tree
    let (serialized_state, sizes, offsets) = serialize::serialize_beacon_state(&state);
    let chunks = merkleize::generate_chunks(&serialized_state, &sizes, &offsets);
    let tree: Vec<Vec<u8>> = merkleize::merkle_tree(chunks);

    let sync_comm_branch: Vec<Vec<u8>> =
        merkle_proofs::get_branch(&tree, constants::NEXT_SYNC_COMMITTEE_INDEX);
    assert_eq!(
        sync_comm_branch.len() as u64,
        constants::NEXT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2
    );

    // let finality_branch: Vec<Vec<u8>> = merkle_proofs::get_branch(&tree, constants::FINALIZED_ROOT_INDEX);
    // assert_eq!(sync_comm_branch.len() as u64, constants::FINALIZED_ROOT_INDEX_FLOOR_LOG2);

    // TODO:
    // temporary branch until merklization is fixed!!
    let finality_branch: Vec<Vec<u8>> =
        merkle_proofs::get_branch(&tree, constants::NEXT_SYNC_COMMITTEE_INDEX);
    assert_eq!(
        sync_comm_branch.len() as u64,
        constants::NEXT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2
    );

    // sync_aggregate comes straight from the block body - this is the source of sync_committee_bits
    let aggregate: SyncAggregate<MainnetEthSpec> =
        block.message().body().sync_aggregate().unwrap().to_owned();

    // build update object
    let update = LightClientUpdate {
        header: state.latest_block_header().to_owned(),
        next_sync_committee: state.next_sync_committee().unwrap().to_owned(),
        next_sync_committee_branch: sync_comm_branch,
        finality_header: finality_header,
        finality_branch: finality_branch,
        sync_committee_bits: aggregate.sync_committee_bits,
        fork_version: state.fork().current_version,
    };

    return update;
}
