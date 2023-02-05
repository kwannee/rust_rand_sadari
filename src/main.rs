use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let names = split_str(&input);
    println!("{:?}", names);
}

fn split_str(str: &str) -> Vec<&str> {
    str.split(' ').collect()
}
