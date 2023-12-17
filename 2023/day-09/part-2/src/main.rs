use std::fs;

use itertools::Itertools;

fn main() {
    let mut sum_extrapolated_values = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let history: Vec<i64> = line.split(" ")
                                .into_iter()
                                .map(|v| v.parse::<i64>().unwrap())
                                .collect();
        let mut diff_vectors: Vec<Vec<i64>> = vec![];
        diff_vectors.push(history.clone());
        let mut last_elem_zero = false;
        while !last_elem_zero {
            let mut new_diff_vector = vec![];
            for (_idx, (&elem, &next)) in diff_vectors[diff_vectors.len() - 1]
                                         .iter()
                                         .tuple_windows()
                                         .enumerate() {
                new_diff_vector.push(next - elem);
            }    
            diff_vectors.push(new_diff_vector.clone());
            if *new_diff_vector.last().unwrap() == 0 {
                last_elem_zero = true;
            }
        }                    
        let mut extrapolated_value = 0;
        for diff_vector in diff_vectors.iter().rev() {
            extrapolated_value = diff_vector.first().unwrap() - extrapolated_value;
        }
        sum_extrapolated_values += extrapolated_value;
    }

    println!("Sum of extrapolated values: {}", sum_extrapolated_values);
 }
