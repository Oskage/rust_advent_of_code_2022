use std::{collections::HashSet, io::stdin};

fn char_to_priority(char: char) -> u8 {
    let char_as_int = char as u8 - 64;

    if char_as_int > 26 {
        char_as_int - 32
    } else {
        char_as_int + 26
    }
}

fn main() {
    let mut part1_priorities_sum = 0u32;
    let mut part2_priorities_sum = 0u32;
    let mut first_half_set = HashSet::new();

    let mut first_elf_stuff = HashSet::new();
    let mut second_elf_stuff = HashSet::new();

    for (i, line) in stdin().lines().enumerate() {
        if let Ok(line) = line {
            let (first_half, second_half) = line.split_at(line.len() / 2);

            for char in first_half.chars() {
                first_half_set.insert(char);
            }

            let mut common_char = second_half
                .chars()
                .filter(|x| first_half_set.contains(&x))
                .next()
                .unwrap();

            part1_priorities_sum += char_to_priority(common_char) as u32;
            first_half_set.clear();

            match i {
                elf_num if (elf_num + 1) % 3 == 1 => {
                    first_elf_stuff.clear();
                    first_elf_stuff.extend(line.chars());
                }
                elf_num if (elf_num + 1) % 3 == 2 => {
                    second_elf_stuff.clear();
                    second_elf_stuff.extend(line.chars());
                }
                _ => {
                    common_char = line
                        .chars()
                        .filter(|x| first_elf_stuff.contains(x) && second_elf_stuff.contains(x))
                        .next()
                        .unwrap();

                    part2_priorities_sum += char_to_priority(common_char) as u32;
                }
            }
        }
    }

    println!("part 1 : {part1_priorities_sum}");
    println!("part 2 : {part2_priorities_sum}");
}
