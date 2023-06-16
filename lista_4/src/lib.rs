use std::collections::VecDeque;
use rand::prelude::*;
use rand_pcg::Pcg64;

const NIL: usize = 0;
const INF: usize = usize::MAX;

#[derive(Debug, Clone)]
pub struct Bigraph {
    adj: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}

impl Bigraph {
    pub fn new(k: u32, i: usize) -> Self {
        let mut rng: Pcg64 = Pcg64::from_entropy();
        let [n, m] = [2_usize.pow(k); 2];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let u_nodes: Vec<usize> = (1..=n).collect();
        for u in 1..=n {
            for v in u_nodes.choose_multiple(&mut rng, i) {
                adj[u].push(*v);
            }
        }
        Bigraph { adj, n, m }
    }
    pub fn hopcroft_karp(&self) -> usize {
        let mut pair_u: Vec<usize> = vec![NIL; self.n + 1];
        let mut pair_v: Vec<usize> = vec![NIL; self.m + 1];
        let mut dist: Vec<usize> = vec![usize::MAX; self.n + 1];
        let mut matching: usize = 0;
        
        while self.bfs(&mut pair_u, &mut pair_v, &mut dist) {
            for u in 1..=self.n {
                if pair_u[u] == NIL && self.dfs(u, &mut pair_u, &mut pair_v, &mut dist) {
                    matching += 1;
                }
            }
        }

        matching
    }
    fn bfs(&self, pair_u: &mut Vec<usize>,  pair_v: &mut Vec<usize>, dist: &mut Vec<usize>) -> bool{
        let mut queue: VecDeque<usize> = VecDeque::new();

        for u in 1..=self.n {
            if pair_u[u] == NIL {
                dist[u] = 0;
                queue.push_back(u);
            } else {
                dist[u] = INF;
            }
        }
        dist[NIL] = INF;
        while let Some(u) = queue.pop_front() {
            if dist[u] < dist[NIL] {
                for v in &self.adj[u] {
                    if dist[pair_v[*v]] == INF {
                        dist[pair_v[*v]] = dist[u] + 1;
                        queue.push_back(pair_v[*v]);
                    }
                }
            }
        }
        dist[NIL] != INF
    }
    fn dfs(&self, u: usize, pair_u: &mut Vec<usize>, pair_v: &mut Vec<usize>, dist: &mut Vec<usize>) -> bool {
        if u != NIL {
            for v in &self.adj[u] {
                if dist[pair_v[*v]] == dist[u] + 1 {
                    if self.dfs(pair_v[*v], pair_u, pair_v, dist) {
                        pair_v[*v] = u;
                        pair_u[u] = *v;
                        return true;
                    }
                }
            }
            dist[u] = INF;
            return false;
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct Hypercube {
    pub node_quantity: usize,
    //adj: Vec<Vec<u8>>,
    cap: Vec<Vec<isize>>
}

impl Hypercube {
    pub fn new(n: u32) -> Self {
        let mut rng: Pcg64 = Pcg64::from_entropy();
        let node_quantity= 2_usize.pow(n);
        let mut adj: Vec<Vec<u8>> = vec![vec![0; node_quantity]; node_quantity];
        let mut cap: Vec<Vec<isize>> = vec![vec![0; node_quantity]; node_quantity];
        for v in 0..node_quantity {
            for u in 0..node_quantity {
                if v < u && hamming_distance(v, u) == 1 {
                    adj[v][u] = 1;
                    let l = *vec![hamming_weight(v), hamming_weight(u), rev_hamming_weight(v, n), rev_hamming_weight(u, n)]
                    .iter()
                    .max()
                    .unwrap();
                    cap[v][u] = rng.gen_range(1..(2_isize.pow(l)));
                }
            }
        }
        Hypercube { node_quantity, cap }
    }

    pub fn edmonds_karp(&self, source: usize, sink: usize) -> isize {
        let node_quantity = self.node_quantity;
        let mut resid: Vec<Vec<isize>> = self.cap.clone();
        let mut parent: Vec<isize> = vec![0; node_quantity];
        let mut maxflow: isize = 0;
    
        while bfs_for_edmonds_karp(source, sink, &mut parent, &mut resid) {
            let mut path_flow = isize::MAX;
            let mut v = sink;
            while v != source {
                let u = parent[v];
                path_flow = path_flow.min(resid[u as usize][v as usize]);
                v = u as usize;
            }
    
            v = sink;
    
            while v != source {
                let u = parent[v];
                resid[u as usize][v] -= path_flow;
                resid[v][u as usize] += path_flow;
                v = u as usize;
            }
    
            maxflow += path_flow;
        }
    
        maxflow
    }

    pub fn dinic(&self, source: usize, sink: usize) -> isize {
        todo!()
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

fn bfs_for_edmonds_karp(source: usize, sink: usize, parent: &mut Vec<isize>, resid: &mut Vec<Vec<isize>>) -> bool{
    let mut visited = vec![false; resid.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(source);
    visited[source] = true;
    parent[source] = -1;

    while let Some(u) = queue.pop_front() {
        for v  in 0..resid.len() {
            if !visited[v] && resid[u][v] > 0 {
                queue.push_back(v);
                parent[v] = u as isize;
                visited[v] = true;
            }
        }
    }
    visited[sink]
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
        Hypercube::new(4);
    }

    #[test]
    fn edmonds_karp_test(){
        let n = Hypercube::new(4);
        n.edmonds_karp(0, 15);
    }

    #[test]
    fn bigraph_test() {
        Bigraph::new(6, 4);
    }

    #[test]
    fn hopcroft_karp_test() {
        let bigraph = Bigraph::new(6, 4);
        bigraph.hopcroft_karp();
    }
}
