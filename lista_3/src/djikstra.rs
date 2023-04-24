use std::collections::{HashMap, BinaryHeap, HashSet, VecDeque};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Graph{
    node_quantity: usize,
    adj: HashMap<usize, HashSet<(usize, usize)>>
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

impl Graph {
    pub fn new() -> Self{
        let node_quantity = 0;
        let adj: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
        Self{ node_quantity, adj }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.node_quantity += 1;
        self.adj.entry(from).or_insert(HashSet::new()).insert((to, cost));
    }
    pub fn djikstra_classic_p2p(&mut self, start: usize, goal: usize) -> Option<usize> {
        if start == goal {
            return Some(0); 
        }
        let mut queue: BinaryHeap<SearchNode> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(start);
        let start_node = SearchNode{id: start, distance: 0};
        for (id, distance) in self.adj.get(&start).unwrap() {
            queue.push(SearchNode{id: *id, distance: *distance,});
        }
        while let Some(current_node) = queue.pop() {
            visited.insert(current_node.id);
            if current_node.id == goal{
                return Some(current_node.distance);
            }
            let d = current_node.distance.clone();
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
        let mut buckets = Vec::new();
        let mut min_bucket = 0;
        for i in 0..(2 * self.node_quantity) {
            buckets.push(HashSet::new());
        }
        distances[start] = Some(0);
        buckets[0].insert(start);
        while min_bucket < buckets.len() {
            let current_bucket = &mut buckets[min_bucket];
            if current_bucket.is_empty() {
                min_bucket += 1;
                continue;
            }
            let node = *current_bucket.iter().next().unwrap();
            current_bucket.remove(&node);
            if node == goal {
                return distances[goal];
            }
            for (neighbor, weight) in self.adj.get(&node).unwrap_or(&HashSet::new()).iter() {
                let bucket_index = distances[node].unwrap_or(0) + weight;
                if distances[*neighbor].map_or(true, |d| bucket_index < d) {
                    let old_bucket_index = distances[*neighbor].unwrap_or(usize::MAX);
                    if old_bucket_index < buckets.len() {
                        buckets[old_bucket_index].remove(neighbor);
                    }
                    buckets[bucket_index].insert(*neighbor);
                    distances[*neighbor] = Some(bucket_index);
                }
            }
        }
        None
    }
    pub fn dial_ss(&mut self, src: usize,)  -> Vec<Option<usize>> {
        let mut distances = vec![None; self.node_quantity];
        distances[src] = Some(0);
        let mut buckets = vec![HashSet::new(); self.node_quantity];
        buckets[0].insert(src);
        let mut max_bucket_index = 0;

        while let Some(&node) = buckets[max_bucket_index].iter().next() {
            buckets[max_bucket_index].remove(&node);
            for (neighbor, weight) in self.adj.get(&node).unwrap_or(&HashSet::new()) {
                let new_distance = distances[node].unwrap() + weight;
                if distances[*neighbor].map_or(true, |d| new_distance < d) {
                    distances[*neighbor] = Some(new_distance);
                    let new_bucket_index = new_distance as usize;
                    if new_bucket_index >= buckets.len() {
                        buckets.resize(new_bucket_index + 1, HashSet::new());
                    }
                    if new_bucket_index > max_bucket_index {
                        max_bucket_index = new_bucket_index;
                    }
                    buckets[new_bucket_index].insert(*neighbor);
                }
            }
            if buckets[max_bucket_index].is_empty() {
                max_bucket_index -= 1;
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
