# Lighthouse Light Client

*** This light client is still in very early development ***
It aims to be a functional prototype, but is curently way off even that!

The initial aim of this repository is to build a prototype Beacon Chain light client in Rust following the following specs: https://github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client

## Design

To begin with the light client will follow a server/client model. The relevant information required for a light client is available by http requests to an active Beacon node. A light client server will make those requests and assemble them into light client update objects, which will then be served to the light client itself.

## Instructions

First spin up a local testnet using Ganache as the execution layer
Then cargo run in the light-client directory

## Current Functionality

The light client randomly selects a beacon node to connect to. The server then creates an initial store object by parsing data from the `beacon_block_body` and `beacon_state` objects associated with the most recent finalized block requested from the Beacon Node. Then, the same process is then repeated for the head of the chain, with new data used to update `store`.


## Dev Notes

For light-client dev make sure the testnet BN's are running altair. The defaults in vars.env set the altair hard fork to block 18million-ish. Set it to 0, then the altair endpoints will be available immediately.

## finality_branch and sync_committee_branch

I have not yet worked out precisely how to derive these values. These fields in the LightClientUpdate object refer to the Merkle branches that connect the finalized root and the sync_committee object respectively to the beacon state root. 

### current knowledge:

- what is FINALIZED_ROOT_INDEX? This is a generalized index that locates the position of the finalized root (beaconState.finalized_checkpoint.root) in the merkle-tree representation of the beaconState object.
- what is the finality_branch? This is the set of indices, cast as a Vec<u8>, representing each node in the Merklized beaconState object required to generate a proof that the finalized root was used to generate the beacon state root. This is like a path through the tree encoded as integers.
- A "path" through the merkle-tree, e.g. a json traverse through the state object to the finalized root would be:
    
    `state["data"]["finalized_checkpoint"]["root"]`
    which can be expressed as a path:
    `["data", "finalized_checkpoint", "root"]`

    This is then converted to a generalized index. A generalized index is an integer that represents a node in a binary merkle tree where each node has a generalized index `2 ** depth + index in row`. 

    ```
          1           --depth = 0  2**0 + 1 = 1
      2       3       --depth = 1  2**1 + 0 = 2, 2**1+1 = 3
    4   5   6   7     --depth = 2  2**2 + 0 = 4, 2**2 + 1 = 5, 2**2 + 2 = 6, 2**2 + 3 = 7
    
    ```
    
    This representation yields a node index for each piece of data in the merkle tree. We need to find the indices for the beacon state root and a) the sync_commitee object, and b) the finalized root in the merklized beacon state object. This is all that is needed for the update object. Downstream, these indices will be used for verification by identifying the hash partners of each node in the branch between the sync_committee/finalized root and the state root. Then, sequentially hashing each node with its partner should yield a hash equal to the state root.

Questions: how does a json object map to a generalized index?
 - the beaconState seems to be pretty much flat - ie. there is no nesting of fields


In Lodestar: 

    finalityBranch =   syncAttestedState.tree.getSingleProof(BigInt(FINALIZED_ROOT_INDEX))

    packages/params/test/unit/constants.test.ts
    const FINALIZED_ROOT_INDEX = Number(ssz.altair.BeaconState.getPathGindex(["finalizedCheckpoint", "root"]));

Immediate objectives are to determine the type and source of the input data and the functions that transform them into the desired branch values. Then, explore the available Rust crates that could help achieve it - if none available try to write funcs in get_branches.rs that do the necessary calculations.