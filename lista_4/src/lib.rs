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

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    to: usize,
    flow: isize,
    residual_capacity: isize
}

#[derive(Debug, Clone)]
pub struct Hypercube {
    pub node_quantity: usize,
    adj: Vec<Vec<Edge>>
}

impl Hypercube {
    pub fn new(n: u32) -> Self {
        let mut rng: Pcg64 = Pcg64::from_entropy();
        let node_quantity= 2_usize.pow(n);
        let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); node_quantity];
        for v in 0..node_quantity {
            for u in 0..node_quantity {
                if v < u && hamming_distance(v, u) == 1 {
                    let l = *vec![hamming_weight(v), hamming_weight(u), rev_hamming_weight(v, n), rev_hamming_weight(u, n)]
                    .iter()
                    .max()
                    .unwrap();
                    let cap = rng.gen_range(1..(2_isize.pow(l)));
                    adj[v].push(Edge { to: u, flow: 0, residual_capacity: cap });
                    adj[u].push(Edge { to: v, flow: 0, residual_capacity: 0 });
                }
            }
        }
        Hypercube { node_quantity, adj }
    }

    pub fn to_jump(&self) {
        let mut printable = String::new();
        let mut matrix = vec![vec![0; self.node_quantity]; self.node_quantity];
        for (v, edges) in self.adj.iter().enumerate() {
            for edge in edges {
                matrix[v][edge.to] = edge.residual_capacity;
            }
        }
        printable += "G = [\n";
        for v in matrix {
            printable += "  ";
            for u in v {
                printable += &u.to_string();
                printable += " ";
            }
            printable += "\n";
        }
        printable += "]\n";
        printable += "n = size(G)[1]\n
max_flow = Model(HiGHS.Optimizer)\n
@variable(max_flow, f[1:n, 1:n] >= 0)\n
# Capacity constraints\n
@constraint(max_flow, [i = 1:n, j = 1:n], f[i, j] <= G[i, j])\n";
        printable += &format!("@constraint(max_flow, [i = 1:n; i != 1 && i != {}], sum(f[i, :]) == sum(f[:, i]))\n", self.node_quantity);
        printable += "# Flow conservation constraints\n
@objective(max_flow, Max, sum(f[1, :]))\n
optimize!(max_flow)\n
objective_value(max_flow)\n";
        println!("{}", printable);
    }

    fn bfs(&self, source: usize, target: usize, parent: &mut Vec<Option<usize>>) -> bool {
        let mut visited = vec![false; self.node_quantity];
        let mut queue = Vec::new();
        queue.push(source);
        visited[source] = true;
        parent[source] = None;

        while !queue.is_empty() {
            let u = queue.remove(0);

            for edge in &self.adj[u]{
                let v = edge.to;
                if !visited[v] && edge.residual_capacity > 0 {
                    queue.push(v);
                    parent[v] = Some(u);
                    visited[v] = true;
                }
            }
        }

        visited[target]
    }

    pub fn edmonds_karp(&mut self, source: usize, target: usize) -> (isize, usize) {
        let mut parent = vec![None; self.node_quantity];
        let mut max_flow = 0;
        let mut augmenting_paths = 0;

        while self.bfs(source, target, &mut parent) {
            augmenting_paths += 1;
            let mut path_flow = std::isize::MAX;

            let mut v = target;
            while let Some(u) = parent[v] {
                let edge = self.adj[u].iter_mut().find(|e| e.to == v).unwrap();
                path_flow = path_flow.min(edge.residual_capacity);
                v = u;
            }

            v = target;
            while let Some(u) = parent[v] {
                let edge = self.adj[u].iter_mut().find(|e| e.to == v).unwrap();
                edge.flow += path_flow;
                edge.residual_capacity -= path_flow;

                let rev_edge = self.adj[v].iter_mut().find(|e| e.to == u).unwrap();
                rev_edge.flow -= path_flow;
                rev_edge.residual_capacity += path_flow;

                v = u;
            }

            max_flow += path_flow;
        }

        (max_flow, augmenting_paths)
    }


    fn d_bfs(&self, source: usize, sink: usize, level: &mut Vec<usize>) -> bool {
        for i in 0..self.node_quantity {
            level[i] = std::usize::MAX;
        }
        level[source] = 0;
    
        let mut queue = VecDeque::new();
        queue.push_back(source);
        let mut visited_sink = false;
    
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
    
            for edge in &self.adj[u] {
                if edge.residual_capacity > 0 && level[edge.to] == std::usize::MAX {
                    level[edge.to] = level[u] + 1;
                    queue.push_back(edge.to);
                    if edge.to == sink {
                        visited_sink = true;
                    }
                }
            }
        }
    
        visited_sink
    }

    // Uses DFS to find blocking flow.
    fn dfs(&mut self, u: usize, min_edge: isize, sink: usize, level: &Vec<usize>, start: &mut Vec<usize>) -> isize {
        if u == sink || min_edge == 0 {
            return min_edge;
        }

        while start[u] < self.adj[u].len() {
            let edge_index = start[u];
            let edge = self.adj[u][edge_index].clone();  // Clone edge to avoid borrow checker issues

            if level[edge.to] == level[u] + 1 {
                let flow = self.dfs(edge.to, min_edge.min(edge.residual_capacity), sink, level, start);

                if flow > 0 {
                    let actual_edge = &mut self.adj[u][edge_index];
                    actual_edge.flow += flow;
                    actual_edge.residual_capacity -= flow;

                    let reverse_edge = self.adj[edge.to].iter_mut().find(|e| e.to == u).unwrap();
                    reverse_edge.flow -= flow;
                    reverse_edge.residual_capacity += flow;

                    return flow;
                }
            }

            start[u] += 1;
        }

        0
    }

    pub fn dinic(&mut self, source: usize, sink: usize) -> (isize, usize) {
        let mut max_flow = 0;
        let mut level = vec![0; self.node_quantity];
        let mut start = vec![0; self.node_quantity];
        let mut num_augmenting_paths = 0;

        while self.d_bfs(source, sink, &mut level) {
            for i in 0..self.node_quantity {
                start[i] = 0;
            }

            let mut flow = self.dfs(source, std::isize::MAX, sink, &level, &mut start);

            while flow != 0 {
                num_augmenting_paths += 1;
                max_flow += flow;
                flow = self.dfs(source, std::isize::MAX, sink, &level, &mut start);
            }
        }

        (max_flow, num_augmenting_paths)
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
        let mut n = Hypercube::new(4);
        n.edmonds_karp(0, 15);
    }

    #[test]
    fn dinic_test(){
        let mut n = Hypercube::new(3);
        n.dinic(0, 7);
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
