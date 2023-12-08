use std::fs;

 fn main() {
     let mut calibration_value = 0;
     for line in fs::read_to_string("input.txt").unwrap().lines() {
         let mut first_digit = 'a';
         let mut last_digit = 'a';
         for c in line.to_string().chars() {
             if c.is_digit(10) {
                 if first_digit == 'a' {
                     first_digit = c;
                     last_digit = c;
                 } else {
                     last_digit = c;
                 }
             }
         }
         let mut my_number = first_digit.to_string();
         my_number.push(last_digit);
         calibration_value += my_number.parse::<i32>().unwrap();
     }
     println!("Calibration value: {}", calibration_value);
 }