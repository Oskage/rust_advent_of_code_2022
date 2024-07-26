use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead}};

fn main() {
    let mut part1_total_score = 0u32;
    let mut part2_total_score = 0u32;

    let mut win_pairs = HashSet::new();
    win_pairs.insert("A Y");
    win_pairs.insert("B Z");
    win_pairs.insert("C X");
    let mut to_win = HashMap::new();
    to_win.insert("A", "Y");
    to_win.insert("B", "Z");
    to_win.insert("C", "X");

    let mut draw_pairs = HashSet::new();
    draw_pairs.insert("A X");
    draw_pairs.insert("B Y");
    draw_pairs.insert("C Z");
    let mut to_draw = HashMap::new();
    to_draw.insert("A", "X");
    to_draw.insert("B", "Y");
    to_draw.insert("C", "Z");

    let mut to_lose = HashMap::new();
    to_lose.insert("A", "Z");
    to_lose.insert("B", "X");
    to_lose.insert("C", "Y");

    let mut turn_to_score = HashMap::new();
    turn_to_score.insert("X", 1);
    turn_to_score.insert("Y", 2);
    turn_to_score.insert("Z", 3);

    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();

            let mut turns = trimmed_line.split_ascii_whitespace();
            let opponent_turn = turns.next().unwrap();
            let our_turn = turns.next().unwrap();
            
            part1_total_score += turn_to_score.get(our_turn).unwrap_or(&0);
            part1_total_score += match trimmed_line {
                line if win_pairs.contains(line) => 6,
                line if draw_pairs.contains(line) => 3,
                _ => 0
            };

            part2_total_score += match our_turn {
                "X" => 0 + turn_to_score[to_lose[opponent_turn]],
                "Y" => 3 + turn_to_score[to_draw[opponent_turn]],
                _ => 6 + turn_to_score[to_win[opponent_turn]]
            };
        }
    }

    println!("part 1 : {part1_total_score}");
    println!("part 2 : {part2_total_score}");
}
