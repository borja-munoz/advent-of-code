use regex::{Match, Regex, RegexSet};
use std::fs;

fn get_digit(my_string: &str) -> char {
    let str_digits: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    if my_string.len() == 1 {
        return my_string.chars().nth(0).unwrap();
    }
    else {
        let index = str_digits.iter().position(|&r| r == my_string).unwrap();
        return char::from_digit((index + 1).try_into().unwrap(), 10).unwrap();
    }
}

fn main() {
    let patterns = [
        r"[0-9]",
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ];    
    
    // Compile a set matching any of our patterns.
    let set = RegexSet::new(patterns).unwrap();

    // Compile each pattern independently.
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    
    let mut calibration_value = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {

        // Matching with a single regex does not work with overlapping expressions
        // so we need first to compare with a RegexSet in order to get 
        // all the patterns that are present in the line and then get
        // the individual byte offsets to get the first and last digits
        let mut first_offset = usize::MAX;
        let mut last_offset = usize::MIN;
        let mut first_digit = '0';
        let mut last_digit = '0';
        let matches = set.matches(line);
        for match_index in matches {
            // println!("Regex matched: {}", regexes[match_index]);
            let digit_matches: Vec<Match> = regexes[match_index].find_iter(line).map(|n| n).collect();
            if digit_matches[0].start() < first_offset {
                first_offset = digit_matches[0].start();
                first_digit = get_digit(digit_matches[0].as_str());
            }                
            if digit_matches[digit_matches.len() - 1].start() >= last_offset {
                last_offset = digit_matches[digit_matches.len() - 1].start();            
                last_digit = get_digit(digit_matches[digit_matches.len() - 1].as_str());
            }                
        }

        let mut my_number = first_digit.to_string();
        my_number.push(last_digit);
        calibration_value += my_number.parse::<i32>().unwrap();
        println!("{}: {}, {}", line, my_number, calibration_value);
    }
    println!("Calibration value: {}", calibration_value);
}
