use std::fs;

fn main() {
    let mut sum_cubes_power = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let game_id = line.split(":")
            .collect::<Vec<_>>()[0].split(" ")
            .collect::<Vec<_>>()[1];
        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;
        let cube_sets = line.split(":")
            .collect::<Vec<_>>()[1].split(";");
        for cube_set in cube_sets {
            let color_cubes = cube_set.split(",").collect::<Vec<_>>();
            for color_cube in color_cubes {
                let color_cube_split = color_cube.trim_start().split(" ").collect::<Vec<_>>();
                let cubes = color_cube_split[0].parse::<i32>().unwrap();               
                let color = color_cube_split[1];
                match color {
                    "red" => if cubes > max_red_cubes { 
                        max_red_cubes = cubes; 
                    },
                    "green" => if cubes > max_green_cubes { 
                        max_green_cubes = cubes;
                    },
                    "blue" => if cubes > max_blue_cubes { 
                        max_blue_cubes = cubes;
                    },
                    _ => println!("Wrong color found: {}", color),
                }
            }
        }
        sum_cubes_power += max_red_cubes * max_green_cubes * max_blue_cubes;
    }
    println!("Sum of the power of cubes: {}", sum_cubes_power);
}
