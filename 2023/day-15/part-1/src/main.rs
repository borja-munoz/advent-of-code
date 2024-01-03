use std::fs;

fn calculate_hash(step: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in step.chars() {
        hash += c as u32;
        hash *= 17;
        hash = hash % 256;
    };
    return hash;
}

fn main() {
    let mut sum_hash = 0;

    let line = fs::read_to_string("input.txt").unwrap();

    let initialization_sequence = line.split(",").collect::<Vec<_>>();

    for step in initialization_sequence {
        sum_hash += calculate_hash(step);    
    }

    println!("Sum of hashes: {}", sum_hash);
}
