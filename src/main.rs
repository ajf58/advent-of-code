use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    day03(&args);
}

fn day03(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2. count every other row? row 0, 2, 4, ...
    */
    let mut num_trees = [0; 5];
    // traverse right 3, down 1, Looping around.
    // We always start on an empty square, so skip the first row.
    for (i, line) in f.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let row = line.unwrap();
        let len = row.len();
        //println!("Result {:?} {:?} {:?}", row, offset, cell_contents);
        for j in 0..4 {
            if let Some('#') = row.chars().nth((i * ((2 * j) + 1)) % len) {
                num_trees[j] += 1;
            }
        }
        if  i % 2 == 0 {
            if let Some('#') = row.chars().nth((i / 2) % len) {
                num_trees[4] += 1;
            }
        }
    }
    let result: u64 = num_trees.iter().fold(1, |acc, x| (acc * x));
    println!("Result {:?}, {:?}", num_trees, result);
}

fn day02(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut policy_a = 0;
    let mut policy_b = 0;
    // Split the line into the policy and password. Example line:
    // 1-3 a: abcde
    for line in f.lines() {
        let s = line.unwrap();
        let v: Vec<&str> = s.split_whitespace().collect();
        let range: Vec<u32> = v[0].split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let min = &range[0];
        let max = &range[1];
        let policy_char = v[1].chars().nth(0).unwrap();
        let num_chars: u32 = v[2].chars().filter(|x| *x == policy_char).count() as u32;
        
        if (min <= &num_chars) && (&num_chars <= max) {
            policy_a += 1;
        }
        if (v[2].chars().nth((*min - 1) as usize).unwrap() == policy_char) ^ (v[2].chars().nth((*max - 1) as usize).unwrap() == policy_char) {
            policy_b += 1;
        }
    }
    println!("Result {:?} {:?}", policy_a, policy_b);
}

fn day01(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Iterate over each line, parsing each line as an unsigned int.
    let expenses: Vec<u32> = f.lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();
    day01part01(&expenses);
    day01part02(&expenses);
}

// find the two entries that sum to 2020, and multiply together.
fn day01part01(expenses: &Vec<u32>) {
    let mut result = 0;
    for expense_a in expenses {
        for expense_b in expenses {
            if expense_a + expense_b == 2020 {
                result = expense_a * expense_b;
                break;
            }
        }
        if result != 0 {
            break;
        }
    }
    println!("Result {}", result);
}

// find the three entries that sum to 2020, and multiply together.
fn day01part02(expenses: &Vec<u32>) {
    let mut result = 0;
    for expense_a in expenses {
        for expense_b in expenses {
            for expense_c in expenses {
                if expense_a + expense_b + expense_c == 2020 {
                    result = expense_a * expense_b * expense_c;
                    break;
                }
            }
            if result != 0 {
                break;
            }
        }
        if result != 0 {
            break;
        }
    }
    println!("Result {}", result);
}
