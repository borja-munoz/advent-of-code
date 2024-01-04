use std::fs;
use std::collections::HashMap;

fn calculate_hash(step: &str) -> usize {
    let mut hash: usize = 0;
    for c in step.chars() {
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    };
    return hash;
}

fn arrange_boxes(initialization_sequence: Vec<&str>) -> HashMap<usize, Vec<(&str, &str)>> {
    let mut boxes: HashMap<usize, Vec<(&str, &str)>> = HashMap::new();
    for step in initialization_sequence {
        if step.find("=") != None {
            let split_step = step.split("=").collect::<Vec<&str>>();
            let label = split_step[0];
            let focal_length = split_step[1];
            let hash = calculate_hash(label);
            boxes.entry(hash.clone())
                 .and_modify(|vec: &mut Vec<_>| {
                    let index = vec.iter().position(|l: &(_, _)| l.0 == label);
                    if index == None {
                        vec.push((label, focal_length));
                    } else {
                        vec[index.unwrap()] = (label, focal_length);
                    }
                 })
                 .or_insert(vec![(label, focal_length)]);
        } else {
            let label = step.split("-").collect::<Vec<&str>>()[0];
            let hash = calculate_hash(label);
            boxes.entry(hash.clone())
                 .and_modify(|vec: &mut Vec<_>| vec.retain(|&l| l.0 != label));
        }
    }
    boxes
}

fn main() {
    let mut focusing_power = 0;

    let line = fs::read_to_string("input.txt").unwrap();

    let initialization_sequence = line.split(",").collect::<Vec<_>>();

    let boxes = arrange_boxes(initialization_sequence);
    // println!("Boxes: {:?}", boxes);

    for (box_index, lenses) in boxes {
        for (lens_index, lens) in lenses.iter().enumerate() {
            focusing_power += 
                (box_index + 1) * 
                (lens_index + 1) * 
                lens.1.parse::<usize>().unwrap();
        }
    }

    println!("Focusing power: {}", focusing_power);
}
