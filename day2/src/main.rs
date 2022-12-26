use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Value {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct Round {
    opponent_move: Value,
    expected_outcome: Outcome,
}
#[derive(Debug)]
struct Arguments {
    rounds: Vec<Round>,
}

fn value_from_str(s: &str) -> Option<Value> {
    match s {
        "A" => Some(Value::Rock),
        "B" => Some(Value::Paper),
        "C" => Some(Value::Scissors),
        _ => None
    }
}

fn outcome_from_str(s: &str) -> Option<Outcome> {
    match s {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None
    }
}

fn required_move(round: &Round) -> Value {
    match &round.expected_outcome {
        Outcome::Draw => round.opponent_move.clone(),
        Outcome::Win => {
            match &round.opponent_move {
                Value::Rock => Value::Paper,
                Value::Paper => Value::Scissors,
                Value::Scissors => Value::Rock,
            }
        }
        Outcome::Lose => {
            match &round.opponent_move {
                Value::Paper => Value::Rock,
                Value::Scissors => Value::Paper,
                Value::Rock => Value::Scissors,
            }
        }
    }
}

fn round_score(opponent_move: &Value, my_move: &Value) -> i64 {
    match (opponent_move, my_move) {
        (Value::Rock, Value::Paper) => 6,
        (Value::Paper, Value::Scissors) => 6,
        (Value::Scissors, Value::Rock) => 6,
        (Value::Paper, Value::Rock) => 0,
        (Value::Scissors, Value::Paper) => 0,
        (Value::Rock, Value::Scissors) => 0,
        _ => 3
    }
}
fn shape_score(my_move: &Value) -> i64 {
    match my_move {
        Value::Rock => 1,
        Value::Paper => 2,
        Value::Scissors => 3
    }
}

fn parse_args() -> Arguments {
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
    let mut rounds: Vec<Round> = vec![];
    for line in data.split("\n") {
        let (opponent_move, outcome) = match line.split(" ").collect::<Vec<&str>>()[..] {
            [first, second] => (value_from_str(first), outcome_from_str(second)),
            _ => (None, None)
        };
        match (opponent_move, outcome) {
            (Some(opponent), Some(expected_outcome)) =>
                rounds.push(Round { opponent_move: opponent, expected_outcome }),
            _ => ()
        }
    }
    Arguments { rounds }
}

fn main() {
    let arguments = parse_args();
    let mut score = 0;
    for round in arguments.rounds {
        let my_move = required_move(&round);
        let current = round_score(&round.opponent_move, &my_move) + shape_score(&my_move);
        score += current;
        println!("Round {:?} Current {} total {}", &round, &current, &score);
    }
}
