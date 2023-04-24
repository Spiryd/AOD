
mod djikstra;
use djikstra::*;

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1, 3);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 0, 1);
    println!("{:?}", graph);
    println!("{:?}", graph.djikstra_classic_ss(0));
}
