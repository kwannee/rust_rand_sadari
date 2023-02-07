use rand::prelude::*;
use std::io;

fn main() {
    let names_input = get_input();
    let names = split_str(&names_input, " ");
    let winner = pick_random_name(names);
    println!("{}", winner);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn split_str<'a>(str: &'a str, separator: &'a str) -> Vec<&'a str> {
    str.split(separator).collect()
}

fn pick_random_name(names: Vec<&str>) -> &str {
    names[gen_random_number((names.len() - 1) as f64)]
}

fn gen_random_number(limit: f64) -> usize {
    let mut rng = thread_rng();
    let random_number: f64 = rng.gen_range(0 as f64..limit);
    random_number as usize
}
