use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct Network {
    node_quantity: usize,
    adj: Vec<Vec<usize>>,
    cap: Vec<Vec<usize>>
}

impl Network {
    pub fn new() -> Self {
        Network { node_quantity: 0, adj: Vec::new(), cap: Vec::new() }
    }
    pub fn new_hypercube(n: usize) -> Self {
        todo!();
        Network { node_quantity: 0, adj: Vec::new(), cap: Vec::new() }
    }
}

pub fn esmond_karp(network: &Network) {
    todo!()
}

pub fn dinic(network: &Network) {
    todo!()
}
