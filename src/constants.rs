// grab precomputed generalized indices and vec[root] lengths
// from lodestar. floor_log2 gives the length of the list
// of roots in the branch connecting these leaves to the state tree root
pub const NEXT_SYNC_COMMITTEE_INDEX: u64 = 55;
pub const NEXT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2: u64 = 5;
pub const FINALIZED_ROOT_INDEX: u64 = 105;
pub const FINALIZED_ROOT_INDEX_FLOOR_LOG2: u64 = 6;
pub const BYTES_PER_CHUNK: usize = 32;
pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
pub const BITS_PER_BYTE: usize = 8;
pub const MAXIMUM_LENGTH: usize = 2usize.pow((BYTES_PER_LENGTH_OFFSET * BITS_PER_BYTE) as u32);
pub const N_VARIABLE_LENGTH: usize = 7;
pub const SLOTS_PER_HISTORICAL_ROOT: usize = 8192;
pub const JUSTIFICATION_BITS_LENGTH: u64 = 4;
pub const VALIDATOR_REGISTRY_LIMIT: usize = 1099511627776;
pub const HISTORICAL_ROOTS_LIMIT: usize = 16777216;
pub const SYNC_COMMITTEE_SIZE: usize = 512;
pub const EPOCHS_PER_SYNC_COMMITTEE_PERIOD: usize = 256;
pub const EPOCHS_PER_SLASHINGS_VECTOR: usize = 8192;
