use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Arguments {
    infile: String,
}
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("{} wrong number of arguments: expected 1, got {}.", "Error", args.len());
        std::process::exit(1);
    }
    Arguments { infile: args[0].clone() }

}

fn parse_input(data: &str) -> Vec<Vec<i64>> {
    let mut inputs: Vec<Vec<i64>> = vec![];
    inputs.push(vec![]);
    let lines = data.split("\n");
    for line in lines {
        match i64::from_str(&line) {
            Ok(calories) => {
                match inputs.last_mut() {
                    Some(last_item) => { last_item.push(calories); }
                    None => ()
                }
            }
            Err(_) => {
                // New person.
                inputs.push(vec![]);
            }
        }
    };
    inputs
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let data = match fs::read_to_string(&args.infile) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("bad filename {}", &args.infile);
            std::process::exit(1)
        }
    };
    let inputs: Vec<Vec<i64>> = parse_input(&data);
    let mut calories: Vec<i64> = vec![];
    for input in inputs {
        let mut current_maximum = 0;
        for calories in input {
            current_maximum += calories;
        }
        calories.push(current_maximum);
    }
    calories.sort();
    println!("{:?}", calories);
    match (calories.pop(), calories.pop(), calories.pop()) {
        (Some(a), Some(b), Some(c)) => { println!("{}", a + b + c); }
        _ => {}
    }

}
