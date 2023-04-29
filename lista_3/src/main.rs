mod djikstra;
use djikstra::*;

fn main() {
    let mut graph = Graph::new(6, 20);
    graph.add_edge(0, 1, 13);
    graph.add_edge(0, 2, 0);
    graph.add_edge(0, 3, 15);
    graph.add_edge(0, 4, 20);
    graph.add_edge(1, 3, 5);
    graph.add_edge(2, 4, 9);
    graph.add_edge(3, 5, 2);
    graph.add_edge(4, 5, 4);
    println!("{:?}", graph);
    println!("{:?}", graph.djikstra_classic_ss(0));
    println!("{:?}", graph.djikstra_classic_p2p(0, 5));
    println!("{:?}", graph.dial_ss(0));
    println!("{:?}", graph.dial_p2p(0, 5));
    //println!("{:?}", graph.radix_ss(0));
    //println!("{:?}", graph.radix_p2p(0, 2));
}
