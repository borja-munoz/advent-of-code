use regex::{Match, Regex};
use std::fs;

fn main() {
    let mut sum_part_numbers = 0;

    // We read the lines from the file into a vector
    let mut lines: Vec<&str> = vec![];
    let binding = fs::read_to_string("input.txt").unwrap();
    for line in binding.lines() {
        lines.push(line);
    }

    // Now we search the part numbers next to a symbol
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut line_index = 0;
    while line_index < lines.len() {
        let line = lines[line_index];
        let number_matches: Vec<Match> = re.find_iter(line).map(|m| m).collect();
        // For each part number, we look in the same, previous and next line
        // to see if there is an adjacent symbol
        let mut number_index = 0;
        while number_index < number_matches.len() {
            let mut is_part_number = false;
            let start_offset = number_matches[number_index].start();
            let end_offset = number_matches[number_index].end();

            // Look in the same line
            if start_offset > 0 {
                if line.chars().nth(start_offset - 1) != Some('.') {
                    is_part_number = true;
                }
            }
            if end_offset < line.len() {
                // println!("Next char: {}", line.chars().nth(end_offset + 1).unwrap().to_string());
                if line.chars().nth(end_offset) != Some('.') {
                    is_part_number = true;
                }
            }

            // Find start and end indexes to look between
            let mut char_start_index = start_offset;
            let mut char_end_index = end_offset;
            if start_offset > 0 {
                char_start_index -= 1;
            }
            if end_offset >= line.len() {
                char_end_index -= 1;
            }

            // Look in the previous line
            if line_index > 0 {
                let previous_line = lines[line_index - 1];
                let mut char_index = char_start_index;
                while char_index <= char_end_index {
                    if previous_line.chars().nth(char_index) != Some('.') {
                        is_part_number = true;
                    }
                    char_index += 1;
                }
            }

            // Look in the next line
            if line_index < lines.len() - 1 {
                let next_line = lines[line_index + 1];
                let mut char_index = char_start_index;
                while char_index <= char_end_index {
                    if next_line.chars().nth(char_index) != Some('.') {
                        is_part_number = true;
                    }
                    char_index += 1;
                }
            }

            if is_part_number {
                sum_part_numbers += number_matches[number_index].as_str().parse::<i32>().unwrap();            
            }                     
    
            // println!("{}, {}, {}", line, number_matches[number_index].as_str(), is_part_number);

            number_index += 1;
        }								
        
        line_index += 1;
    }
    println!("Sum of all part numbers: {}", sum_part_numbers);
 }
