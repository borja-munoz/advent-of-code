use std::collections::HashMap;
use std::fs;

struct Element {
    left_node: String,
    right_node: String,
}

// Greatest common divisor
fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

// Least common multiple
fn lcm(a: u128, b: u128) -> u128 {
    return a * (b / gcd(a, b));
}

fn main() {
    let mut steps: usize;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let instructions = lines[0];

    let mut nodes = HashMap::new();
    let mut index_line = 2;
    while index_line < lines.len() {
        let splitted_line = lines[index_line].split("=").into_iter().collect::<Vec<&str>>();
        let node = splitted_line[0].trim_end();
        let left_right_node = splitted_line[1].split(",").into_iter().collect::<Vec<&str>>();
        nodes.insert(
            node,
            Element {
                left_node: left_right_node[0][2..5].to_string(),
                right_node: left_right_node[1][1..4].to_string()
            }
        );
        index_line += 1;
    }   

    // Starting nodes end with "A"
    let mut starting_nodes: Vec<&str> = vec![];
    for node in nodes.keys() {
        if node.chars().nth(2).unwrap() == 'A' {
            starting_nodes.push(node);
        }
    }

    // All the starting nodes reach an ending node after X steps 
    // and then there is a cycle of X steps after reaching an 
    // ending node again.
    // So we need to calculate the number steps to the ending node 
    // for each starting node and then calculate the least common 
    // multiple of the steps for each node.

    let instructions_length = instructions.len();

    let mut steps_ending_node: Vec<u128> = vec![];
    for starting_node in starting_nodes {
        steps = 0;
        let mut node = starting_node;
        while node.chars().nth(2).unwrap() != 'Z' {
            let instruction = instructions.chars().nth(steps % instructions_length).unwrap();
            if instruction == 'L' {
                node = nodes.get(node).unwrap().left_node.as_str();
            } else {
                node = nodes.get(node).unwrap().right_node.as_str();
            }    
            steps += 1;
        }
        steps_ending_node.push(u128::try_from(steps).unwrap());
        println!("Ending node reached after {} steps", steps);
    }

    // Calculate the least common multiple
    let mut least_common_multiple = lcm(steps_ending_node[0], steps_ending_node[1]);
    let mut index_steps = 2;
    while index_steps < steps_ending_node.len() {
        least_common_multiple = lcm(least_common_multiple, steps_ending_node[index_steps]);
        index_steps += 1;
    }

    println!("Steps to ending node: {:?}, LCM: {}", steps_ending_node, least_common_multiple);
 }
