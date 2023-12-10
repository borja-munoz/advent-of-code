use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let times = lines[0].split(":")
                .collect::<Vec<&str>>()[1]
                .split_whitespace().collect::<Vec<&str>>()
                .iter().map(|t| t.trim_start().trim_end().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

    let record_distances = lines[1].split(":")
                           .collect::<Vec<&str>>()[1]
                           .split_whitespace().collect::<Vec<&str>>()
                           .iter().map(|t| t.trim_start().trim_end().parse::<u32>().unwrap())
                           .collect::<Vec<u32>>();

    println!("Times: {:?}", times);
    println!("Record Distances: {:?}", record_distances);

    let mut beat_options: Vec<u32> = vec![0; times.len().try_into().unwrap()];
    let mut index_race = 0;
    while index_race < times.len() {
        let race_time = times[index_race];
        let race_record = record_distances[index_race];

        for hold_button_ms in 1..race_time - 1 {
            let distance = (race_time - hold_button_ms) * hold_button_ms;
            if distance > race_record {
                beat_options[index_race] += 1;
            }
        }

        index_race += 1;
    }

    println!("Beat options multiplication: {:?}", beat_options.iter().product::<u32>());
 }
