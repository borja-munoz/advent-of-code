use std::fs;

fn main() {
    let mut sum_card_points = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
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
        let mut card_points = 0;
        if numbers_found > 0 {
            if numbers_found == 1 {
                card_points = 1;
            } 
            else {
                card_points = i32::pow(2, numbers_found - 1);
            }
        }
        sum_card_points += card_points;
    }

    println!("Sum of all card points: {}", sum_card_points);
 }
