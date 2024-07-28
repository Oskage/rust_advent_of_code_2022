use std::{
    fmt::Error,
    io::{stdin, BufRead, Stdin},
};

fn fill_level(state: &mut Vec<Vec<char>>, level: String) {
    for (i, box_) in level.chars().skip(1).step_by(4).enumerate() {
        if !box_.is_ascii_whitespace() {
            state[i].push(box_);
        }
    }
}

fn read_state(stdin: Stdin) -> Vec<Vec<char>> {
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .take_while(|l| !l.trim().is_empty())
        .collect();

    let ns: Vec<&str> = lines.last().unwrap().split_ascii_whitespace().collect();
    let n_stacks = ns.last().unwrap().parse::<usize>().unwrap();

    let mut state = vec![vec![]; n_stacks];
    let height = lines.len() - 1;
    for line in lines.into_iter().take(height) {
        fill_level(&mut state, line);
    }

    for i in 0..state.len() {
        state[i].reverse();
    }

    return state;
}

fn main() {
    let stdin = stdin();
    let mut state = read_state(stdin);

    println!("{state:?}");
    // for line in stdin.lock().lines() {}
}
