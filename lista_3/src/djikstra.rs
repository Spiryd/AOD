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
    pub fn djikstra_classic(&mut self, start: usize, goal: usize) -> Option<Vec<usize>> {
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
        println!("{:?}", queue);
        println!("{:?}", visited);
        while let Some(current_node) = queue.pop() {
            if current_node.id == goal{
                let mut path: Vec<usize> = Vec::new();
                let mut current = &current_node;
                while let Some(parent) = &current.parent {
                    path.push(current.id);
                    current = parent;
                }
                path.push(current.id);
                path.reverse();
                return Some(path);
            }

            visited.insert(current_node.id);
            let d = current_node.distance.clone();
            for (id, dist) in self.adj.get(&current_node.id).unwrap() {
                
                queue.push(SearchNode { id: *id, distance: (dist + d), parent: Some(Box::new(current_node.clone())) });
            }
        }
        None
    }
}
