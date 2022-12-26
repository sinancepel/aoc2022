use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    left: Vec<u8>,
    right: Vec<u8>,
}

fn priority(c: u8) -> i64 {
    if b'a' <= c && c <= b'z' {
        (c as i64) - (b'a' as i64) + 1
    } else {
        27 + (c as i64) - (b'A' as i64)
    }
}

fn parse_args() -> Vec<Rucksack> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("{} wrong number of arguments: expected 1, got {}.", "Error", args.len());
        std::process::exit(1);
    }
    let data = match fs::read_to_string(&args[0]) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("bad filename {}", &args[0]);
            std::process::exit(1)
        }
    };
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in data.split("\n") {
        let bytes = line.as_bytes();
        if bytes.len() > 0 {
            let (left, right) = bytes.split_at(bytes.len() / 2);
            rucksacks.push(Rucksack { left: left.to_vec(), right: right.to_vec() });
        }
    }
    rucksacks
}

fn in_both_compartments(rucksack: &Rucksack) -> Option<u8> {
    let mut left_set = HashSet::new();
    for item in &rucksack.left {
        left_set.insert(item.clone());
    }
    for item in &rucksack.right {
        if left_set.contains(&item) {
            return Some(item.clone())
        }
    }
    None
}

fn all_items(rucksack: &Rucksack) -> HashSet<u8> {
    let mut items = HashSet::new();
    for item in &rucksack.left {
        items.insert(item.clone());
    }
    for item in &rucksack.right {
        items.insert(item.clone());
    }
    items
}

fn main() {
    let rucksacks = parse_args();
    let mut priority_sum = 0;
    for rucksack_group in rucksacks.chunks(3) {
        let mut items = all_items(&rucksack_group[0]);

        println!("{:?}", &items);
        let second_items = all_items(&rucksack_group[1]);
        items.retain(|item| {second_items.contains(item)});
        let third_items = all_items(&rucksack_group[2]);
        items.retain(|item| {third_items.contains(item)});
        match items.iter().next() {
            Some(value) => {
                priority_sum += priority(*value);
            }
            None => ()
        }
    }
    println!("{:?}", priority_sum);
}
