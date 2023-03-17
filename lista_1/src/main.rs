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
    }

    fn dfs_tool(node_id: usize, visited: &mut Vec<usize>, adj: &Vec<Vec<usize>>){
        visited.push(node_id);
        print!("{node_id} ");
        for edges in &adj[node_id - 1] {

        }
    }

    pub fn bfs(&self, _tree: bool){
        todo!()
    }
}

fn str_to_edge(string: String) -> (usize, usize){
    let mut nodes: Vec<usize> = Vec::new();
    for node in string.split_whitespace(){
        nodes.push(node.trim().parse().expect("Should be a number!"));
    }
    (nodes.get(0).unwrap().to_owned(), nodes.get(1).unwrap().to_owned())
}

fn main() {
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
    let graph: Graph;
    if directionality == "D" {
        graph = Graph::new(Directionality::Directed, node_quantity, edges);
    } else {
        graph = Graph::new(Directionality::Undirected, node_quantity, edges);
    }
    println!("{:?}", graph);
    graph.dfs(false);
    
}
