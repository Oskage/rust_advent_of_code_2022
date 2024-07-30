use std::{
    collections::VecDeque,
    io::{self, Read},
};

#[derive(Debug)]
struct BoundedQueue<T> {
    queue: VecDeque<T>,
    max_length: usize,
}

impl<T> BoundedQueue<T> {
    fn new(max_length: usize) -> Self {
        BoundedQueue {
            queue: VecDeque::new(),
            max_length,
        }
    }

    fn push(&mut self, item: T) {
        if self.queue.len() == self.max_length {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }

    fn erase_front(&mut self, n: usize) {
        self.queue.drain(..n);
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T: PartialEq> BoundedQueue<T> {
    fn find_last(&self, item: &T) -> Option<usize> {
        self.queue.iter().rposition(|r| r == item)
    }
}

impl<T: Clone> Extend<T> for BoundedQueue<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

fn solve(input: &mut String, signal_length: usize) -> usize {
    let mut seen = BoundedQueue::new(signal_length);
    let mut marker: usize = 0;

    let mut input_iter = input.chars();
    loop {
        let ch = input_iter.next();
        marker += 1;

        if let Some(ch) = ch {
            if let Some(pos) = seen.find_last(&ch) {
                seen.erase_front(pos + 1);
                seen.push(ch);
            } else if seen.len() < signal_length - 1 {
                seen.push(ch);
                continue;
            } else {
                break marker;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let part1_answer = solve(&mut input, 4);
    let part2_answer = solve(&mut input, 14);

    println!("part 1 : {part1_answer}");
    println!("part 2 : {part2_answer}");
}
