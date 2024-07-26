use std::{i32, io::{stdin, BufRead}};

fn main() {
    let mut top3 = (i32::MIN, i32::MIN, i32::MIN);

    let mut curr_calories_sum = 0;
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            if line.trim_end().len() > 0 {
                curr_calories_sum += line.parse::<i32>().unwrap();
            } else {
                if curr_calories_sum >= top3.0 {
                    top3.2 = top3.1;
                    top3.1 = top3.0;
                    top3.0 = curr_calories_sum;
                } else if curr_calories_sum >= top3.1 {
                    top3.2 = top3.1;
                    top3.1 = curr_calories_sum;
                } else if curr_calories_sum > top3.2 {
                    top3.2 = curr_calories_sum;
                }
                curr_calories_sum = 0;
            }
        }
    }

    if curr_calories_sum >= top3.0 {
        top3.2 = top3.1;
        top3.1 = top3.0;
        top3.0 = curr_calories_sum;
    } else if curr_calories_sum >= top3.1 {
        top3.2 = top3.1;
        top3.1 = curr_calories_sum;
    } else if curr_calories_sum > top3.2 {
        top3.2 = curr_calories_sum;
    }

    let part1_answer = top3.0;
    let part2_answer = top3.0 + top3.1 + top3.2;
    println!("part 1 : {part1_answer}");
    println!("part 2 : {part2_answer}");
}
