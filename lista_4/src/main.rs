use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::time::Instant;
use std::fs::File;
use std::io::Write;

use lib::*;

fn main() {
    let choices = ["Edmonds-Karp Algorithm", "Maximum Cardinality Matching", "Dinic's Algorithm", "ALL"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Collect Data On:")
        .items(&choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if selection.is_none() {
        panic!("User did not select anything")
    }

    match selection.unwrap() {
        0 => collect_edmonds_karp_data(),
        1 => collect_mcm_data(),
        2 => collect_dinic_data(),
        3 => {
            collect_edmonds_karp_data();
            collect_mcm_data();
            collect_dinic_data();
        },
        _ => panic!("somthing went wrong"),
    }   
}

fn collect_edmonds_karp_data() {
    let mut file = File::create("./data/edmonds_karp.csv").unwrap();
    file.write_all(b"n;flow;time;aug_count\n").unwrap();
    for n in 1..=16 {
        for _ in 0..100 {
            let mut hyper_cube = Hypercube::new(n);
            println!("{n}-dimensional Hypercube created");
            let start = Instant::now();
            let (max_flow, aug_count) = hyper_cube.edmonds_karp(0,  2_usize.pow(n) - 1);
            let duration =  start.elapsed().as_nanos();
            file.write_all(format!("{n};{max_flow};{duration};{aug_count}\n").as_bytes()).unwrap();
        }
    }
}

fn collect_mcm_data() {
    let mut file = File::create("./data/mcm.csv").unwrap();
    file.write_all(b"k;i;size;time\n").unwrap();
    for k in 3..=10 {
        for i in 1..=k {
            for _ in 0..1000{
                let bigraph = Bigraph::new(k, i as usize);
                let start = Instant::now();
                let size = bigraph.hopcroft_karp();
                let duration =  start.elapsed().as_nanos();
                file.write_all(format!("{k};{i};{size};{duration}\n").as_bytes()).unwrap();
            }
        }
    }
}

fn collect_dinic_data() {
    let mut file = File::create("./data/dinic.csv").unwrap();
    file.write_all(b"n;flow;time;aug_count\n").unwrap();
    for n in 1..=16 {
        for _ in 0..100 {
            let mut hyper_cube = Hypercube::new(n);
            println!("{n}-dimensional Hypercube created");
            let start = Instant::now();
            let (max_flow, aug_count) = hyper_cube.dinic(0,  2_usize.pow(n) - 1);
            let duration =  start.elapsed().as_nanos();
            file.write_all(format!("{n};{max_flow};{duration};{aug_count}\n").as_bytes()).unwrap();
        }
    }
}
