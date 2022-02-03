# Ethereum Beacon Chain Light Client

*** WARNING This light client is still in very early development! ***

The initial aim of this repository is to build a prototype Beacon Chain light client in Rust following the following specs: https://github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client. The initial aim is to have a working prototype that tracks finalized blocks, then later expanding the scope to track the head of the chain. 

## Design

To begin with the light client will follow a server/client model. The relevant information required for a light client is available by http requests to an active Beacon node. A light client server will make those requests and assemble them into light client update objects, which will then be served to the light client itself.

## Instructions

First spin up a local testnet using Ganache as the execution layer (clone lighthouse repo, navigate to lighthouse/scripts/local_testnet, run `./start_local_testnet.sh`). Then cargo run in the beacon-light-client top level directory. At the current stagte of development this will get all the relevant data, serialize and merkleize the beacon state object and print some debugging logs to the console.

## Current Functionality

The light client randomly selects a Beacon node to connect to from the N nodes spun up as part of a local Lighthouse testnet (N can be set in /lighthouse/scripts/local_testnetvars.env). The server then creates an initial store object by parsing data from the `beacon_block_body` and `beacon_state` objects associated with the most recent finalized block requested from the Beacon Node. Then, the same process is then repeated for the head of the chain, with new data used to update `store`.

## Dev Notes

For light-client dev make sure the testnet BN's are running altair. The defaults in vars.env set the altair hard fork to block 18million-ish. Set it to 0, then the altair endpoints will be available immediately.

## finality_branch and sync_committee_branch

see SSZ_notes.md


