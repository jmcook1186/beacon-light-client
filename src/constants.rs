// grab precomputed generalized indices and vec[root] lengths
// from lodestar. floor_log2 gives the length of the list
// of roots in the branch connecting these leaves to the state tree root
pub const NEXT_SYNC_COMMITTEE_INDEX: u64 = 55;
pub const NEXT_SYNC_COMMITTEE_INDEX_FLOORLOG2: u64 = 5;
pub const FINALIZED_ROOT_INDEX: u64 = 105;
pub const FINALIZED_ROOT_INDEX_FLOOR_LOG2: u64 = 6;
pub const BYTES_PER_CHUNK: usize = 32;
pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
pub const MAXIMUM_LENGTH: usize = 2usize.pow((BYTES_PER_LENGTH_OFFSET * 8) as u32);
pub const N_VARIABLE_LENGTH: usize = 7;
