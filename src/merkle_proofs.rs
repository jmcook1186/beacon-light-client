extern crate hex;
use bit_vec::BitVec;
use math::round;


// flatten tree into vec of u8s
// each 32-byte chunk is then a node
// with te generalized index of
// each element counting backwards
// i.e. last element is the root


// tree is the ful merkle tree as a vec of vecs of bytes
// gen_index is the position in the tree of the leaf
// whose branch we want to return

pub fn get_branch(tree: &Vec<Vec<u8>>, tree_idx: u64)->Vec<Vec<u8>>{
    
    let mut branch: Vec<Vec<u8>> = vec![];
    let branch_idxs: Vec<u64> = get_branch_indices(tree_idx);
    for i in branch_idxs.iter(){
        branch.push(tree[*i as usize].clone());
    }

    return branch
}

pub fn get_branch_indices(tree_idx: u64)->Vec<u64>{
    
    let mut o: Vec<u64> = vec![];
    o.push(generalized_index_sibling(tree_idx));

    while o[o.len()-1]>1{
        o.push(generalized_index_parent(o[o.len()-1]));
    }
    return o[0..o.len()-1].to_vec()
}


// returns the length of the branch linking
// given node to root
pub fn get_generalized_index_length(tree_idx: u64)->u64{
    let idx = tree_idx as f64;

    return idx.log2() as u64
}



pub fn generalized_index_sibling(idx: u64)->u64{
    return idx.pow(1);
}


pub fn generalized_index_parent(idx: u64)->u64{
    return round::floor((idx/2) as f64, 0) as u64
}

