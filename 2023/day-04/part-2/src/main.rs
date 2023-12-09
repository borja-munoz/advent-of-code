use std::fs;

fn get_numbers_found(line: &str) -> u32 {
    let card_numbers = line.split(":")
    .collect::<Vec<_>>()[1].split("|")
    .collect::<Vec<_>>();
    let winning_numbers: Vec<_> = card_numbers[0].trim_start().trim_end()
                        .split(" ").collect::<Vec<_>>()
                        .iter().map(|n| n.trim()).collect();
    let own_numbers: Vec<_> = card_numbers[1].trim_start().trim_end()
                    .split(" ").collect::<Vec<_>>()
                    .iter().map(|n| n.trim()).collect();
    let mut numbers_found = 0;
    for number in winning_numbers {
        if !number.is_empty() {
            if own_numbers.iter().position(|&x| x == number) != None {
                numbers_found += 1;
            }        
        }
    }
    return numbers_found;
}

fn main() {
    let mut numbers_found: Vec<u32> = vec![];
    let mut total_scratchcards = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        numbers_found.push(get_numbers_found(line));
    }

    let mut card_copies: Vec<u32> = vec![0; numbers_found.len()];
    let mut card_index = 0;
    while card_index < numbers_found.len() {
        card_copies[card_index] += 1;
        let numbers_found = numbers_found[card_index];
        for card in card_index + 1..card_index + 1 + (numbers_found as usize) {
            card_copies[card] += card_copies[card_index];
        }
        card_index += 1;
    }

    for cards in card_copies {
        total_scratchcards += cards;        
    }

    println!("Total scratchcards: {}", total_scratchcards);
 }
