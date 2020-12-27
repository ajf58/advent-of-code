use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    day01(&args);
}

fn day01(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Iterate over each line, parsing each line as an unsigned int.
    let expenses: Vec<u32> = f.lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();
    // find the two entries that sum to 2020
    for expense_a in &expenses {
        for expense_b in &expenses {
            if expense_a + expense_b == 2020 {
                println!("Result {}", expense_a * expense_b);
                break;
            }
        }
    }
}
