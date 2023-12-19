use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Movement {
    Left,
    Right,
    Top,
    Bottom,
    NoMovement
}

fn get_char(char_matrix: &Vec<Vec<char>>, coordinate: &Coordinate) -> char {
    return char_matrix[coordinate.y][coordinate.x];
}

fn find_starting_point(char_matrix: &Vec<Vec<char>>) -> Coordinate {
    let mut index_row = 0;
    let mut index_char = 0;
    while index_row < char_matrix.len() {
        let position = char_matrix[index_row].iter().position(|c| *c == 'S');
        if  position != None {
            index_char = position.unwrap();
            break;
        }
        index_row += 1;
    }
    return Coordinate {
        x: index_char,
        y: index_row,
    };
}

fn make_first_movement(char_matrix: &Vec<Vec<char>>, 
                       current_point: &Coordinate) -> (Movement, Coordinate) {

    // Only for the 'S' all the movements (to/from) are possible
    if current_point.x > 0 {
        let left_char = get_char(&char_matrix, 
                                &Coordinate { 
                                    x: current_point.x - 1, 
                                    y: current_point.y 
                                });
        if left_char == '-' || left_char == 'L' || left_char == 'F' {
            return (Movement::Left, 
                    Coordinate { x: current_point.x - 1, y: current_point.y });
        }
    }

    // Check if there is an 'S' on the right
    if current_point.x < char_matrix[current_point.y].len() - 1 {
        let right_char = get_char(&char_matrix, 
                                &Coordinate { 
                                    x: current_point.x + 1, 
                                    y: current_point.y 
                                });
        if right_char == '-' || right_char == 'J' || right_char == '7' {
            return (Movement::Right,
                    Coordinate { x: current_point.x + 1, y: current_point.y });
        }
    }

    // Check if there is an 'S' on the top
    if current_point.y > 0 {
        let top_char = get_char(&char_matrix,  
                                &Coordinate { 
                                    x: current_point.x, 
                                    y: current_point.y - 1 
                                });
        if top_char == '|' || top_char == '7' || top_char == 'F' {
            return (Movement::Top,
                    Coordinate { x: current_point.x, y: current_point.y - 1 });
        }
    }

    // Check if there is an 'S' on the bottom
    if current_point.y < char_matrix.len() - 1 {
        let bottom_char = get_char(&char_matrix,  
                                &Coordinate { 
                                    x: current_point.x, 
                                    y: current_point.y + 1 
                                });
        if bottom_char == '|' || bottom_char == 'L' || bottom_char == 'J' {
            return (Movement::Bottom,
                    Coordinate { x: current_point.x, y: current_point.y + 1 });
        }
    }
    return (Movement::NoMovement, Coordinate { x: usize::MAX, y: usize::MAX });
}        

fn select_next_point(char_matrix: &Vec<Vec<char>>, 
                     previous_movement: &Movement, 
                     current_point: &Coordinate) -> (Movement, Coordinate) {

    let current_char = get_char(&char_matrix, 
        &Coordinate { 
            x: current_point.x, 
            y: current_point.y 
        });

    // For all chars except 'S', only one movement is possible
    // and depends on the previous movement.
    // If you are at an 'L' and the previous movement has been
    // from the top char, then you need to move to the right char
    let next_movement = match (previous_movement, current_char) {
        (Movement::Top, '|') => Movement::Top,
        (Movement::Bottom, '|') => Movement::Bottom,
        (Movement::Left, '-') => Movement::Left,
        (Movement::Right, '-') => Movement::Right,
        (Movement::Bottom, 'L') => Movement::Right,
        (Movement::Left, 'L') => Movement::Top,
        (Movement::Bottom, 'J') => Movement::Left,
        (Movement::Right, 'J') => Movement::Top,
        (Movement::Top, '7') => Movement::Left,
        (Movement::Right, '7') => Movement::Bottom,
        (Movement::Top, 'F') => Movement::Right,
        (Movement::Left, 'F') => Movement::Bottom,
        _ => Movement::NoMovement,
    };

    println!("Next movement: {:?}", next_movement);
    let next_point = match next_movement {
        Movement::Left => Coordinate { 
                              x: current_point.x - 1,
                              y: current_point.y,
                          },
        Movement::Right => Coordinate { 
                               x: current_point.x + 1,
                               y: current_point.y,
                           },
        Movement::Top => Coordinate { 
                             x: current_point.x,
                             y: current_point.y - 1,
                         },
        Movement::Bottom => Coordinate { 
                                x: current_point.x,
                                y: current_point.y + 1,
                            },
        _ => Coordinate { x: usize::MAX, y: usize::MAX },
    };

    if next_point.x == usize::MAX {
        println!("No possible movement. Current point coordinates: {:?}", current_point);
    }
    return (next_movement, next_point);
}

fn count_enclosed_tiles(char_matrix: &Vec<Vec<char>>,
                        tile_visited: Vec<Vec<bool>>) -> usize {

    // We are going to check if tiles are inside or outside
    // the loop using the even-odd rule:
    // https://en.wikipedia.org/wiki/Even–odd_rule

    // For each not visited tile, we will follow the "ray"
    // to the bottom of the diagram. If it crosses an odd
    // number of loop tiles, then the tile is inside the loop.

    // If we cross a '-' char, we count 1
    // If we cross a '|' char, we continue
    // If we cross a 90º char (L, J, 7, F), we need to continue
    // until the next char to see if it is a cross or not

    let mut enclosed_tiles = 0;

    for (index_row, line) in tile_visited.iter().enumerate() {
        for (index_column, loop_tile) in line.iter().enumerate() {
            if !loop_tile {
                let mut crosses = 0;
                let mut prev = '.';
                for index_row_below in index_row..tile_visited.len() {
                    if tile_visited[index_row_below][index_column] {
                        let char_below = get_char(&char_matrix, 
                                                  &Coordinate {
                                                      x: index_column,
                                                      y: index_row_below,
                                                  });
                        if char_below == '-' {
                            crosses += 1;
                        } else if char_below == '7' || char_below == 'F' {
                            prev = char_below;
                        } else if (char_below == 'J' && prev == 'F') || 
                                  (char_below == 'L' && prev == '7') {
                            crosses += 1;
                        }
                    }
                }
                if crosses % 2 == 1 {
                    enclosed_tiles += 1;
                }    
            }
        }
    }

    return enclosed_tiles;
}

fn main() {
    let enclosed_tiles;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let mut index_line = 0;
    let mut char_matrix: Vec<Vec<char>> = vec![];
    let mut tile_visited: Vec<Vec<bool>> = vec![];
    while index_line < lines.len() {
        char_matrix.push(lines[index_line].chars().into_iter().collect());
        tile_visited.push(vec![false; char_matrix[index_line].len()]);
        index_line += 1;
    }

    let starting_point = find_starting_point(&char_matrix);
    println!("Starting point: {:?}", starting_point);
    tile_visited[starting_point.y][starting_point.x] = true;

    let (mut previous_movement, next_point) = make_first_movement(&char_matrix, 
                                                              &starting_point);
    let mut next_char = get_char(&char_matrix, &next_point);
    tile_visited[next_point.y][next_point.x] = true;
    let mut current_point = next_point.clone();
    while next_char != 'S' {
        let (next_movement, next_point) = select_next_point(&char_matrix, &previous_movement, &current_point);
        next_char = get_char(&char_matrix, &next_point);                             
        tile_visited[next_point.y][next_point.x] = true;
        previous_movement = next_movement;
        println!("Next char: {}", next_char);
        current_point = next_point.clone();
    }
    enclosed_tiles = count_enclosed_tiles(&char_matrix, tile_visited);

    println!("Enclosed tiles: {}", enclosed_tiles);
 }
