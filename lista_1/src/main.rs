use std::collections::VecDeque;
use::std::io;

#[derive(Debug)]
pub enum Directionality {
    Directed,
    Undirected
}

#[derive(Debug)]
pub struct Graph{
    _directionality: Directionality,
    node_quantity: usize,
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
        Self { _directionality: directionality, node_quantity: node_quantity, adj: adj }
    }

    pub fn dfs(&self, _tree: bool) {
        let mut visited: Vec<usize> = Vec::new();
        for node_id in 1..=self.node_quantity{
            if !visited.contains(&node_id) {
                Self::dfs_tool(node_id, &mut visited, &self.adj);
            }
        }
        print!("\n");
    }

    fn dfs_tool(node_id: usize, visited: &mut Vec<usize>, adj: &Vec<Vec<usize>>){
        visited.push(node_id);
        print!("{node_id} ");
        for edge in &adj[node_id - 1] {
            if !visited.contains(edge){
                Self::dfs_tool(edge.clone(), visited, adj);
            }
        }
    }

    pub fn bfs(&self, _tree: bool){
        let mut visited: Vec<usize> = Vec::new();
        let mut traversal: Vec<usize> = Vec::new();
        if _tree == false{
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
                                queue.push_back(j.clone())
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
                                queue.push_back(j.clone())
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

}

fn str_to_edge(string: String) -> (usize, usize){
    let mut nodes: Vec<usize> = Vec::new();
    for node in string.split_whitespace(){
        nodes.push(node.trim().parse().expect("Should be a number!"));
    }
    (nodes.get(0).unwrap().to_owned(), nodes.get(1).unwrap().to_owned())
}

fn gen_graph_from_console() ->  Graph{
    let mut directionality = String::new();
    println!("Directed or not(D or U): ");
    io::stdin()
        .read_line(&mut directionality)
        .expect("Failed to read line");
    let mut node_quantity = String::new();
    println!("number of nodes: ");
    io::stdin()
        .read_line(&mut node_quantity)
        .expect("Failed to read line");
    let node_quantity: usize = node_quantity.trim().parse().expect("Should be a number!");
    let mut edge_quantity = String::new();
    println!("number of edges: ");
    io::stdin()
        .read_line(&mut edge_quantity)
        .expect("Failed to read line");
    let edge_quantity: usize = edge_quantity.trim().parse().expect("Should be a number!");
    let mut edge: String = String::from("");
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for _ in 0..edge_quantity {
        println!("edge: ");
        io::stdin()
            .read_line(&mut edge)
            .expect("Failed to read line");
        edges.push(str_to_edge(edge.clone()));
        edge = String::from("");
    } 

    if directionality.contains('D'){
        Graph::new(Directionality::Directed, node_quantity, edges)
    } else {
        Graph::new(Directionality::Undirected, node_quantity, edges)
    }
}

fn main() {
    let graph = Graph::new(Directionality::Directed, 6, vec![(1, 3), (1, 2), (3, 6), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6)]);
    println!("{:?}", graph);
    graph.dfs(false);
    graph.bfs(false);
}
