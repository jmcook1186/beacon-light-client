use rand::Rng;

// While the light client is implemented for the local testnet
// this function just grabs a random node_ID from 0-n_nodes
// and uses this as the Beacon node to serve http requests
// the ip address is localhost, this code appends a the port.

// Later when the light client migrates to public networks
// this will need to become proper node discovery
pub fn get_random_node_id(n_nodes: i16, base_port: i16) -> (String, i16) {
    let mut rng = rand::thread_rng();
    let node_number: i16 = rng.gen_range(1..n_nodes);
    let id: i16 = base_port + node_number;
    let node_id = id.to_string();

    return (node_id, node_number);
}
