# Ethereum Beacon Chain Light Client

*** WARNING This light client is still in very early development! ***

The initial aim of this repository is to build a prototype Beacon Chain light client in Rust following the following specs: https://github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client. 

This version is **NOT YET FUNCTIONAL** even as a prototype. The calculated state root does not match the downloaded state root, indicating a bug in my merkleization code that I am yet to find and fix. The light client objects are not currently served at all - running this code just builds the light client objects for the latest finalized blocks and reports some information from them to the console. The next steps, after fixing the merkleization functions, are to serve the light client objects over http so that they can be requested by a light client.

## Design

To begin with the light client will follow a server/client model. The relevant information required for a light client is available by http requests to an active Beacon node. A light client server will make those requests and assemble them into light client update objects, which will then be served to the light client itself.

## Instructions

First spin up a local testnet using Ganache as the execution layer (clone lighthouse repo, navigate to lighthouse/scripts/local_testnet, run `./start_local_testnet.sh`). Then cargo run in the beacon-light-client top level directory. At the current stagte of development this will get all the relevant data, serialize and merkleize the beacon state object and print some debugging logs to the console.

## Current Functionality

The light client randomly selects a Beacon node to connect to from the N nodes spun up as part of a local Lighthouse testnet (N can be set in /lighthouse/scripts/local_testnet/vars.env). The server then retrieves the `beacon_state` from the node, SSZ serializes it, merkelizes it, and calculates the necessary proofs. it then uses the proofs and other state information to construct light client updates.

The next steps are to serve those updates over http and construct a client that retrieves the updates and processes them in each slot.

## Dev Notes

For light-client dev make sure the testnet BN's are running altair. The defaults in vars.env set the altair hard fork to block 18million-ish. Set it to 0, then the altair endpoints will be available immediately.

Make sure you are using the lts version of node, the latest version breaks the local testnet (ganache fails to run). 
`nvm use --lts`

