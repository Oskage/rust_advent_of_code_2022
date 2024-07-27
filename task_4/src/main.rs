use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

fn parse_elf_order(order: &str) -> Option<(i32, i32)> {
    match order.split_once('-') {
        Some((begin, end)) => {
            let begin_as_i32 = begin.parse::<i32>().unwrap();
            let end_as_i32 = end.parse::<i32>().unwrap();
            return Some((begin_as_i32, end_as_i32));
        }
        None => None,
    }
}

fn main() {
    let mut number_of_full_overlaps = 0;
    let mut number_of_overlaps = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines().filter_map(Result::ok) {
        if let Some((first, second)) = line.split_once(',') {
            let first_elf_order = parse_elf_order(first).unwrap();
            let second_elf_order = parse_elf_order(second).unwrap();

            number_of_full_overlaps += match Ord::cmp(&first_elf_order.0, &second_elf_order.0) {
                Ordering::Less => (first_elf_order.1 >= second_elf_order.1) as i32,
                Ordering::Equal => 1,
                Ordering::Greater => (first_elf_order.1 <= second_elf_order.1) as i32,
            };

            number_of_overlaps += match Ord::cmp(&first_elf_order.0, &second_elf_order.0) {
                Ordering::Less => (first_elf_order.1 >= second_elf_order.0) as i32,
                Ordering::Equal => 1,
                Ordering::Greater => (second_elf_order.1 >= first_elf_order.0) as i32,
            }
        }
    }

    println!("part 1 : {number_of_full_overlaps}");
    println!("part 2 : {number_of_overlaps}");
}
