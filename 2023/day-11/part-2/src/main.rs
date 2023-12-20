use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

fn get_empty_rows(universe: Vec<&str>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = vec![];

    for (pos, row) in universe.iter().enumerate() {
        if row.find('#') == None {
            empty_rows.push(pos);
        }
    } 

    return empty_rows;
}

fn get_empty_columns(universe: Vec<&str>) -> Vec<usize> {
    let mut empty_columns: Vec<usize> = vec![];

    for column in 0..universe[0].len() {
        let mut galaxy_found = false;
        for row in universe.iter() {
            if row.chars().nth(column).unwrap() == '#' {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            empty_columns.push(column);
        }
    }
    return empty_columns;
}

fn get_galaxy_locations(universe: Vec<&str>) -> Vec<Coordinate> {
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

fn get_shortest_path(location_a: &Coordinate, 
                     location_b: &Coordinate,
                     empty_rows: &Vec<usize>,
                     empty_columns: &Vec<usize>,
                     factor: usize) -> usize {
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

    // Calculate if there are empty rows between the locations
    let mut empty_rows_in_between = 0;
    let range = if location_a.y < location_b.y { 
        location_a.y..location_b.y
    } else { 
        location_b.y..location_a.y
    };
    for row in empty_rows {
        if range.contains(&row) {
            empty_rows_in_between += 1;
        }
    }

    // Calculate if there are empty columns between the locations
    let mut empty_columns_in_between = 0;
    let range = if location_a.x < location_b.x { 
        location_a.x..location_b.x
    } else { 
        location_b.x..location_a.x 
    };
    for column in empty_columns {
        if range.contains(&column) {
            empty_columns_in_between += 1;
        }
    }
    
    // println!("Location A: {:?}, Location B: {:?}", location_a, location_b);
    // println!("Empty rows: {}, Empty columns: {}", empty_rows_in_between, empty_columns_in_between);
    return diff_x + empty_rows_in_between * (factor - 1) +
           diff_y + empty_columns_in_between * (factor - 1);
}

fn main() {
    let mut sum_shortest_paths_lengths = 0;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let galaxy_locations = get_galaxy_locations(lines.clone());

    let empty_rows = get_empty_rows(lines.clone());
    let empty_columns = get_empty_columns(lines.clone());

    // println!("Empty rows: {:?}", empty_rows);
    // println!("Empty columns: {:?}", empty_columns);

    for i in 0..galaxy_locations.len() - 1 {
        for j in i+1..galaxy_locations.len() {
            let shortest_path = get_shortest_path(&galaxy_locations[i], 
                                                  &galaxy_locations[j],
                                                  &empty_rows,
                                                  &empty_columns,
                                                  1000000);
            // println!("Shortest path between {} and {}: {}", i, j, shortest_path);
            sum_shortest_paths_lengths += shortest_path;
        }
    }

    println!("Sum of shortest paths lengths: {}", sum_shortest_paths_lengths);
 }
