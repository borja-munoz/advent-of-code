use std::fs;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    let mut sum_game_ids = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let game_id = line.split(":")
            .collect::<Vec<_>>()[0].split(" ")
            .collect::<Vec<_>>()[1];
        let mut game_possible = true;
        let cube_sets = line.split(":")
            .collect::<Vec<_>>()[1].split(";");
        for cube_set in cube_sets {
            let color_cubes = cube_set.split(",").collect::<Vec<_>>();
            for color_cube in color_cubes {
                let color_cube_split = color_cube.trim_start().split(" ").collect::<Vec<_>>();
                let cubes = color_cube_split[0].parse::<i32>().unwrap();               
                let color = color_cube_split[1];
                match color {
                    "red" => if cubes > MAX_RED_CUBES { 
                        game_possible = false; 
                        break;
                    },
                    "green" => if cubes > MAX_GREEN_CUBES { 
                        game_possible = false;
                        break;
                    },
                    "blue" => if cubes > MAX_BLUE_CUBES { 
                        game_possible = false;
                        break;
                    },
                    _ => println!("Wrong color found: {}", color),
                }
            }
        }
        if game_possible {
            sum_game_ids += game_id.parse::<i32>().unwrap();
        }
    }
    println!("Sum of the IDs: {}", sum_game_ids);
}
