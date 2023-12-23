use std::collections::HashMap;
use std::fs;

// I am going to use Dynamic Programming to solve the problem.
// Dynamic Programming works when we use recursion to solve
// a problem by dividing it into subproblems (like Fibonacci or
// factorial) and some of the subproblems are overlapping so we 
// can reuse the solutions we have already found.

// I have tried to use memoization with the memoize crate but 
// I think the function that calculate the possible solutions
// cannot be memoized.

// I'm using a solution copied from this gist:
// https://gist.github.com/icub3d/7aa45ca96ccb88ebf95b91d6a28eba74
// where the developer is using a HashMap to store the 
// solutions and avoid calculating them again.
// I'm also copying the data structure to store the information.

// The solution is based on this Python solution translated to Rust, 
// but with a HashMap instead of the cache function from functools:
// https://github.com/fuglede/adventofcode/blob/master/2023/day12/solutions.py

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => Spring::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
    num_done_in_group: usize,
}

fn get_possible_arrangements(record: &Record,
                             memo: &mut HashMap<Record, usize>) -> usize {

    // We first check if we have already computed the solution
    if let Some(&s) = memo.get(record) {
        return s;
    }

    if record.springs.len() == 0 {
        // This is a solution if we have handled all the groups
        if record.groups.len() == 0 && record.num_done_in_group == 0 {
            memo.insert(record.clone(), 1);
            return 1;
        } else {
            memo.insert(record.clone(), 0);
            return 0;
        }
    }
    let mut possible_arrangements = 0;
    let mut possible: Vec<Spring> = vec![];
    if record.springs[0] == Spring::Unknown {
        // If the next spring is unknown, we need to
        // consider both options
        possible.push(Spring::Damaged);
        possible.push(Spring::Operational);
    } else {
        // If it is known, we just process it
        possible.push(record.springs[0].clone());
    }
    for s in possible {
        if s == Spring::Damaged {
            // Damaged -> Extend current group
            possible_arrangements += get_possible_arrangements(
                &Record {
                    springs: record.springs[1..].to_vec(), 
                    groups: record.groups.clone(),
                    num_done_in_group: record.num_done_in_group + 1
                },
                memo
            );
        } else { 
            // Operational
            if record.num_done_in_group > 0 {
                // If we were in a group that can be closed, close it
                if record.groups.len() > 0 && 
                   record.groups[0] == record.num_done_in_group {
                    possible_arrangements += get_possible_arrangements(
                        &Record {
                            springs: record.springs[1..].to_vec(), 
                            groups: record.groups[1..].to_vec(),
                            num_done_in_group: 0
                        },
                        memo
                    );
                }
            } else {
                // If we are not in a group, move on to next symbol
                possible_arrangements += get_possible_arrangements(
                    &Record {
                        springs: record.springs[1..].to_vec(), 
                        groups: record.groups.clone(),
                        num_done_in_group: 0
                    },                   
                    memo
                );
            }
        }
    }
    memo.insert(record.clone(), possible_arrangements);    

    return possible_arrangements;
}

fn main() {
    let mut sum_possible_arrangements = 0;
    let mut memo = HashMap::new();

    let mut i = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let now = std::time::Instant::now();
        let splitted_line = line.split(" ").into_iter().collect::<Vec<&str>>();
        let springs_before_expansion: Vec<Spring> = 
            splitted_line[0] 
            .chars()
            .map(|c| c.into())
            .collect();
        let groups_before_expansion: Vec<usize> = 
            splitted_line[1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut expanded_springs: Vec<Spring> = vec![];
        for i in 1..6 {
            expanded_springs.append(&mut springs_before_expansion.clone());
            if i != 5 {
                expanded_springs.push(Spring::Unknown);
            }
        }
        expanded_springs.push(Spring::Operational); // This is needed to detect end of line
        let mut expanded_groups: Vec<usize> = vec![];
        for _ in 1..6 {
            expanded_groups.append(&mut groups_before_expansion.clone());
        }
        let record = Record {
            springs: expanded_springs,
            groups: expanded_groups,
            num_done_in_group: 0,
        };
        let possible_arrangements = get_possible_arrangements(
            &record,
            &mut memo
        );
        sum_possible_arrangements += possible_arrangements;
        i += 1;
        println!("Record {} of 1000. Elapsed: {:?}", i, now.elapsed());
    }  

    println!("Sum of possible arrangements: {}", sum_possible_arrangements);
 }
