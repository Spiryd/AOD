use std::{io, fs};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::thread;

use dialoguer::{console::Term, Input, theme::ColorfulTheme, Select};

use lib::*;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
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
                        println!("{:?}", gc.clone().djikstra_classic_p2p(p2p.clone().0, p2p.clone().1));
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
                },
                _ => panic!("Something went wrong!")
            }
        }
    }
}
