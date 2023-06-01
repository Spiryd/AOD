use std::{io, fs};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::thread;
use std::time::{Instant};

use dialoguer::{console::Term, Input, theme::ColorfulTheme, Select};

use lib::*;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let choices = ["Test", "USA", "Manual"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SELECT MODE")
        .items(&choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if selection.is_none() {
        panic!("User did not select anything")
    }

    match selection.unwrap() {
        0 => test(),
        1 => usa(),
        2 => manual(),
        _ => panic!("somthing went wrong"),
    }    

    
}

fn test() {
    let input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide inputs directory path")
        .interact_text()
        .unwrap();
    let paths = fs::read_dir(input).unwrap();
    let mut graphs: Vec<String> = Vec::new();
    let mut challanges: Vec<String> = Vec::new();
    //let paths = fs::read_dir(r"C:\\Users\\neuma\\Desktop\\TEST".to_string()).unwrap();
    for path in paths {
        let tmp = path.unwrap().path().display().to_string();
        println!("{:?}", tmp);
        if tmp.contains(".gr") {
            graphs.push(tmp);
        } else if tmp.contains(".ss") {
            challanges.push(tmp);
        }
    }
    let mut file = File::create("./data/dijkstra_data.csv").unwrap();
    for i in 0..graphs.len() {
        file.write_all(b"algo;graph;n;e;time\n").unwrap();
        println!("{}", graphs[i]);
        let name = graphs[i].replace("C:\\Users\\neuma\\Desktop\\TEST\\", "");
        let mut n: usize = 0;
        let mut e: usize = 0;
        let mut g = Graph::default();
        if let Ok(lines) = read_lines(graphs[i].clone()) {
            for line in lines.flatten() {
                if line.starts_with('p') {
                    n = line.split_whitespace().collect::<Vec<&str>>()[2].parse().unwrap();
                    e = line.split_whitespace().collect::<Vec<&str>>()[3].parse().unwrap();
                    g = Graph::new(n);
                } else if line.starts_with('a') {
                    let tmp = line.split_whitespace().collect::<Vec<&str>>();
                    g.add_edge(tmp[1].parse().unwrap(), tmp[2].parse().unwrap(), tmp[3].parse().unwrap())
                }
            }
        }
        let mut challenge_file = "".to_string();
        for challange in challanges.clone() {
            if challange.contains(&graphs[i].clone().replace(".gr", "")) {
                challenge_file = challange;
                break;
            }
        }
        let mut ss: Vec<usize> = Vec::new();
        if let Ok(lines) = read_lines(challenge_file) {
            for line in lines.flatten() {
                if line.starts_with('s') {
                    let tmp = line.split_whitespace().collect::<Vec<&str>>();
                    ss.push(tmp[1].parse().unwrap());
                }
            }
        }
        for src in ss {
            let mut start = Instant::now();
            g.djikstra_classic_ss(src);
            file.write_all(format!("classic;{};{};{};{}\n", name, n, e, start.elapsed().as_secs_f32()).as_bytes()).unwrap();
            start = Instant::now();
            g.dial_ss(src);
            file.write_all(format!("dial;{};{};{};{}\n", name, n, e, start.elapsed().as_secs_f32()).as_bytes()).unwrap();
            //g.radix_ss(src);
            //println!("{:?}", g.djikstra_classic_ss(src));
            //println!("{:?}", g.dial_ss(src));
            //println!("{:?}", g.radix_ss(src));
        }
    }
}

fn manual() {
    let alorithm_choices = vec!["basic", "dial's", "radix"];
    let algorithm_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SELECT ALGORITHM")
        .items(&alorithm_choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if algorithm_selection.is_none() {
        panic!("User did not select anything")
    }

    let graph_input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide graph file")
        .interact_text()
        .unwrap();

    let mut g = Graph::default();
    if let Ok(lines) = read_lines(graph_input) {
        for line in lines.flatten() {
            if line.starts_with('p') {
                g = Graph::new(line.split_whitespace().collect::<Vec<&str>>()[2].parse().unwrap());
            } else if line.starts_with('a') {
                let tmp = line.split_whitespace().collect::<Vec<&str>>();
                g.add_edge(tmp[1].parse().unwrap(), tmp[2].parse().unwrap(), tmp[3].parse().unwrap())
            }
        }
    }

    let challange_input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide challange file")
        .interact_text()
        .unwrap();

    let mut ss: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines(challange_input) {
        for line in lines.flatten() {
            if line.starts_with('s') {
                let tmp = line.split_whitespace().collect::<Vec<&str>>();
                ss.push(tmp[1].parse().unwrap());
            }
        }
    }

    let res_input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide resoult file")
        .interact_text()
        .unwrap();

    for src in ss {
        g.djikstra_classic_ss(src);
        g.dial_ss(src);
        //g.radix_ss(src);
        //println!("{:?}", g.djikstra_classic_ss(src));
        //println!("{:?}", g.dial_ss(src));
        //println!("{:?}", g.radix_ss(src));
    }
    
}

fn usa() {
    let input : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide inputs directory path")
        .interact_text()
        .unwrap();
    let paths = fs::read_dir(input).unwrap();


    let graph_choices = vec!["NY", "BAY", "COL", "FLA", "NW", "NE", "CAL", "LKS", "E", "W", "CRT"];
    let graph_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SELECT GRAPH")
        .items(&graph_choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if graph_selection.is_none() {
        panic!("User did not select anything")
    }

    let challenge_choices = vec!["p2p", "ss"];
    let challenge_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SELECT CHALLENGE")
        .items(&challenge_choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if challenge_selection.is_none() {
        panic!("User did not select anything")
    }

    let alorithm_choices = vec!["basic", "dial's", "radix"];
    let algorithm_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SELECT ALGORITHM")
        .items(&alorithm_choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if algorithm_selection.is_none() {
        panic!("User did not select anything")
    }

    let mut graph_file: String = "".to_string();
    let mut challenge_file: String = "".to_string();

    for path in paths{
        let tmp = path.unwrap().path().display().to_string();
        if tmp.contains(graph_choices[graph_selection.unwrap()]){
            if tmp.contains(".gr"){
                graph_file = tmp.clone();
            }
            if tmp.contains(challenge_choices[challenge_selection.unwrap()]){
                challenge_file = tmp.clone();
            }
        }
    }
    //println!("{:?}", graph_file);
    //println!("{:?}", challenge_file);
    let mut g = Graph::default();
    if let Ok(lines) = read_lines(graph_file) {
        for line in lines.flatten() {
            if line.starts_with('p') {
                g = Graph::new(line.split_whitespace().collect::<Vec<&str>>()[2].parse().unwrap());
            } else if line.starts_with('a') {
                let tmp = line.split_whitespace().collect::<Vec<&str>>();
                g.add_edge(tmp[1].parse().unwrap(), tmp[2].parse().unwrap(), tmp[3].parse().unwrap())
            }
        }
    }

    let start: Instant = Instant::now();
    //p2p if 0
    if challenge_selection.unwrap() == 0 {
        let mut p2ps: Vec<(usize, usize)> = Vec::new();
        if let Ok(lines) = read_lines(challenge_file) {
            for line in lines.flatten() {
                if line.starts_with('q') {
                    let tmp = line.split_whitespace().collect::<Vec<&str>>();
                    p2ps.push((tmp[1].parse().unwrap(), tmp[2].parse().unwrap()));
                }
            }
        }
        for p2p in p2ps{
            let gc = g.clone();
            match algorithm_selection.unwrap() {
                0 => {
                    thread::spawn(move || {  
                        println!("{:?}", gc.clone().djikstra_classic_p2p(p2p.0, p2p.1));
                    });
                },
                1 => {
                    println!("{:?}", g.dial_p2p(p2p.0, p2p.1))
                },
                2 => {
                    println!("{:?}", g.radix_p2p(p2p.0, p2p.1))
                },
                _ => panic!("Something went wrong!")
            }
        }
    } else {
        let mut ss: Vec<usize> = Vec::new();
        if let Ok(lines) = read_lines(challenge_file) {
            for line in lines.flatten() {
                if line.starts_with('s') {
                    let tmp = line.split_whitespace().collect::<Vec<&str>>();
                    ss.push(tmp[1].parse().unwrap());
                }
            }
        }
        for src in ss{
            match algorithm_selection.unwrap() {
                0 => {
                    println!("{:?}", g.djikstra_classic_ss(src))
                },
                1 => {
                    println!("{:?}", g.dial_ss(src))
                },
                2 => {
                    println!("{:?}", g.radix_ss(src))
                   // g.radix_ss(src);
                },
                _ => panic!("Something went wrong!")
            }
        }
    }
    println!("TIME:{}", start.elapsed().as_secs_f64())
}