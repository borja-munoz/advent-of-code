use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let time = lines[0].split(":")
               .collect::<Vec<&str>>()[1]
               .split_whitespace().collect::<Vec<&str>>()
               .join("").parse::<u64>().unwrap();

    let record_distance = lines[1].split(":")
                          .collect::<Vec<&str>>()[1]
                          .split_whitespace().collect::<Vec<&str>>()
                          .join("").parse::<u64>().unwrap();
    
    println!("Time: {:?}", time);
    println!("Record Distance: {:?}", record_distance);

    let mut beat_options = 0;

    for hold_button_ms in 1..time - 1 {
        let distance = (time - hold_button_ms) * hold_button_ms;
        if distance > record_distance {
            beat_options += 1;
        }
        else if beat_options > 0 {
            break;
        }
    }

    println!("Beat options: {}", beat_options);
 }
