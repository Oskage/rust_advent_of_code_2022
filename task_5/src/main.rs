use std::io::{stdin, BufRead, Stdin};

#[derive(Debug)]
struct Command {
    cnt: i32,
    src: usize,
    dst: usize,
}

enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

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
            src: (command_items[1] - 1) as usize,
            dst: (command_items[2] - 1) as usize,
        });
    }

    commands
}

fn apply_command(state: &mut Vec<Vec<char>>, command: &Command, crane_type: CraneType) {
    let src_n_boxes = state[command.src].len();
    let boxes: Vec<char> = match crane_type {
        CraneType::CrateMover9000 => state[command.src]
            .drain((src_n_boxes - command.cnt as usize)..)
            .rev()
            .collect(),
        CraneType::CrateMover9001 => state[command.src]
            .drain((src_n_boxes - command.cnt as usize)..)
            .collect(),
    };
    state[command.dst].extend(boxes);
}

fn main() {
    let stdin = stdin();
    let mut part1_state = read_state(&stdin);
    let mut part2_state = part1_state.clone();
    let commands = read_commands(&stdin);
    for command in commands {
        apply_command(&mut part1_state, &command, CraneType::CrateMover9000);
        apply_command(&mut part2_state, &command, CraneType::CrateMover9001);
    }

    let part1_answer: String = part1_state.iter().filter_map(|x| x.last()).collect();
    let part2_answer: String = part2_state.iter().filter_map(|x| x.last()).collect();

    println!("part 1 : {part1_answer}");
    println!("part 2 : {part2_answer}");
}
