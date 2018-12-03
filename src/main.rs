use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Iterate over each line, parsing each line as a signed int, and sum.
    let result: i32 = f.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).sum();
    println!("Result {}", result);
}
