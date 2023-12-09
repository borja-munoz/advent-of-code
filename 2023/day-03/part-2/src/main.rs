use regex::{Match, Regex};
use std::fs;

// Process a line (string) to find adjacent part numbers
// to a symbol located in a given index
fn process_line(line: &str, symbol_index: usize) -> Vec<i32> {
    let mut adjacent_part_numbers: Vec<i32> = vec![];
    let re = Regex::new(r"[0-9]+").unwrap();

    let number_matches: Vec<Match> = re.find_iter(line).map(|m| m).collect();
    let mut number_index = 0;
    while number_index < number_matches.len() {
        let start_offset = number_matches[number_index].start();
        let end_offset = number_matches[number_index].end();
        if start_offset > 0 {
            if symbol_index >= start_offset - 1 &&
                symbol_index <= end_offset {
                adjacent_part_numbers.push(number_matches[number_index].as_str().parse::<i32>().unwrap());
            } 
        }
        else {
            if symbol_index <= end_offset {
                adjacent_part_numbers.push(number_matches[number_index].as_str().parse::<i32>().unwrap());
            } 
        }
        number_index += 1;
    }

    return adjacent_part_numbers;
}

fn main() {
    let mut sum_gear_ratios = 0;

    // We read the lines from the file into a vector
    let mut lines: Vec<&str> = vec![];
    let binding = fs::read_to_string("input.txt").unwrap();
    for line in binding.lines() {
        lines.push(line);
    }

    let re = Regex::new(r"[0-9]+").unwrap();
    let mut line_index = 0;
    while line_index < lines.len() {
        let line = lines[line_index];
        // Now we search for the symbol *
        let star_symbol_indices: Vec<_> = line.match_indices("*").into_iter().collect();

        // For each * symbol, we look in the same, previous and next line
        // to see if it is adjacent to exactly 2 part numbers
        let mut star_symbol_index = 0;
        while star_symbol_index < star_symbol_indices.len() {
            let mut adjacent_part_numbers: Vec<i32> = vec![];
            let (symbol_index, _pattern) = star_symbol_indices[star_symbol_index];

            // Look in the same line (adjacent chars)
            let number_matches: Vec<Match> = re.find_iter(line).map(|m| m).collect();
            let mut number_index = 0;
            while number_index < number_matches.len() {
                let start_offset = number_matches[number_index].start();
                let end_offset = number_matches[number_index].end();
                if start_offset == symbol_index + 1 {
                    adjacent_part_numbers.push(number_matches[number_index].as_str().parse::<i32>().unwrap());
                }
                else if end_offset == symbol_index {
                    adjacent_part_numbers.push(number_matches[number_index].as_str().parse::<i32>().unwrap());
                }
                number_index += 1;
            }

            // Look in the previous line
            if line_index > 0 {
                adjacent_part_numbers.append(&mut process_line(lines[line_index - 1], symbol_index));
            }

            // Look in the next line
            if line_index < lines.len() - 1 {
                adjacent_part_numbers.append(&mut process_line(lines[line_index + 1], symbol_index));
            }

            // It is a gear if there are exactly 2 adjacent part numbers
            if adjacent_part_numbers.len() == 2 {
                sum_gear_ratios += adjacent_part_numbers[0] * adjacent_part_numbers[1];            
            }                     
    
            star_symbol_index += 1;
        }								
        
        line_index += 1;
    }
    println!("Sum of gear ratios: {}", sum_gear_ratios);
 }
