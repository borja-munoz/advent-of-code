use std::collections::HashMap;
use std::fs;

struct Element {
    left_node: String,
    right_node: String,
}

fn main() {
    let mut steps = 0;

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

    // for (node, element) in nodes.iter() {
    //     println!("key: {} val: {}, {}", node, element.left_node, element.right_node);
    // }

    let initial_node = "AAA";
    let final_node = "ZZZ";
    let instructions_length = instructions.len();

    let mut node = initial_node;
    while node != final_node {
        let instruction = instructions.chars().nth(steps % instructions_length).unwrap();
        if instruction == 'L' {
            node = &nodes.get(node).unwrap().left_node;
        } else {
            node = &nodes.get(node).unwrap().right_node;
        }
        steps += 1;
    }

    println!("Steps: {}", steps);
 }
