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

fn main() {
    let stdin = io::stdin();
    let max_len: usize = 4;

    let mut seen = BoundedQueue::new(max_len);
    let mut marker: usize = 0;
    let part1_answer = loop {
        let ch = stdin.lock().bytes().filter_map(Result::ok).take(1).next();
        marker += 1;

        if let Some(ch) = ch {
            if let Some(pos) = seen.find_last(&ch) {
                seen.erase_front(pos + 1);
                seen.push(ch);
            } else if seen.len() < max_len - 1 {
                seen.push(ch);
                continue;
            } else {
                break marker;
            }
        }
    };

    println!("part 1 : {part1_answer}");
}
