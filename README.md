# Lighthouse Light Client

*** This light client is still in very early development ***
It aims to be a functional prototype, but is curently way off even that!

The initial aim of this repository is to build a prototype Beacon Chain light client in Rust following the following specs: https://github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client

## Design

To begin with the light client will follow a server/client model. The relevant information required for a light client is available by http requests to an active Beacon node. A light client server will make those requests and assemble them into light client update objects, which will then be served to the light client itself.


## Instructions

First spin up a local testnet using Ganache as the execution layer
Then cargo run in the light-client directory
At present, this will print the public keys of the 512 sync comittee validators to the console.



For light-client dev make sure the BN is running altair. The defaults in vars.env set the altair hard fork to block 18million-ish. Set it to 0, then the altair endpoints will be available immediately.

