use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::cmp::Reverse;

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

#[derive(Clone, Debug)]
pub struct Graph{
    node_quantity: usize,
    max_weight: usize,
    adj: HashMap<usize, HashSet<(usize, usize)>>
}

impl Graph {
    pub fn new(node_quantity: usize, max_weight: usize) -> Self{
        let adj: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
        Graph {
            node_quantity,
            max_weight,
            adj 
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.adj.entry(from).or_insert(HashSet::new()).insert((to, cost));
    }
    pub fn djikstra_classic_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        if start == goal {
            return Some(0); 
        }
        let mut queue: BinaryHeap<SearchNode> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(start);
        for (id, distance) in self.adj.get(&start).unwrap() {
            queue.push(SearchNode{id: *id, distance: *distance});
        }
        while let Some(current_node) = queue.pop() {
            visited.insert(current_node.id);
            if current_node.id == goal{
                return Some(current_node.distance);
            }
            let d = current_node.distance;
            for (id, dist) in self.adj.get(&current_node.id).unwrap() {
                if !visited.contains(id){
                    queue.push(SearchNode { id: *id, distance: (dist + d)});
                }
            }
        }
        None
    }
    pub fn djikstra_classic_ss(&mut self, src: usize) -> Vec<Option<usize>>  {
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
            for (neighbor, weight) in self.adj.get(&node).unwrap_or(&HashSet::new()) {
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
        let mut distances = vec![None; self.node_quantity];
        let max_buckets = self.node_quantity * self.max_weight;
        let mut buckets: Vec<HashSet<usize>> = vec![HashSet::new(); max_buckets];
        distances[start] = Some(0);
        buckets[0].insert(start);
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), start));
        while let Some((Reverse(dist), node)) = heap.pop() {
            if node == goal {
                return Some(dist);
            }
            if let Some(d) = distances.get(node).and_then(|d| *d) {
                if dist > d {
                    continue;
                }
            }
            buckets[dist].remove(&node);
            distances[node] = Some(dist);
            for (neighbor, weight) in self.adj.get(&node).unwrap() {
                let new_dist = dist + *weight;
                if let Some(d) = distances.get(*neighbor).and_then(|d| *d) {
                    if new_dist >= d {
                        continue;
                    }
                }
                buckets[new_dist].insert(*neighbor);
                heap.push((Reverse(new_dist), *neighbor));
            }
        }
        None
    }
    pub fn dial_ss(&mut self, src: usize,)  -> Vec<Option<usize>> {
        let mut distances = vec![None; self.node_quantity];
        let max_buckets = self.node_quantity * self.max_weight;
        let mut buckets: Vec<HashSet<usize>> = vec![HashSet::new(); max_buckets];
        distances[src] = Some(0);
        buckets[0].insert(src);
        let mut bucket_idx: usize = 0;
        loop {
            while bucket_idx < max_buckets && buckets[bucket_idx].is_empty() {
                bucket_idx += 1;
            }
            if bucket_idx >= max_buckets {
               break; 
            }
            let v = *buckets[bucket_idx].iter().next().unwrap();
            buckets[bucket_idx].remove(&v);
            for (u, w) in self.adj.get(&v).unwrap_or(&HashSet::new()){
                let alt_dist = distances[v].unwrap_or(usize::MAX) + w;
                let curr_dist = distances[*u].unwrap_or(usize::MAX);
                if alt_dist < curr_dist {
                    if curr_dist != usize::MAX {
                        buckets[alt_dist].remove(u);
                    }
                    buckets[alt_dist].insert(*u);
                    distances[*u] = Some(alt_dist);
                }
            }
        }        
        distances
    }
    pub fn radix_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        todo!()
    }
    pub fn radix_ss(&mut self, src: usize,) -> Vec<Option<usize>> {
        todo!()
    }
}
