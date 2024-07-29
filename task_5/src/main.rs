use std::{
    fmt::Error,
    io::{stdin, BufRead, Stdin},
};

#[derive(Debug)]
struct Command {
    cnt: i32,
    src: i32,
    dst: i32,
}

// impl FromIterator<i32> for Command {
//     fn from_iter<T: IntoIterator<Item = i32>>(_: T) -> Self {
//         todo!()
//     }
// }

fn fill_level(state: &mut Vec<Vec<char>>, level: String) {
    for (i, box_) in level.chars().skip(1).step_by(4).enumerate() {
        if !box_.is_ascii_whitespace() {
            state[i].push(box_);
        }
    }
}

fn read_state(stdin: &Stdin) -> Vec<Vec<char>> {
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

    for stack in state.iter_mut() {
        stack.reverse();
    }

    return state;
}

fn read_commands(stdin: &Stdin) -> Vec<Command> {
    let mut commands = vec![];

    // stdin
    //     .lock()
    //     .lines()
    //     .filter_map(Result::ok)
    //     .map(|l| l.split_ascii_whitespace().skip(1).step_by(2).collect())
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let command_items: Vec<i32> = line
            .split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<i32>().unwrap())
            .take(3)
            .collect();
        commands.push(Command {
            cnt: command_items[0],
            src: command_items[1],
            dst: command_items[2],
        });
    }

    commands
}

fn apply_command(state: &mut Vec<Vec<char>>, command: Command) {}

fn main() {
    let stdin = stdin();
    let mut state = read_state(&stdin);
    let commands = read_commands(&stdin);
    for command in commands {
        apply_command(&mut state, command)
    }

    // println!("{state:?}");

    // for line in stdin.lock().lines() {}
}
