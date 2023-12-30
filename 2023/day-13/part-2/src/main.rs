use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Terrain {
    Ash,
    Rocks,
    Unknown
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '.' => Terrain::Ash,
            '#' => Terrain::Rocks,
            _ => Terrain::Unknown,
        }
    }
}

// Now, two lines are identical if they differ exactly in one element
fn identical(pattern_a: &Vec<Terrain>, pattern_b: &Vec<Terrain>) -> (bool, bool) {
    // Iterator::zip takes two iterators and produces another 
    // iterator of the tuple of each iterator's values.
    // Iterator::filter takes a reference to the iterator's value 
    // and discards any value where the predicate closure returns false. 
    // This performs the comparison.
    // Iterator::count counts the number of elements in the iterator.
    let matching = pattern_a
        .iter()
        .zip(pattern_b)
        .filter(|&(a, b)| a == b).count();
    if matching == pattern_a.len() {
        return (true, false);
    } else if matching == pattern_a.len() - 1 {
        // println!("Smudge found");
        return (true, true);        
    } else {
        return (false, false);
    }
}

// Return the start index of the line of reflection
fn get_line_of_reflection(pattern: &Vec<Vec<Terrain>>) -> usize {
    let mut i = 0;
    let mut start_line_reflection = usize::MAX;
    let mut reflection;
    let mut smudge;
    let mut smudge_found;
    while i < pattern.len() - 1 {
        smudge_found = false;
        (reflection, smudge) = identical(&pattern[i], &pattern[i+1]);
        if smudge { 
            smudge_found = true;
        }
        if reflection {
            // println!("Identical consecutive lines found: {} and {}", i, i + 1);
            // Two consecutive identical lines found
            // Now we need to check in both directions
            if i == 0 || i == pattern.len() - 2 {
                // If it is the first pattern or the second to last,
                // and there is a smudge, we don't need to check further
                if smudge_found {
                    start_line_reflection = i;
                    break;        
                }
            } else {
                let mut j = 1;
                let mut different_lines_found = false;
                while i + j + 1 < pattern.len() {
                    // println!("Comparing rows {} and {}", i-j, i+j+1);
                    (reflection, smudge) = identical(&pattern[i-j], &pattern[i+j+1]);
                    if smudge { 
                        smudge_found = true;
                    }
                    if !reflection {
                        different_lines_found = true;
                        // println!("Rows {} and {} are different", i-j, i+j+1);
                        break;
                    } else {
                        // println!("Rows {} and {} are identical", i-j, i+j+1);
                    }
                    // println!("Smudge: {}", smudgeFound);
                    j += 1;
                    if j > i {
                        break;
                    }
                }
                // println!("Smudge: {}", smudgeFound);
                if !different_lines_found && smudge_found {
                    start_line_reflection = i;
                    // println!("Horizontal line of reflection found between {} and {}", i, i+1);
                    break;
                }    
            }
        }
        i += 1;
    }
    return start_line_reflection;
}

fn summarize_pattern_notes(pattern: &Vec<Vec<Terrain>>) -> usize {

    // Find horizontal lines of reflections
    let mut start_line_reflection = get_line_of_reflection(pattern);
    if start_line_reflection != usize::MAX {
        return (start_line_reflection + 1) * 100;
    } else {
        // If we haven't found an horizontal line of reflection,
        // look for a vertical line of reflection

        // Build column patterns
        let mut i = 0;
        let mut column_pattern: Vec<Vec<Terrain>> = vec![];
        while i < pattern[0].len() {
            let mut cp: Vec<Terrain> = vec![];
            let mut j = 0;
            while j < pattern.len() {
                cp.push(pattern[j][i].clone());
                j += 1;
            }
            column_pattern.push(cp);
            i += 1;
        }

        start_line_reflection = get_line_of_reflection(&column_pattern);
        if start_line_reflection != usize::MAX {
            return start_line_reflection + 1;
        } else {
            println!("No line of reflection found");
            return 0;
        }            
    }
}

fn main() {
    let mut sum_pattern_notes = 0;

    let mut pattern: Vec<Vec<Terrain>> = vec![];
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if !line.is_empty() {
            pattern.push(
                line
                .chars()
                .map(|c| c.into())
                .collect()
            );
        } else {
            sum_pattern_notes += summarize_pattern_notes(&pattern);
            pattern = vec![];
        }
    }  
    sum_pattern_notes += summarize_pattern_notes(&pattern);

    println!("Sum of pattern notes: {}", sum_pattern_notes);
 }
