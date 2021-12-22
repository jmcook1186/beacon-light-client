

pub fn expand_eth1_data_votes()->Vec<u8>{
    // set up vec to hold raw eth1_data_votes data
    let mut eth1_data_votes_vec: Vec<u8> = vec![];

    // start looping through eth1_data_votes, one iteration per eth1_data container
    println!(
        "Iterating over {:?} vals in eth1_data_votes",
        eth1_data_votes.len()
    );

    for i in 0..eth1_data_votes.len() {
        // set up temp vec that exists to pad count: u64 to [u8; 32]
        let mut count_vec: Vec<u8> = vec![];
        //extract necessary values from  eth1_data object
        let dep_root: Vec<u8> = eth1_data_votes[i].deposit_root.as_ssz_bytes();
        let count = eth1_data_votes[i].deposit_count.as_ssz_bytes();
        let block_hash: Vec<u8> = eth1_data_votes[i].block_hash.as_ssz_bytes();

        // now for each byte in each field, push to eth1_data_votes_vec.
        // The ordering is critical - 32 bytes from dep_root first then
        // 32 bytes from count_vec, 32 bytes fromblock_hash
        for j in dep_root {
            eth1_data_votes_vec.push(j)
        }

        for j in count {
            eth1_data_votes_vec.push(j)
        }

        for j in block_hash {
            eth1_data_votes_vec.push(j)
        }
    }

    // assert that the length of the serialized dataset is equal to the number of eth1_data containers
    // multiplied by the sum of the lengths of each of their elements (32 bytes for hashes, 8 bytes for u64)
    assert_eq!(
        eth1_data_votes_vec.len(),
        eth1_data_votes.len() * (32 + 32 + 8)
    );

    // after loop has finished, eth1_data_votes_vec is the serialized form of eth1_data_votes ready to be merkleized
    // To avoid mistakes with var naming, we can overwrite eth1_data_votes (vec of containers) with eth1_data_votes_vec
    // (vec of bytes) and just use var eth1_data_votes from here on.
    let eth1_data_votes = eth1_data_votes_vec

}