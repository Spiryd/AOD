use std::{io, fs};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

mod graph;
use graph::*;

fn str_to_edge(string: &String) -> (usize, usize){
    let mut nodes: Vec<usize> = Vec::new();
    for node in string.split_whitespace(){
        nodes.push(node.trim().parse().expect("Should be a number!"));
    }
    (nodes.get(0).unwrap().to_owned(), nodes.get(1).unwrap().to_owned())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _gen_graph_from_console() ->  Graph{
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
        edges.push(str_to_edge(&edge));
        edge = String::from("");
    } 

    if directionality.contains('D'){
        Graph::new(Directionality::Directed, node_quantity, edges)
    } else {
        Graph::new(Directionality::Undirected, node_quantity, edges)
    }
}

fn test1(){
    let mut graphs: Vec<Graph> = Vec::new();
    graphs.push(Graph::new(Directionality::Directed, 6, vec![(1, 3), (1, 2), (3, 5), (3, 6), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6)]));
    graphs.push(Graph::new(Directionality::Undirected, 6, vec![(1, 3), (1, 2), (3, 5), (3, 6), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6)]));
    graphs.push(Graph::new(Directionality::Directed, 8, vec![(1, 2), (1, 4), (2, 3), (2, 6), (3, 4), (4, 8), (5, 1), (5, 8), (6, 5), (6, 7), (7, 3), (8, 7)]));
    graphs.push(Graph::new(Directionality::Undirected, 8, vec![(1, 2), (1, 4), (2, 3), (2, 6), (3, 4), (4, 8), (5, 1), (5, 8), (6, 5), (6, 7), (7, 3), (8, 7)]));
    graphs.push(Graph::new(Directionality::Directed, 9, vec![(1, 2), (1, 3), (1, 5), (2, 4), (2, 5), (3, 5), (3, 6), (4, 5), (4, 8), (5, 6), (6, 7), (6, 9), (7, 4), (7, 5), (7, 8), (9, 7), (9, 8)]));
    graphs.push(Graph::new(Directionality::Undirected, 9, vec![(1, 2), (1, 3), (1, 5), (2, 4), (2, 5), (3, 5), (3, 6), (4, 5), (4, 8), (5, 6), (6, 7), (6, 9), (7, 4), (7, 5), (7, 8), (9, 7), (9, 8)]));
    for graph in graphs{
        println!("dfs:");
        graph.dfs(true);
        println!("bfs:");
        graph.bfs(true);
    }
}

fn test2(){
    let paths = fs::read_dir("./test_data/2").unwrap();
    for path in paths {
        let graph: Graph;
        if let Ok(mut lines) = read_lines(path.unwrap().path().display().to_string()) {
            let directionality = lines.next().unwrap().unwrap();
            let node_quantity: usize = lines.next().unwrap().unwrap().parse().unwrap();
            lines.next();
            let mut edges:Vec<(usize, usize)> = Vec::new();
            for line in lines {
                if let Ok(ip) = line {
                    edges.push(str_to_edge(&ip))
                }
            }
            if directionality == "D"{
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }else {
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }
            let result = graph.topological_sort();
            if graph.node_quantity <= 200{
                match result {
                    Ok(sorted) => println!("{:?}", sorted),
                    Err(e) => println!("{e}")
                }
            } else {
                match result {
                    Ok(_) => println!("DAG"),
                    Err(_) => println!("Not a DAG")
                }
            }
        }
    }
}

fn test3(){
    let paths = fs::read_dir("./test_data/3").unwrap();
    for path in paths {
        let graph: Graph;
        if let Ok(mut lines) = read_lines(path.unwrap().path().display().to_string()) {
            let directionality = lines.next().unwrap().unwrap();
            let node_quantity: usize = lines.next().unwrap().unwrap().parse().unwrap();
            lines.next();
            let mut edges:Vec<(usize, usize)> = Vec::new();
            for line in lines {
                if let Ok(ip) = line {
                    edges.push(str_to_edge(&ip))
                }
            }
            if directionality == "D"{
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }else {
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }
            println!("SCC = {:?}", graph.find_scc());
        }
    }
}

fn test4(){
    let paths = fs::read_dir("./test_data/4").unwrap();
    for path in paths {
        let graph: Graph;
        if let Ok(mut lines) = read_lines(path.unwrap().path().display().to_string()) {
            let directionality = lines.next().unwrap().unwrap();
            let node_quantity: usize = lines.next().unwrap().unwrap().parse().unwrap();
            lines.next();
            let mut edges:Vec<(usize, usize)> = Vec::new();
            for line in lines {
                if let Ok(ip) = line {
                    edges.push(str_to_edge(&ip))
                }
            }
            if directionality == "D"{
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }else {
                graph = Graph::new(Directionality::Directed, node_quantity, edges);
            }
            if graph.is_bipartite() {
                println!("Graph is bipartite");
            } else {
                println!("Graph NOT bipartite");
            }
        }
    }
}

fn main() {
    let items = vec!["Test 1", "Test 2", "Test 3", "Test 4", "Exit"];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");

        match selection.unwrap() + 1 {
            1 => test1(),
            2 => test2(),
            3 => test3(),
            4 => test4(),
            _ => break
        }
    }
}
