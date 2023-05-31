use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::usize;

#[derive(Clone, Debug)]
struct RadixBucket {
    v_list: VecDeque<usize>,
    range_a: usize,
    range_b: usize,
}

impl RadixBucket {
    fn generate_sequence(n: u32) -> Vec<(usize, usize)> {
        let mut sequence = Vec::new();
        let mut prev_value = 0;
    
        for i in 0..n {
            let current_value = 2usize.pow(i);
            let current_tuple = (prev_value, current_value - 1);
            sequence.push(current_tuple);
            prev_value = current_value;
        }
        sequence
    }

}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct SearchNode{
    pub id: usize,
    pub distance: usize,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}   

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Graph{
    node_quantity: usize,
    max_weight: usize,
    adj: Vec<HashSet<(usize, usize)>>
}

impl Graph {
    pub fn new(node_quantity: usize) -> Self{
        let adj: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); node_quantity];
        Graph {
            node_quantity,
            max_weight: 0,
            adj 
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        if cost > self.max_weight{
            self.max_weight = cost;
        }
        self.adj[from - 1].insert((to - 1, cost));
    }
    pub fn djikstra_classic_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        let start = start - 1;
        let goal = goal - 1;

        if start == goal {
            return Some(0); 
        }
        let mut queue: BinaryHeap<SearchNode> = BinaryHeap::new();
        let mut visited = vec![false; self.node_quantity];
        visited[start] = true;
        for (id, distance) in &self.adj[start] {
            queue.push(SearchNode{id: *id,  distance: *distance});
        }
        while let Some(current_node) = queue.pop() {
            visited[current_node.id] = true;
            if current_node.id == goal{
                return Some(current_node.distance);
            }
            let d = current_node.distance;
            for (id, dist) in &self.adj[current_node.id] {
                if !visited[*id]{
                    queue.push(SearchNode { id: *id, distance: (dist + d)});
                }
            }
        }
        None
    }
    pub fn djikstra_classic_ss(&mut self, src: usize) -> Vec<Option<usize>>  {
        let src = src - 1;
        let mut distances = vec![None; self.node_quantity];
        distances[src] = Some(0);
        let mut heap = BinaryHeap::new();
        heap.push((0, src));
        while let Some((distance, node)) = heap.pop() {
            if let Some(current_distance) = distances[node] {
                if distance > current_distance {
                    continue;
                }
            }
            for (neighbor, weight) in &self.adj[node] {
                let new_distance = distance + weight;
                if distances[*neighbor].map_or(true, |d| new_distance < d) {
                    distances[*neighbor] = Some(new_distance);
                    heap.push((new_distance, *neighbor));
                }
            }
        }
        distances
    }
    pub fn dial_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        if start == goal{
            return Some(0);
        }
        let goal = goal - 1;
        self.dial_ss(start)[goal]
    }
    pub fn dial_ss(&mut self, src: usize)  -> Vec<Option<usize>> {
        let src = src - 1;

        let mut distances = vec![None; self.node_quantity];
        let max_buckets = self.node_quantity * self.max_weight;
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); max_buckets];
        distances[src] = Some(0);
        buckets[0].push(src);
        let mut bucket_idx: usize = 0;
        loop {
            while bucket_idx < max_buckets && buckets[bucket_idx].is_empty() {
                bucket_idx += 1;
            }
            if bucket_idx >= max_buckets {
               break; 
            }
            let v = *buckets[bucket_idx].first().unwrap();
            buckets[bucket_idx].retain(|x| *x != v);
            for (u, w) in &self.adj[v] {
                let alt_dist = distances[v].unwrap_or(usize::MAX) + w;
                let curr_dist = distances[*u].unwrap_or(usize::MAX);
                if alt_dist < curr_dist {
                    if curr_dist != usize::MAX {
                        buckets[alt_dist].retain(|x| *x != *u);
                    }
                    buckets[alt_dist].push(*u);
                    distances[*u] = Some(alt_dist);
                }
            }
        }        
        distances
    }
    pub fn radix_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        if start == goal{
            return Some(0);
        }
        let goal = goal - 1;
        self.radix_ss(start)[goal]
    }
    pub fn radix_ss(&self, src: usize) -> Vec<Option<usize>> {
        let src = src - 1;
        let mut dist: Vec<Option<usize>> = vec![None; self.node_quantity];
        dist[src] = Some(0);

        let no_buckets = (((self.max_weight * self.node_quantity) as f64).log2().ceil() + 1_f64) as usize;
        let mut buckets: Vec<RadixBucket> = Vec::with_capacity(no_buckets as usize);
        let bucket_seq = RadixBucket::generate_sequence(no_buckets as u32);
        //println!("{:?}", bucket_seq);
        for rang in bucket_seq {
            buckets.push(RadixBucket { v_list: VecDeque::new(), range_a: rang.0, range_b: rang.1 })
        }
        //println!("{:?}", buckets);
        buckets[0].v_list.push_front(src);
        
        let mut idx;
        loop {
            //choosing the node
            idx = 0;
            while idx < buckets.len() && buckets[idx].v_list.len() == 0 {
                idx += 1;
            }
            if idx == buckets.len(){
                break;
            }

            let mut u = *buckets[idx].v_list.front().unwrap();
            //ckecking if is alone in bucket
            if buckets[idx].v_list.len() == 1 {
                buckets[idx].v_list.clear();
            } else {
                let mut minv = 0;
                let mut mindist = usize::MAX;

                for v in &buckets[idx].v_list {
                    if  dist[*v].unwrap_or(usize::MAX) < mindist {
                        mindist = dist[*v].unwrap_or(usize::MAX);
                        minv = *v;
                    }
                }
                u = minv;
                //resize
                let r = RadixBucket::generate_sequence(idx as u32);
                for i in 0..idx {
                    buckets[i].range_a = r[i].0 + mindist;
                    buckets[i].range_b = r[i].1 + mindist;
                }
                if idx != 0 {
                    buckets[idx -1].range_b = buckets[idx].range_b;
                }
                //repopulate
                for v in buckets[idx].v_list.clone() {
                    for i in 0..idx {
                        if dist[v].unwrap() >= buckets[i].range_a && dist[v].unwrap() <= buckets[i].range_b {
                            buckets[i].v_list.push_front(v);
                            break;
                        }
                    }
                }

                buckets[idx].range_a = usize::MAX;
                buckets[idx].range_b = usize::MAX;
                buckets[idx].v_list.clear();
            }

            for (v, weight) in &self.adj[u] {
                let dv = dist[*v].unwrap_or(usize::MAX);
                let du = dist[u].unwrap();
                if dv > du + weight {
                    if dv != usize::MAX {
                        let mut tmp = 0;
                        while !(buckets[tmp].range_a <= dv && buckets[tmp].range_b >= dv) {
                            tmp += 1;
                        }
                        for (i, j) in buckets[tmp].v_list.iter().enumerate() {
                            if j == v {
                                buckets[tmp].v_list.remove(i);
                                break;
                            }
                        }
                    }
                    let mut b = 0;
                    //println!("{}", du + weight);
                    while !(buckets[b].range_a <= du + weight && buckets[b].range_b >= du + weight) {
                        //println!("{}, {}", buckets[b].range_a, buckets[b].range_b);
                        b += 1;
                    }
                    dist[*v] = Some(du + weight);
                    buckets[b].v_list.push_front(*v);
                }
            }
            //println!("{:?}", dist); 
        }
        dist
    }
}


#[test]
fn graph_test() {
    let mut graph = Graph::new(6);
    graph.add_edge(1, 2, 13);
    graph.add_edge(1, 3, 0);
    graph.add_edge(1, 4, 15);
    graph.add_edge(1, 5, 20);
    graph.add_edge(2, 4, 5);
    graph.add_edge(3, 5, 9);
    graph.add_edge(4, 6, 2);
    graph.add_edge(5, 6, 4);
    println!("{:?}", graph);
    println!("{:?}", graph.djikstra_classic_ss(1));
    println!("{:?}", graph.djikstra_classic_p2p(1, 5));
    println!("{:?}", graph.dial_ss(1));
    println!("{:?}", graph.dial_p2p(1, 5));
    println!("{:?}", graph.radix_ss(1));
    println!("{:?}", graph.radix_p2p(1, 2));
}
