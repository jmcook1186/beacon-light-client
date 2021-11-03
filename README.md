# Lighthouse Light Client

This light client is still in very early development. 
Initial aim is to build a light client that gets its data via http request to Beacon node.

This will involve building an intermediate server between the Beacon node and a light client that
makes requests to the BN and builds the necessary light client update objects.


Current functionality:
 - spin up local testnet with ganache execution layer, N beacon nodes
 - randomly choose beacon node
 - get node's IP address
 - get finalized header and committees from http request to node
 - update LightClientSnapshot object


For light-client dev make sure the BN is running altair. The defaults in vars.env set the altair hard fork to block 18million-ish. Set it to 0, then the altair endpoints will be available immediately.

## Questions:

- how often should light client update? Every epoch? Every slot?
- current stage - reading in api-token to pass to /lighthouse/validators endpoint to get sigs

see spec here:
- https://github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client