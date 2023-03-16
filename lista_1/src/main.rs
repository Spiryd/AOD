use::std::io;

#[derive(Debug)]
struct Edge(u32, u32);

#[derive(Debug)]
enum Directionality {
    Directed,
    Undirected
}

#[derive(Debug)]
struct Graph{
    directionality: Directionality,
    node_quantity: u32,
    edges: Vec<Edge>
}

impl Graph {
    fn dsf(&self) {
        let mut visited: Vec<u32> = Vec::new();
        println!("{}", self.node_quantity);
        todo!()
    }
}

fn str_to_edge(string: String) -> Edge{
    let mut nodes: Vec<u32> = Vec::new();
    for node in string.split_whitespace(){
        nodes.push(node.trim().parse().expect("Should be a number!"));
    }
    Edge(nodes.get(0).unwrap().to_owned(), nodes.get(1).unwrap().to_owned())
}

fn main() {
    let mut directionality = String::new();
    println!("Drected or not(D or U): ");
    io::stdin()
        .read_line(&mut directionality)
        .expect("Failed to read line");
    let mut node_quantity = String::new();
    println!("number of nodes: ");
    io::stdin()
        .read_line(&mut node_quantity)
        .expect("Failed to read line");
    let node_quantity: u32 = node_quantity.trim().parse().expect("Should be a number!");
    let mut edge_quantity = String::new();
    println!("number of edges: ");
    io::stdin()
        .read_line(&mut edge_quantity)
        .expect("Failed to read line");
    let edge_quantity: u32 = edge_quantity.trim().parse().expect("Should be a number!");
    let mut edge: String = String::from("");
    let mut edges: Vec<Edge> = Vec::new();
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
        graph = Graph {
            directionality: Directionality::Directed,
            node_quantity: node_quantity,
            edges: edges
        };
    } else {
        graph = Graph {
            directionality: Directionality::Undirected,
            node_quantity: node_quantity,
            edges: edges
        };
    }
    println!("{:?}", graph);
    
}
