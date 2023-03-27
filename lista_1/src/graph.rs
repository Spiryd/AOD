use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub enum Directionality {
    Directed,
    Undirected
}

#[derive(Debug)]
pub struct Graph{
    pub directionality: Directionality,
    pub node_quantity: usize,
    adj: Vec<Vec<usize>>
}

impl Graph {
    pub fn new(directionality: Directionality, node_quantity: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut adj: Vec<Vec<usize>> = Vec::new();
        for _ in 1..=node_quantity {
            adj.push(Vec::new());
        }
        match directionality {
            Directionality::Directed => {
                for edge in edges {
                    adj[edge.0 - 1].push(edge.1);
                }
            },
            Directionality::Undirected => {
                for edge in edges {
                    adj[edge.0 - 1].push(edge.1);
                    adj[edge.1 - 1].push(edge.0);
                }
            }
        }
        Self { directionality: directionality, node_quantity: node_quantity, adj: adj }
    }

    pub fn dfs(&self, tree: bool){
        let mut visited: Vec<usize> = Vec::new();
        let mut traversal: Vec<usize> = Vec::new();
        if tree == false {
            for i in 1..=self.node_quantity {
                if !visited.contains(&i) {
                    let mut stack: Vec<usize> = Vec::new();
                    visited.push(i);
                    stack.push(i);
                    while !stack.is_empty() {
                        let node = stack.pop().unwrap();
                        traversal.push(node);
                        for j in &self.adj[node-1] {
                            if !visited.contains(j) {
                                visited.push(j.clone());
                                stack.push(j.clone());
                            }
                        }
                    }
                }
            }
        } else {
            let mut tree_edges: Vec<(usize, usize)> = Vec::new();
            for i in 1..=self.node_quantity {
                if !visited.contains(&i) {
                    let mut stack: Vec<usize> = Vec::new();
                    visited.push(i);
                    stack.push(i);
                    while !stack.is_empty() {
                        let node = stack.pop().unwrap();
                        traversal.push(node);
                        for j in &self.adj[node-1] {
                            if !visited.contains(j) {
                                tree_edges.push((node, j.clone()));
                                visited.push(j.clone());
                                stack.push(j.clone());
                            }
                        }
                    }
                }
            }
            println!("{:?}", tree_edges)
        }
        for node in traversal {
            print!("{node} ");
        }
        print!("\n");
    }

    pub fn bfs(&self, tree: bool){
        let mut visited: Vec<usize> = Vec::new();
        let mut traversal: Vec<usize> = Vec::new();
        if tree == false{
            for i in 1..=self.node_quantity {
                if !visited.contains(&i) {
                    let mut queue: VecDeque<usize> = VecDeque::new();
                    visited.push(i);
                    queue.push_back(i);
                    while !queue.is_empty() {
                        let node = queue.pop_front().unwrap();
                        traversal.push(node);
                        for j in &self.adj[node-1] {
                            if !visited.contains(j) {
                                visited.push(j.clone());
                                queue.push_back(j.clone());
                            }
                        }
                    }
                }
            }
        } else {
            let mut tree_edges: Vec<(usize, usize)> = Vec::new();
            for i in 1..=self.node_quantity {
                if !visited.contains(&i) {
                    let mut queue: VecDeque<usize> = VecDeque::new();
                    visited.push(i);
                    queue.push_back(i);
                    while !queue.is_empty() {
                        let node = queue.pop_front().unwrap();
                        traversal.push(node);
                        for j in &self.adj[node-1] {
                            if !visited.contains(j) {
                                tree_edges.push((node, j.clone()));
                                visited.push(j.clone());
                                queue.push_back(j.clone());
                            }
                        }
                    }
                }
            }
            println!("{:?}", tree_edges)
        }
        for node in traversal {
            print!("{node} ");
        }
        print!("\n");
    }


    //Kahn's Algorithm Topological Sort
    pub fn topological_sort(&self) -> Result<Vec<usize>, String> {
        let n = self.node_quantity;
        let mut in_degrees: Vec<usize> = vec![0; n];
        for from in &self.adj {
            for to in from{
                in_degrees[to - 1] += 1;
            }
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        for i in 0..n {
            if in_degrees[i] == 0 {
                queue.push_back(i + 1)
            }
        }
        let mut order: Vec<usize> = Vec::new();
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            order.push(node);
            for to in &self.adj[node - 1] {
                in_degrees[*to - 1] -= 1;
                if in_degrees[*to - 1] == 0{
                    queue.push_back(*to);
                }
            }
        }
        if order.len() != n{
            Err("Not a DAG".to_string())
        }else {
            Ok(order)
        }
    }


    pub fn find_sccs(&self) -> Vec<Vec<usize>> {
        todo!();
    }

    pub fn is_bipartite(&self) -> bool{
        // -1 = uncolored, 0 = red, 1 = blue
        let mut color: Vec<i8> = vec![-1; self.node_quantity];
        //bfs queue (node, color)
        let mut queue: VecDeque<(usize, i8)> = VecDeque::new();
        //bfs with coloring loop to get disconnected parts
        for node in 1..=self.node_quantity {
            if color[node - 1] == -1 {
                queue.push_back((node, 0));
                color[node - 1] = 0;
                while !queue.is_empty() {
                    //(node, color)
                    let pair = queue.pop_front().unwrap();
                    for connected_node in self.adj[pair.0 - 1].clone() {
                        // two colors cannot be touching if it is a bigraph
                        if color[connected_node - 1] == pair.1 {
                            return false;
                        }else if color[connected_node - 1] == -1 {
                            if pair.1 == 0 {
                                color[connected_node - 1] = 1;
                            }else {
                                color[connected_node - 1] = 0;
                            }
                            queue.push_back((connected_node, color[connected_node - 1]))
                        }
                    }
                    
                }
            }
        }
        return true;
    }

}