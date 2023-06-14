use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Debug, Clone)]
pub struct Network {
    node_quantity: usize,
    adj: Vec<Vec<u8>>,
    cap: Vec<Vec<usize>>
}

impl Network {
    pub fn new(node_quantity: usize) -> Self {
        let adj: Vec<Vec<u8>> = vec![vec![0; node_quantity]; node_quantity];
        let cap: Vec<Vec<usize>> = vec![vec![0; node_quantity]; node_quantity];
        Network { node_quantity, adj, cap }
    }
    pub fn new_hypercube(n: u32) -> Self {
        let mut rng: Pcg64 = Pcg64::from_entropy();
        let node_quantity= 2_usize.pow(n);
        let mut adj: Vec<Vec<u8>> = vec![vec![0; node_quantity]; node_quantity];
        let mut cap: Vec<Vec<usize>> = vec![vec![0; node_quantity]; node_quantity];
        for v in 0..node_quantity {
            for u in 0..node_quantity {
                if v > u {
                    if hamming_distance(v, u) == 1 {
                        adj[v][u] = 1;
                    }
                }
            }
        }
        for v in 0..node_quantity {
            for u in 0..node_quantity {
                if adj[v][u] == 1 {
                    let l = *vec![hamming_weight(v), hamming_weight(u), rev_hamming_weight(v, n), rev_hamming_weight(u, n)]
                        .iter()
                        .max()
                        .unwrap();
                    cap[v][u] = rng.gen_range(1..(2_usize.pow(l)));
                }
            }
        }
        Network { node_quantity, adj, cap }
    }
}

fn hamming_distance(x: usize, y: usize) -> u32 {
    let mut counter = 0;
    let mut z = x ^ y;
    while z != 0 {
        counter += z & 1;
        z >>= 1;
    }
    counter as u32
}

fn hamming_weight(x: usize) -> u32 {
    x.count_ones()
}

fn rev_hamming_weight(x: usize, position: u32) -> u32 {
    let mut num = x;
    let mut count = 0;
    for _ in 0..position {
        if num & 1 == 0 {
            count += 1;
        }
        
        num >>= 1;
        
        if num == 0 {
            break;
        }
    }
    count
}

pub fn esmond_karp(network: &Network, source: usize, sink: usize) {
    todo!()
}

pub fn dinic(network: &Network) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hamming_distance_test() {
        assert_eq!(hamming_distance(10, 6), 2)
    }

    #[test]
    fn hamming_weight_test() {
        assert_eq!(hamming_weight(10), 2)
    }

    #[test]
    fn rev_hamming_weight_test() {
        assert_eq!(rev_hamming_weight(10, 4), 2)
    }

    #[test]
    fn hyper_cube_test(){
        Network::new_hypercube(4);
    }
}
