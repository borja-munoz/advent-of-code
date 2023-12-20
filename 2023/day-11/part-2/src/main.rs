use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

fn expand_universe(universe: Vec<&str>, factor: usize) -> Vec<String> {
    let mut row_expanded_universe: Vec<&str> = vec![];
    let mut expanded_universe: Vec<String> = vec![];

    for row in universe {
        row_expanded_universe.push(row);
        expanded_universe.push(row.to_string());
        let galaxy_found = row.find('#');
        if galaxy_found == None {
            for _ in 0..factor - 1 {
                row_expanded_universe.push(row);
                expanded_universe.push(row.to_string());
            }
        }
    }

    let mut empty_columns = 0;
    for column in 0..row_expanded_universe[0].len() {
        let mut galaxy_found = false;
        for row in &row_expanded_universe {
            if row.chars().nth(column).unwrap() == '#' {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            // println!("No galaxy found for column {}, expanding...", column);
            let mut index_row = 0;
            while index_row < row_expanded_universe.len() {
                for _ in 0..factor - 1 {
                    expanded_universe[index_row].insert(column + empty_columns, '.');
                }
                index_row += 1;
            }
            empty_columns += factor - 1;
        }
    }

    // for row in &expanded_universe {
    //     println!("{}", row);
    // }

    return expanded_universe;
}

fn get_galaxy_locations(universe: Vec<String>) -> Vec<Coordinate> {
    let mut galaxy_locations: Vec<Coordinate> = vec![];

    let mut index_row = 0;
    while index_row < universe.len() {
        let mut index_char = 0;
        for c in universe[index_row].chars() {
            if c == '#' {
                galaxy_locations.push(
                    Coordinate {
                        x: index_char,
                        y: index_row,
                    }
                );
            }
            index_char += 1;
        }
        index_row += 1;
    }

    return galaxy_locations;
}

fn get_shortest_path(location_a: &Coordinate, location_b: &Coordinate) -> usize {
    let diff_x = if location_a.x > location_b.x {
        location_a.x - location_b.x
    } else {
        location_b.x - location_a.x
    };
    let diff_y = if location_a.y > location_b.y {
        location_a.y - location_b.y
    } else {
        location_b.y - location_a.y
    };
    return diff_x + diff_y;
}

fn main() {
    let mut sum_shortest_paths_lengths = 0;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let expanded_universe = expand_universe(lines, 1000000);
    let galaxy_locations = get_galaxy_locations(expanded_universe);
    for i in 0..galaxy_locations.len() - 1 {
        for j in i+1..galaxy_locations.len() {
            let shortest_path = get_shortest_path(&galaxy_locations[i], 
                                                  &galaxy_locations[j]);
            // println!("Shortest path between {} and {}: {}", i, j, shortest_path);
            sum_shortest_paths_lengths += shortest_path;
        }
    }

    println!("Sum of shortest paths lengths: {}", sum_shortest_paths_lengths);
 }
