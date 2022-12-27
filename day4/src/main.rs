use std::str::FromStr;
use std::fs;
use std::env;

#[derive(Debug)]
struct Region {
    start: i64,
    len: i64,
}

fn parse_region(region_str: &str) -> Option<Region> {
    match region_str.split("-").collect::<Vec<&str>>()[..] {
        [left, right] => {
            let start = i64::from_str(left).expect("start");
            let end = i64::from_str(right).expect("end");
            Some(Region { start, len: end - start })
        }
        _ => None
    }
}

fn parse_args() -> Vec<(Region, Region)> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("{} wrong number of arguments: expected 1, got {}.", "Error", args.len());
        std::process::exit(1);
    }
    let data = fs::read_to_string(&args[0]).unwrap();
    let mut regions: Vec<(Region, Region)> = vec![];
    for line in data.split("\n") {
        println!("here");
        let sections = line.split(",").collect::<Vec<&str>>();
        if sections.len() != 2 {
            continue
        }
        let left_region = parse_region(&sections[0]);
        let right_region = parse_region(&sections[1]);
        match (left_region, right_region) {
            (Some(left), Some(right)) => regions.push((left, right)),
            _ => ()
        }
    }
    regions
}

fn encloses(bigger: &Region, smaller: &Region) -> bool {
    bigger.start <= smaller.start && (bigger.start + bigger.len) >= (smaller.start + smaller.len)
}

fn bonks(a: &Region, b: &Region) -> bool {
    if a.start <= b.start && (a.start + a.len) >= b.start {
        return true
    }
    return false
}

fn main() {
    let arguments = parse_args();
    println!("got args");
    let mut matches = 0;
    for (a, b) in arguments {
        if bonks(&a, &b) || bonks(&b, &a) {
            matches += 1;
        }
    }
    println!("{}", matches);
}
