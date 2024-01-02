use indicatif::ProgressIterator;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Platform {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
    Unknown,
}

impl From<char> for Platform {
    fn from(c: char) -> Self {
        match c {
            '.' => Platform::EmptySpace,
            'O' => Platform::RoundedRock,
            '#' => Platform::CubeShapedRock,
            _ => Platform::Unknown,
        }
    }
}

fn roll_north(platform: &Vec<Vec<Platform>>) -> Vec<Vec<Platform>> {
    let mut rolled_platform: Vec<Vec<Platform>> = vec![];
    let mut first_empty_space;

    // Initialize with empty spaces
    for _ in 0..platform.len() {
        rolled_platform.push(vec![Platform::EmptySpace; platform[0].len()]);
    }

    for i in 0..platform[0].len() {
        first_empty_space = usize::MAX;
        for j in 0..platform.len() {
            match platform[j][i] {
                Platform::EmptySpace => {
                    if first_empty_space == usize::MAX {
                        first_empty_space = j;
                    }
                },
                Platform::RoundedRock => {
                    if first_empty_space != usize::MAX {
                        rolled_platform[first_empty_space][i] = Platform::RoundedRock;
                        first_empty_space = j - (j - first_empty_space - 1);
                    } else {
                        rolled_platform[j][i] = Platform::RoundedRock;
                    }
                },
                Platform::CubeShapedRock => {
                    first_empty_space = usize::MAX;
                    rolled_platform[j][i] = Platform::CubeShapedRock;
                },
                _ => println!("Unknown element found"),
            }
        }
    }

    return rolled_platform;
}

fn roll_west(platform: &Vec<Vec<Platform>>) -> Vec<Vec<Platform>> {
    let mut rolled_platform: Vec<Vec<Platform>> = vec![];
    let mut first_empty_space;

    // Initialize with empty spaces
    for _ in 0..platform.len() {
        rolled_platform.push(vec![Platform::EmptySpace; platform[0].len()]);
    }

    for i in 0..platform.len() {
        first_empty_space = usize::MAX;
        for j in 0..platform[i].len() {
            match platform[i][j] {
                Platform::EmptySpace => {
                    if first_empty_space == usize::MAX {
                        first_empty_space = j;
                    }
                },
                Platform::RoundedRock => {
                    if first_empty_space != usize::MAX {
                        rolled_platform[i][first_empty_space] = Platform::RoundedRock;
                        first_empty_space = j - (j - first_empty_space - 1);
                    } else {
                        rolled_platform[i][j] = Platform::RoundedRock;
                    }
                },
                Platform::CubeShapedRock => {
                    first_empty_space = usize::MAX;
                    rolled_platform[i][j] = Platform::CubeShapedRock;
                },
                _ => println!("Unknown element found"),
            }
        }
    }

    return rolled_platform;
}

fn roll_east(platform: &Vec<Vec<Platform>>) -> Vec<Vec<Platform>> {
    let mut rolled_platform: Vec<Vec<Platform>> = vec![];
    let mut first_empty_space;

    // Initialize with empty spaces
    for _ in 0..platform.len() {
        rolled_platform.push(vec![Platform::EmptySpace; platform[0].len()]);
    }

    for i in 0..platform.len() {
        first_empty_space = usize::MAX;
        for j in (0..platform[i].len()).rev() {
            match platform[i][j] {
                Platform::EmptySpace => {
                    if first_empty_space == usize::MAX {
                        first_empty_space = j;
                    }
                },
                Platform::RoundedRock => {
                    if first_empty_space != usize::MAX {
                        rolled_platform[i][first_empty_space] = Platform::RoundedRock;
                        first_empty_space -= 1;
                    } else {
                        rolled_platform[i][j] = Platform::RoundedRock;
                    }
                },
                Platform::CubeShapedRock => {
                    first_empty_space = usize::MAX;
                    rolled_platform[i][j] = Platform::CubeShapedRock;
                },
                _ => println!("Unknown element found"),
            }
        }
    }

    return rolled_platform;
}

fn roll_south(platform: &Vec<Vec<Platform>>) -> Vec<Vec<Platform>> {
    let mut rolled_platform: Vec<Vec<Platform>> = vec![];
    let mut first_empty_space;

    // Initialize with empty spaces
    for _ in 0..platform.len() {
        rolled_platform.push(vec![Platform::EmptySpace; platform[0].len()]);
    }

    for i in 0..platform[0].len() {
        first_empty_space = usize::MAX;
        for j in (0..platform.len()).rev() {
            match platform[j][i] {
                Platform::EmptySpace => {
                    if first_empty_space == usize::MAX {
                        first_empty_space = j;
                    }
                },
                Platform::RoundedRock => {
                    if first_empty_space != usize::MAX {
                        rolled_platform[first_empty_space][i] = Platform::RoundedRock;
                        first_empty_space -= 1;
                    } else {
                        rolled_platform[j][i] = Platform::RoundedRock;
                    }
                },
                Platform::CubeShapedRock => {
                    first_empty_space = usize::MAX;
                    rolled_platform[j][i] = Platform::CubeShapedRock;
                },
                _ => println!("Unknown element found"),
            }
        }
    }

    return rolled_platform;
}

fn calculate_load(platform: &Vec<Vec<Platform>>) -> usize {
    let mut load = 0;

    for i in 0..platform.len() {
        let rounded_rocks = platform[i]
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Platform::RoundedRock)
            .collect::<Vec<_>>()
            .len();
        // println!("Row {}, Rounded Rocks: {}", i, rounded_rocks);
        load += rounded_rocks * (platform.len() - i);
    }

    return load;
}

// Code from https://doc.rust-lang.org/std/hash/index.html
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let load_final_iteration;

    let mut platform: Vec<Vec<Platform>> = vec![];
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        platform.push(line.chars().map(|c| c.into()).collect());
    }
    
    // It is not possible to do 100M iterations but,
    // from a certain iteration, the positions will be 
    // repeated in a cycle, so we need to find the first
    // repeated position and then calculate the final position

    // To check if two positions are equal, we will 
    // calculate a hash of the position
    let mut positions = HashMap::new();
    let mut index_repeated_position = usize::MAX;
    let mut hash = 0;
    const NUMBER_OF_CYCLES: usize = 1_000_000_000;
    for i in (0..NUMBER_OF_CYCLES).progress() {
        platform = roll_north(&platform);
        platform = roll_west(&platform);
        platform = roll_south(&platform);
        platform = roll_east(&platform);
        hash = calculate_hash(&platform);
        if positions.get(&hash) == None {
            // We store the hash with the cycle index
            positions.insert(hash, i);
        } else {
            index_repeated_position = i;
            break;
        }
    }

    if index_repeated_position != usize::MAX {
        println!(
            "Repeated position found, Iteration {}", 
            index_repeated_position
        );
    }            

    // The cycle length is the difference
    // between the current cycle and the 
    // first cycle where the position was found
    let cycle_length = index_repeated_position - positions.get(&hash).unwrap();
    println!("Cycle length: {}", cycle_length);

    // Now we calculate how many cycle lenghts
    // remain to reach the iteration 100M
    let remaining_cycles = 
        (NUMBER_OF_CYCLES - index_repeated_position) / 
        cycle_length;
    println!("Remaining cycles: {}", remaining_cycles);

    // Now we jump to the last iteration of the
    // first repeated position and execute the final cycles
    let first_iteration = index_repeated_position + cycle_length * remaining_cycles + 1;
    println!("Jumping to iteration {}", first_iteration);
    for _ in first_iteration..NUMBER_OF_CYCLES {
        platform = roll_north(&platform);
        platform = roll_west(&platform);
        platform = roll_south(&platform);
        platform = roll_east(&platform);
    }

    load_final_iteration = calculate_load(&platform);

    println!("Load final iteration: {}", load_final_iteration);
}
