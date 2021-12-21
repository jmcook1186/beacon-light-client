// grab precomputed generalized indices and vec[root] lengths
// from lodestar. floor_log2 gives the length of the list
// of roots in the branch connecting these leaves to the state tree root
pub const NEXT_SYNC_COMMITTEE_INDEX: u64 = 55;
pub const NEXT_SYNC_COMMITTEE_INDEX_FLOORLOG2: u64 = 5;
pub const FINALIZED_ROOT_INDEX: u64 = 105;
pub const FINALIZED_ROOT_INDEX_FLOOR_LOG2: u64 = 6;
