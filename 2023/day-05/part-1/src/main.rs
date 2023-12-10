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

    // Initialize collections
    let seeds: Vec<u64>;
    let seed_soil_map: Vec<Range>;
    let soil_fertilizer_map: Vec<Range>;
    let fertilizer_water_map: Vec<Range>;
    let water_light_map: Vec<Range>;
    let light_temperature_map: Vec<Range>;
    let temperature_humidity_map: Vec<Range>;
    let humidity_location_map: Vec<Range>;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let mut line_index = 0;

    // Read seeds
    seeds = lines[line_index].split(":")
            .collect::<Vec<_>>()[1]
            .trim_start()
            .split(" ")
            .into_iter()
            .map(|n| n.parse::<u64>().unwrap()).collect();
    println!("{:?}", seeds);

    // Read seed to soil map
    line_index += 3; // Skip blank line and title line
    seed_soil_map = read_map(&lines, &mut line_index);

    // Read soil to fertilizer map
    line_index += 1; // Skip title line
    soil_fertilizer_map = read_map(&lines, &mut line_index);
    
    // Read fertilizer to water map
    line_index += 1; // Skip title line
    fertilizer_water_map = read_map(&lines, &mut line_index);
    
    // Read water to light map
    line_index += 1; // Skip title line
    water_light_map = read_map(&lines, &mut line_index);
    
    // Read light to temperature map
    line_index += 1; // Skip title line
    light_temperature_map = read_map(&lines, &mut line_index);
    
    // Read temperature to humidity map
    line_index += 1; // Skip title line
    temperature_humidity_map = read_map(&lines, &mut line_index);
    
    // Read humidity to location map
    line_index += 1; // Skip title line
    humidity_location_map = read_map(&lines, &mut line_index);
    
    for seed in seeds {
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
