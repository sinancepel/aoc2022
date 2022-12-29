use std::fs;
use std::env;
use std::collections::HashSet;

struct RingBuffer {
    capacity: usize,
    next_free: usize,
    items: Vec<char>,
}

impl RingBuffer {
    fn add(self: &mut RingBuffer, item: char) -> () {
        if self.items.len() < self.capacity {
            self.items.push(item);
            return
        }
        self.items[self.next_free] = item;
        self.next_free = (self.next_free + 1) % self.capacity;
    }

    fn new(capacity: usize) -> RingBuffer {
        RingBuffer { capacity, next_free: 0, items: vec![] }
    }

    fn all_unique(&self) -> bool {
        if self.items.len() < self.capacity {
            return false
        }
        let mut seen = vec![];
        for item in &self.items {
            if seen.contains(&item) {
                return false;
            }
            seen.push((&item).clone());
        }
        return true;
    }
}
fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    let data = fs::read_to_string(&args[0]).unwrap();
    data
}

fn main() {
    let data = parse_args();
    let mut buffer = RingBuffer::new(14);
    let mut count = 0;
    for c in data.chars() {
        count += 1;
        buffer.add(c);
        if buffer.all_unique() {
            println!("Done with {}", count);
            return;
        }
    }
}
