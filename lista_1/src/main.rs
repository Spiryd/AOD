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

    fn find_zero_indegrees(&self) -> Vec<usize> {
        let mut zero_indegree: Vec<usize> = (1..=self.node_quantity).collect();
        for i in self.adj.clone() {
            for j in i {
                if zero_indegree.contains(&j){
                    let index = zero_indegree.iter().position(|x| *x == j).unwrap();
                    zero_indegree.swap_remove(index);
                }
            }
        }
        return zero_indegree;
    }

    fn has_zero_indegree(n: usize, graph: Vec<Vec<usize>>) -> bool {
        for outcoming_node_edges in graph{
            for node in outcoming_node_edges {
                if node == n {
                    return false;
                }
            }
        }
        return true;
    }

    fn has_edges(graph: Vec<Vec<usize>>) -> bool{
        for  outcoming_node_edges in graph {
            if !outcoming_node_edges.is_empty(){
                return true;
            }
        }
        return false;
    }

    pub fn topological_sort(&self) -> Result<Vec<usize>, String> {
        let mut sorted: Vec<usize> = Vec::new();
        let mut zero_indegree: Vec<usize> = self.find_zero_indegrees();
        let mut working_graph = self.adj.clone();
        while !zero_indegree.is_empty() {
            let n = zero_indegree.pop().unwrap();
            sorted.push(n);
            for m in working_graph[n - 1].clone() {
                let index = working_graph[n - 1].iter().position(|x| *x == m).unwrap();
                working_graph[n-1].swap_remove(index);
                if Self::has_zero_indegree(m, working_graph.clone()) {
                    zero_indegree.push(m);
                }
            }
        }
        if Self::has_edges(working_graph) {
            panic!("Not a DAG")
        }else {
            Ok(sorted)
        }
    }

    fn strong_connect(&self, node: usize, index: &mut usize, node_index: &mut Vec<Option<usize>>, low_link: &mut Vec<usize>, stack: &mut Vec<usize>, strongly_connected_cmps: &mut  Vec<Vec<usize>>) {
        node_index[node - 1] = Some(*index);
        low_link[node - 1] = *index;
        *index += 1;
        stack.push(node);

        for edge_outgoing in self.adj[node - 1].clone() {
            if node_index[edge_outgoing - 1].is_none(){
                self.strong_connect(edge_outgoing, index, node_index, low_link, stack, strongly_connected_cmps);
                low_link[node - 1] = std::cmp::min(low_link[node - 1], low_link[edge_outgoing - 1]);
            } else if stack.contains(&edge_outgoing) {
                low_link[node - 1] = std::cmp::min(low_link[node - 1], node_index[edge_outgoing - 1].unwrap());
            }
        }

        if low_link[node - 1] == node_index[node - 1].unwrap() {
            let mut strong_connected_cmp: Vec<usize> = Vec::new();
            loop {
                let other_node = stack.pop().unwrap();
                strong_connected_cmp.push(other_node);
                if other_node == node {
                    break;
                }
            }
        }

    }

    pub fn find_strongly_connected_cmp(&self) -> Vec<Vec<usize>>{
        let mut strongly_connected_cmps: Vec<Vec<usize>> = Vec::new();
        let mut node_index: Vec<Option<usize>> = vec![None; self.node_quantity];
        let mut low_link: Vec<usize> = vec![0; self.node_quantity];
        let mut index: usize = 0;
        let mut stack: Vec<usize> = Vec::new();
        for node in 1..=self.node_quantity {
            if node_index[node - 1].is_none() {
                self.strong_connect(node, &mut index, &mut node_index, &mut low_link, &mut stack, &mut strongly_connected_cmps);
            }
        }
        return strongly_connected_cmps;
    }

    pub fn is_bipartite(&self) -> bool{
        // -1 = uncolored, 0 = red, 1 = blue
        let mut color: Vec<i8> = vec![-1; self.node_quantity];
        //bfs queue (node, color)
        let mut queue: VecDeque<(usize, i8)> = VecDeque::new();

        for node in 1..=self.node_quantity {
            if color[node - 1] == -1 {
                queue.push_back((node, 0));
                color[node - 1] = 0;
                while !queue.is_empty() {
                    //(node, color)
                    let pair = queue.pop_front().unwrap();
                    for connected_node in self.adj[pair.0 - 1].clone() {
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

fn _str_to_edge(string: String) -> (usize, usize){
    let mut nodes: Vec<usize> = Vec::new();
    for node in string.split_whitespace(){
        nodes.push(node.trim().parse().expect("Should be a number!"));
    }
    (nodes.get(0).unwrap().to_owned(), nodes.get(1).unwrap().to_owned())
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
        edges.push(_str_to_edge(edge.clone()));
        edge = String::from("");
    } 

    if directionality.contains('D'){
        Graph::new(Directionality::Directed, node_quantity, edges)
    } else {
        Graph::new(Directionality::Undirected, node_quantity, edges)
    }
}

fn main() {
    let graph = Graph::new(Directionality::Directed, 6, vec![(1, 3), (1, 2), (3, 5), (3, 6), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6)]);
    println!("{:?}", graph);
    graph.dfs(false);
    graph.bfs(false);
    println!("{:?}", graph.topological_sort());
}
