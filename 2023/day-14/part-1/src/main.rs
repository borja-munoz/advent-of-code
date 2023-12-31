use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Platform {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
    Unknown,
}

impl From<char> for Platform {
    fn from(c: char) -> Self {
        match c {
            '.' => Platform::EmptySpace,
            'O' => Platform::RoundedRock,
            '#' => Platform::CubeShapedRock,
            _ => Platform::Unknown,
        }
    }
}

fn calculate_column_load(column: &Vec<Platform>) -> usize {
    let mut first_empty_space = usize::MAX;
    let mut load = 0;

    let mut i = 0;
    while i < column.len() {
        match column[i] {
            Platform::EmptySpace => {
                if first_empty_space == usize::MAX {
                    first_empty_space = i;
                }
            },
            Platform::RoundedRock => {
                if first_empty_space != usize::MAX {
                    load += column.len() - first_empty_space;
                    first_empty_space = i - (i - first_empty_space - 1);
                } else {
                    load += column.len() - i;
                }
            },
            Platform::CubeShapedRock => {
                first_empty_space = usize::MAX;
            },
            _ => println!("Unknown element found"),
        }
        i += 1;
    }

    return load;
}

fn main() {
    let mut total_load = 0;

    let mut platform: Vec<Vec<Platform>> = vec![];
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        platform.push(line.chars().map(|c| c.into()).collect());
    }

    let mut i = 0;
    while i < platform[0].len() {
        let mut column: Vec<Platform> = vec![];
        let mut j = 0;
        while j < platform.len() {
            column.push(platform[j][i].clone());
            j += 1;
        }
        total_load += calculate_column_load(&column);
        // println!("Total load after column {:?}: {}", column, total_load);
        i += 1;
    }

    println!("Total load: {}", total_load);
}
