use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    day05(&args);
}

// Binary Boarding. Decode the boarding passes to find the highest seat ID.
fn day05(args: &Vec<String>) {
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut max_id: u32 = 0;
    //let boarding_passes = fs::read_to_string(filename).expect("Unable to read file");
    for line in f.lines() {
        let boarding_pass = line.unwrap();
        let mut column = 0;
        let mut row = 0;
        for (i, c) in boarding_pass[7..].chars().rev().enumerate() {
            column += match c {
                'R' => 1 << i,
                _ => 0,
            };
        }
        for (i, c) in boarding_pass[..7].chars().rev().enumerate() {
            row += match c {
                'B' => 1 << i,
                _ => 0,
            };
        }
        let seat_id = (row * 8) + column;
        if seat_id > max_id {
            max_id = seat_id;
        }
        //println!("{:?}: row {:?}, column {:?}, seat ID {:?}", boarding_pass, row, column, seat_id);
    }
    println!("Max ID: {:?}", max_id);
}

fn valid_year(value: &str, min: u32, max: u32) -> bool {
    match value.parse::<u32>() {
        Ok(year) => (year >= min) && (year <= max),
        Err(e) => false,
    }
}

fn valid_height(value: &str) -> bool {
    /*
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    */
    let number = match value[..value.len() - 2].parse::<u32>() {
        Ok(number) => number,
        Err(e) => 0,
    };
    match &value[value.len() - 2..] {
        "cm" => (number >= 150) && (number <= 193),
        "in" => (number >= 59) && (number <= 76),
        _ => false,
    }
}

// Detect which passports have all required fields.
fn day04(args: &Vec<String>) {
    let filename = &args[1];
    let mut valid_passports: u32 = 0;
    let records = fs::read_to_string(filename).expect("Unable to read file");
    for record in records.split("\n\n") {
        // Verify if any fields are missing.
        let mut valid_passport = true;
        for kv in record.split_ascii_whitespace() {
            //println!("{:?}", kv);
            let key = &kv[..3];
            let value = &kv[4..];
            /*
                byr (Birth Year) - four digits; at least 1920 and at most 2002.
                iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                eyr (Expiration Year) - four digits; at least 2020 and at most 2030.

                hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                pid (Passport ID) - a nine-digit number, including leading zeroes.
                cid (Country ID) - ignored, missing or not.
            */
            let valid_value = match key {
                "byr" => valid_year(value, 1920, 2002),
                "iyr" => valid_year(value, 2010, 2020),
                "eyr" => valid_year(value, 2020, 2030),
                "hgt" => valid_height(value),
                "hcl" => (value.len() == 7) && Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(value),
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                "pid" => (value.len() == 9) && Regex::new(r"[0-9]{9}").unwrap().is_match(value),
                "cid" => true,
                _ => false,
            };
            if !valid_value {
                //println!("Invalid {:?}", kv);
                valid_passport = false;
                break;
            }
        }

        // Passport is valid if all the required fields are present, and no fields were invalid.
        valid_passport &= ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|x| record.contains(x));
        if valid_passport {
            valid_passports += 1;
        }
    }
    println!("Result {:?}", valid_passports);
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
    for (i, line) in f.lines().enumerate() {
        // We always start on an empty square, so skip the first row.
        if i == 0 {
            continue;
        }
        let row = line.unwrap();
        let len = row.len();
        for j in 0..4 {
            if let Some('#') = row.chars().nth((i * ((2 * j) + 1)) % len) {
                num_trees[j] += 1;
            }
        }
        // Handle right 1, down 2.
        if i % 2 == 0 {
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
        let range: Vec<u32> = v[0].split('-').map(|x| x.parse::<u32>().unwrap()).collect();
        let min = &range[0];
        let max = &range[1];
        let policy_char = v[1].chars().nth(0).unwrap();
        let num_chars: u32 = v[2].chars().filter(|x| *x == policy_char).count() as u32;

        if (min <= &num_chars) && (&num_chars <= max) {
            policy_a += 1;
        }
        if (v[2].chars().nth((*min - 1) as usize).unwrap() == policy_char)
            ^ (v[2].chars().nth((*max - 1) as usize).unwrap() == policy_char)
        {
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
    let expenses: Vec<u32> = f
        .lines()
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
