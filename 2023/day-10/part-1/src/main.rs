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
        if left_char != '.' {
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
        if right_char != '.' {
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
        if top_char != '.' {
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
        if bottom_char != '.' {
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

fn main() {
    let steps_to_farthest_point;

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let mut index_line = 0;
    let mut char_matrix: Vec<Vec<char>> = vec![];
    while index_line < lines.len() {
        char_matrix.push(lines[index_line].chars().into_iter().collect());
        index_line += 1;
    }

    let starting_point = find_starting_point(&char_matrix);
    println!("Starting point: {:?}", starting_point);

    let (mut previous_movement, next_point) = make_first_movement(&char_matrix, 
                                                              &starting_point);
    let mut next_char = get_char(&char_matrix, &next_point);
    let mut current_point = next_point.clone();
    let mut steps_starting_point = 1;
    while next_char != 'S' {
        let (next_movement, next_point) = select_next_point(&char_matrix, &previous_movement, &current_point);
        next_char = get_char(&char_matrix, &next_point);                             
        previous_movement = next_movement;
        println!("Next char: {}", next_char);
        current_point = next_point.clone();
        steps_starting_point += 1;
    }
    steps_to_farthest_point = steps_starting_point / 2;

    println!("Steps to farthest point: {}", steps_to_farthest_point);
 }
