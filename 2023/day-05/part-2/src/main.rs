use indicatif::ProgressIterator;
use std::fs;

struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

// Reads a map until a blank line is found or 
// there are no more lines
fn read_map(lines: &Vec<&str>, line_index: &mut usize) -> Vec<Range> {
    let mut blank_line = false;
    let mut range_map: Vec<Range> = vec![];
    while !blank_line && *line_index < lines.len() {
        if lines[*line_index].is_empty() {
            blank_line = true;
        }
        else {
            let map: Vec<u64> = lines[*line_index].split(" ")
                                .into_iter()
                                .map(|n| n.parse::<u64>().unwrap()).collect();
            range_map.push(Range {
                destination: map[0],
                source: map[1],
                length: map[2],
            })
        }
        *line_index += 1;
    }

    return range_map;
}

// Get a destinatio value from a source value according to the range map
fn get_mapped_value(source: u64, range_map: &Vec<Range>) -> u64 {
    let destination = source;
    for range in range_map {
        if source >= range.source &&
           source < range.source + range.length {
            return range.destination + source - range.source;
        }
    }
    return destination;
}

fn main() {
    let mut lowest_location_number = u64::MAX;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let mut line_index = 0;

    // Read seeds
    let mut seeds: Vec<u64> = lines[line_index].split(":")
                              .collect::<Vec<_>>()[1]
                              .trim_start()
                              .split(" ")
                              .into_iter()
                              .map(|n| n.parse::<u64>().unwrap()).collect();
    let mut seed_index = 0;
    let seeds_clone = seeds.clone();
    let mut seed_position = 1;
    println!("Creating seeds...");
    while seed_index < seeds_clone.len() {
        let seed_start = seeds_clone[seed_index] + 1;
        let seed_range = seeds_clone[seed_index + 1];
        println!("Seed Start: {}, Range Length: {}", seed_start, seed_range);
        for seed in seed_start..seed_start + seed_range - 1 {
            seeds.insert(seed_position, seed);
            seed_position += 1;
        }
        seeds.remove(seed_position);
        seed_position += 1;
        seed_index += 2;
    }
    // println!("{:?}", seeds);

    // Read seed to soil map
    line_index += 3; // Skip blank line and title line
    let seed_soil_map = read_map(&lines, &mut line_index);

    // Read soil to fertilizer map
    line_index += 1; // Skip title line
    let soil_fertilizer_map = read_map(&lines, &mut line_index);
    
    // Read fertilizer to water map
    line_index += 1; // Skip title line
    let fertilizer_water_map = read_map(&lines, &mut line_index);
    
    // Read water to light map
    line_index += 1; // Skip title line
    let water_light_map = read_map(&lines, &mut line_index);
    
    // Read light to temperature map
    line_index += 1; // Skip title line
    let light_temperature_map = read_map(&lines, &mut line_index);
    
    // Read temperature to humidity map
    line_index += 1; // Skip title line
    let temperature_humidity_map = read_map(&lines, &mut line_index);
    
    // Read humidity to location map
    line_index += 1; // Skip title line
    let humidity_location_map = read_map(&lines, &mut line_index);
    
    println!("Mapping seeds to locations...");
    for seed in seeds.into_iter().progress() {
        let soil = get_mapped_value(seed, &seed_soil_map);
        let fertilizer = get_mapped_value(soil, &soil_fertilizer_map);
        let water = get_mapped_value(fertilizer, &fertilizer_water_map);
        let light = get_mapped_value(water, &water_light_map);
        let temperature = get_mapped_value(light, &light_temperature_map);
        let humidity = get_mapped_value(temperature, &temperature_humidity_map);
        let location = get_mapped_value(humidity, &humidity_location_map);

        if location < lowest_location_number {
            lowest_location_number = location;
        }
    }

    println!("Lowest location number: {}", lowest_location_number);
 }
