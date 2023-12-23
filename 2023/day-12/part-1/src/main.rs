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

// The solution is based on this Python solution, but with a HashMap
// instead of the cache function from functools:
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
}

fn get_possible_arrangements(record: Record,
                             num_done_in_group: usize,
                             memo: &mut HashMap<Record, usize>) -> usize {

    // We first check if we have already computed the solution
    if let Some(&s) = memo.get(&record) {
        return s;
    }

    if record.springs.len() == 0 {
        // Is this a solution? Did we handle and close all groups?
        if record.groups.len() == 0 && num_done_in_group == 0 {
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
        // If the next spring is unknown, we branch
        possible.push(Spring::Damaged);
        possible.push(Spring::Operational);
    } else {
        possible.push(record.springs[0].clone());
    }
    for s in possible {
        if s == Spring::Damaged {
            // Damaged -> Extend current group
            possible_arrangements += get_possible_arrangements(
                // &Record::new(
                Record {
                    springs: record.springs[1..].to_vec(), 
                    groups: record.groups.clone()
                },
                num_done_in_group + 1,
                memo
            );
        } else { // Operational
            if num_done_in_group > 0 {
                // If we were in a group that can be closed, close it
                if record.groups.len() > 0 && 
                   record.groups[0] == num_done_in_group {
                    possible_arrangements += get_possible_arrangements(
                        Record {
                            springs: record.springs[1..].to_vec(), 
                            groups: record.groups[1..].to_vec(),
                        },
                        0,
                        memo
                    );
                }
            } else {
                // If we are not in a group, move on to next symbol
                possible_arrangements += get_possible_arrangements(
                    Record {
                        springs: record.springs[1..].to_vec(), 
                        groups: record.groups.clone(),
                    },                   
                    0,
                    memo
                );
            }
        }
    }
    return possible_arrangements;
}

fn main() {
    let mut sum_possible_arrangements = 0;
    let mut memo = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let splitted_line = line.split(" ").into_iter().collect::<Vec<&str>>();
        let record = Record {
            springs: (splitted_line[0].to_owned() + ".") // This is needed to detect end of line
                .chars()
                .map(|c| c.into())
                .collect(),
            groups: splitted_line[1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
        };
        // println!("Condition Record: {:?}, Groups: {:?}", 
        //     splitted_line[0], 
        //     record.groups,
        // );
        let possible_arrangements = get_possible_arrangements(
            record,
            0,
            &mut memo
        );
        // println!("Arrangements: {}", possible_arrangements);
        sum_possible_arrangements += possible_arrangements;
    }  

    println!("Sum of possible arrangements: {}", sum_possible_arrangements);
 }
