use std::{
    collections::VecDeque,
    fs::read,
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
            self.queue.pop_front(); // Remove the oldest item
        }
        self.queue.push_back(item);
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T: PartialEq> BoundedQueue<T> {
    fn find(&self, item: &T) -> Option<usize> {
        self.queue.iter().position(|r| r == item)
    }
}

impl<T: Clone> Extend<T> for BoundedQueue<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

fn read_n_chars<R: Read>(stream: R, seen: &mut BoundedQueue<u8>, n: usize) {
    seen.extend(stream.bytes().filter_map(Result::ok).take(n))
}

fn main() {
    let stdin = io::stdin();

    let mut seen = BoundedQueue::new(4);
    // while seen.len() != 4 {
    //     let ch = stdin.lock().bytes().filter_map(Result::ok).take(1).next();
    //     if let Some(ch) = ch {
    //         if let Some(pos) = seen.find(&ch) {
    //             read_n_chars(stdin.lock(), &mut seen, pos);
    //             marker += pos;
    //         } else {
    //             break marker;
    //         }
    //     } else {
    //         unreachable!();
    //     };
    // }
    // read_n_chars(stdin.lock(), &mut seen, 4);

    let mut marker: usize = 0;
    let part1_answer = loop {
        let ch = stdin.lock().bytes().filter_map(Result::ok).take(1).next();
        marker += 1;

        if let Some(ch) = ch {
            if let Some(pos) = seen.find(&ch) {
                read_n_chars(stdin.lock(), &mut seen, pos);
                marker += pos;
                seen.push(ch);

                let a: String = seen.queue.iter().map(|&x| x as char).collect();
                println!("{a}");
            } else if seen.len() < 4 {
                seen.push(ch);

                let a: String = seen.queue.iter().map(|&x| x as char).collect();
                println!("{a}");

                continue;
            } else {
                break marker;
            }
        } else {
            unreachable!();
        };
    };

    println!("part 1 : {part1_answer}");
}
