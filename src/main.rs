use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap,HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut letters = HashMap::new();
    for box_id in f.lines() {
        // How many times does this letter appear in the box ID?
        for letter in line.unwrap().chars() {
            let count = letters.entry(letter).or_insert(0);
            *count += 1;
        }
        for (_, letter_count) in &letters {

        }
    }
}

fn day01(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Iterate over each line, parsing each line as a signed int.
    let frequency_shifts: Vec<i32> = f.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    let mut frequency_offsets = HashSet::new();
    let mut current_offset = 0;
    for shift in frequency_shifts.iter().cycle() {
        current_offset += shift;
        // Record the current offset. If the offset has been visited before, then break.
        if !frequency_offsets.insert(current_offset) {
            break
        }
    }
    println!("Result {}", current_offset);
}
