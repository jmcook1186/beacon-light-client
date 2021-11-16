## The Beacon State Object
### (With specific reference to Lighthouse/Rust)

The `beacon_state` object is a core struct that must be agreed upon across the entire network as it represents the current state of the Beacon Chain. The root of its Merkleized form is used to validate a Beacon (light) client's view of reality. For this reason, the `beacon_state` has a spec that defines the type, size and order of each value stored within it. A Beacon client must parse the `beacon_state` object conforming precisely to the spec or else its root hash will not match the rest of the network.



```Rust
let epoch: Epoch =state.slot().epoch(32); //32 slots per epoch https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_epoch_at_slot
let next_epoch = epoch+1;
println!("{:?} {:?}",epoch, next_epoch);
```

