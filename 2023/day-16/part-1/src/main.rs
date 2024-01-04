use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Tile {
    EmptySpace,
    LeftUpMirror,
    RightDownMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Unknown,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::EmptySpace,
            '/' => Tile::LeftUpMirror,
            '\\' => Tile::RightDownMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => Tile::Unknown,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Hash)]
enum BeamingDirection {
    Left,
    Right,
    Up,
    Down,
}

// Code from https://doc.rust-lang.org/std/hash/index.html
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn select_beaming_direction(
    tile: &Tile, 
    current_beaming_direction: BeamingDirection
) -> Vec<BeamingDirection> {
    let mut new_beaming_directions = vec![];
    match tile {
        Tile::EmptySpace => {
            // Continue in the same direction
            new_beaming_directions.push(current_beaming_direction.clone());
        },
        Tile::LeftUpMirror => {
            // Turn 90º
            new_beaming_directions.push(match current_beaming_direction {
                BeamingDirection::Right => BeamingDirection::Up,
                BeamingDirection::Down => BeamingDirection::Left,
                BeamingDirection::Left => BeamingDirection::Down,
                BeamingDirection::Up => BeamingDirection::Right,
            })
        },
        Tile::RightDownMirror => {
            // Turn 90º
            new_beaming_directions.push(match current_beaming_direction {
                BeamingDirection::Right => BeamingDirection::Down,
                BeamingDirection::Down => BeamingDirection::Right,
                BeamingDirection::Left => BeamingDirection::Up,
                BeamingDirection::Up => BeamingDirection::Left,
            })
        },
        Tile::VerticalSplitter => {
            // Check if we should continue in the same direction
            // or split into two beams
            if current_beaming_direction == BeamingDirection::Left ||
               current_beaming_direction == BeamingDirection::Right {
                new_beaming_directions.push(BeamingDirection::Up);
                new_beaming_directions.push(BeamingDirection::Down);
            } else {
                new_beaming_directions.push(current_beaming_direction.clone());                
            }
        },
        Tile::HorizontalSplitter => {
            // Check if we should continue in the same direction
            // or split into two beams
            if current_beaming_direction == BeamingDirection::Up ||
               current_beaming_direction == BeamingDirection::Down {
                new_beaming_directions.push(BeamingDirection::Right);
                new_beaming_directions.push(BeamingDirection::Left);
            } else {
                new_beaming_directions.push(current_beaming_direction.clone());                
            }
        },
        _ => {
            println!("Wrong tile found");
        },
    };

    return new_beaming_directions;
}

fn trace_light_beam(
    contraption: &Vec<Vec<Tile>>, 
    energized_tiles: &mut Vec<Vec<bool>>,
    initial_position: (usize, usize),
    initial_beaming_direction: BeamingDirection,
    generated_beams: &mut usize,
    visited_tiles_direction: &mut HashMap<u64, bool>
) {
    let mut beaming_direction = initial_beaming_direction;
    let mut current_position = initial_position;
    let mut wall_found = false;
    let mut tile_direction_visited = false;
    
    while !wall_found && !tile_direction_visited {

        // Energize the tile in the current position
        energized_tiles[current_position.0][current_position.1] = true;

        // If we already traced the light beam from the initial position 
        // in the initial beaming direction, we stop
        let hash_current_conditions = calculate_hash(
            &(current_position.0, 
              current_position.1, 
              beaming_direction.clone())
        );
        if visited_tiles_direction.get(&hash_current_conditions) == None {

            visited_tiles_direction.insert(hash_current_conditions, true);

            let new_beaming_directions = select_beaming_direction(
                &contraption[current_position.0][current_position.1],
                beaming_direction
            );

            beaming_direction = new_beaming_directions[0].clone();

            // If two directions are returned because there is a
            // splitter, we create a new beam
            if new_beaming_directions.len() == 2 {
                // println!("Creating new beam");
                trace_light_beam(
                    contraption, 
                    energized_tiles, 
                    current_position,
                    new_beaming_directions[1].clone(),
                    generated_beams,
                    visited_tiles_direction
                );
            }
            
            // Check if the next movement is possible
            if (current_position.1 == 0 && beaming_direction == BeamingDirection::Left) ||
               (current_position.1 == contraption[0].len() - 1 && beaming_direction == BeamingDirection::Right) ||
               (current_position.0 == 0 && beaming_direction == BeamingDirection::Up) ||
               (current_position.0 == contraption.len() - 1 && beaming_direction == BeamingDirection::Down) {
                wall_found = true;
                // println!("Wall reached");
            } else {
                (current_position, beaming_direction) = select_next_tile(
                    current_position, 
                    beaming_direction
                );

                // println!(
                //     "Position: {:?}, Beaming Direction: {:?}, Next Tile: {:?}",
                //     current_position, 
                //     beaming_direction, 
                //     contraption[current_position.0][current_position.1]
                // );
            }                        
        } else {
            tile_direction_visited = true;
        }
    }

    // Energize the tile in the last position
    energized_tiles[current_position.0][current_position.1] = true;
}

fn select_next_tile(
    current_position: (usize, usize),
    beaming_direction: BeamingDirection,
) -> ((usize, usize), BeamingDirection) {
    let new_position;

    match beaming_direction {
        BeamingDirection::Right => {
            new_position = (current_position.0, current_position.1 + 1);
        },
        BeamingDirection::Left => {
            new_position = (current_position.0, current_position.1 - 1);
        },
        BeamingDirection::Up => {
            new_position = (current_position.0 - 1, current_position.1);
        },
        BeamingDirection::Down => {
            new_position = (current_position.0 + 1, current_position.1);
        },
    }

    return (new_position, beaming_direction);
}

fn main() {
    let mut total_energized_tiles = 0;

    let mut contraption: Vec<Vec<Tile>> = vec![];
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        contraption.push(line.chars().map(|c| c.into()).collect());
    }

    // Initialize the matrix that will store the energized tiles
    let mut energized_tiles: Vec<Vec<bool>> = vec![];
    for _ in 0..contraption.len() {
        energized_tiles.push(vec![false; contraption[0].len()]);
    }
    
    let mut visited_tiles_direction: HashMap<u64, bool> = HashMap::new();
    trace_light_beam(
        &contraption, 
        &mut energized_tiles, 
        (0, 0),
        BeamingDirection::Right,
        &mut 1,
        &mut visited_tiles_direction
    );

    // println!("Energized tiles: {:?}", energized_tiles);

    // Count energized tiles
    for row in &energized_tiles {
        total_energized_tiles += row.iter().filter(|&&x| x).collect::<Vec<_>>().len();
    }
    
    println!("Energized tiles: {}", total_energized_tiles);
}
