import time;


use std::net::IpAddr;

struct Node {
    id: u128,
    routing_lists: RoutingLists
}

// primary kbucket, secondary kbucket
type RoutingLists = [[KBucket; 2]; 128];

impl RoutingLists {
    fn store_node(node: NodeReference) {

    }

    fn retrive_node(id: u128) {

    }
}

usize K_BUCKET_K = 20;

type KBucket = [NodeReference; K_BUCKET_K];

struct NodeReference {
    ip_address: IpAddr,
    port: u16,
    id: u128,
    last_seen: PreciseTime,
}
