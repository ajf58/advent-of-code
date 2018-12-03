use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Iterate over each line, parsing each line as a signed int, and sum.
    let frequency_shifts: Vec<i32> = f.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    let mut frequency_offsets = HashSet::new(); 
    let mut current_offset = 0;
    let mut found = false;
    while !found {
        for shift in &frequency_shifts {
            current_offset += shift;
            found = !frequency_offsets.insert(current_offset);
            if found {
                break
            }
        }
    }

    println!("Result {}", current_offset);
}
