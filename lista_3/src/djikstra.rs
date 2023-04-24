use std::collections::{HashMap, BinaryHeap, HashSet};
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
    pub parent: Option<Box<SearchNode>>,
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
        if self.adj.contains_key(&from) {
            self.adj.get_mut(&from).unwrap().insert((to, cost));
        } else {
            self.adj.insert(from, HashSet::from([(to, cost)]));
        }   
    }
    pub fn djikstra_classic_p2p(&mut self, start: usize, goal: usize) -> Option<Vec<SearchNode>> {
        if start == goal {
            return Some(Vec::new()); 
        }
        let mut queue: BinaryHeap<SearchNode> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(start);
        let start_node = SearchNode{id: start, distance: 0, parent: None};
        for (id, distance) in self.adj.get(&start).unwrap() {
            queue.push(SearchNode{id: *id, distance: *distance, parent: Some(Box::new(start_node.clone()))});
        }
        while let Some(current_node) = queue.pop() {
            visited.insert(current_node.id);
            if current_node.id == goal{
                let mut path: Vec<SearchNode> = Vec::new();
                let mut current = &current_node;
                while let Some(parent) = &current.parent {
                    path.push(current.clone());
                    current = parent;
                }
                path.push(current.clone());
                path.reverse();
                return Some(path);
            }
            let d = current_node.distance.clone();
            for (id, dist) in self.adj.get(&current_node.id).unwrap() {
                if !visited.contains(id){
                    queue.push(SearchNode { id: *id, distance: (dist + d), parent: Some(Box::new(current_node.clone())) });
                }
            }
        }
        None
    }
    pub fn djikstra_classic_ss(&mut self, src: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.node_quantity];
        distances[src] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(SearchNode { id: src, distance: 0 , parent: None});
        while let Some(curr_node) = heap.pop() {
            let SearchNode { id, distance , parent: _parent} = curr_node.clone();
            if distance > distances[id] {
                continue;
            }
            for (neighbor, weight) in self.adj.get(&id).unwrap_or(&HashSet::new()) {
                let new_distance = distance + weight;
                if new_distance < distances[*neighbor] {
                    distances[*neighbor] = new_distance;
                    heap.push(SearchNode { id: *neighbor, distance: new_distance , parent: Some(Box::new(curr_node.clone()))});
                }
            }
        }
        distances
    }
    pub fn dial_p2p(&mut self, start: usize, goal: usize) -> Option<Vec<usize>> {
        todo!()
    }
    pub fn dial_ss(&mut self, src: usize,) -> Option<Vec<usize>> {
        todo!()
    }
    pub fn radix_p2p(&mut self, start: usize, goal: usize) -> Option<Vec<usize>> {
        todo!()
    }
    pub fn radix_ss(&mut self, src: usize,) -> Option<Vec<usize>> {
        todo!()
    }
}
