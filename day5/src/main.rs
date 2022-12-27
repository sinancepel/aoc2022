use std::str::FromStr;
use std::fs;
use std::env;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_args() -> (Vec<Vec<char>>, Vec<Move>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let data = fs::read_to_string(&args[0]).unwrap();
    let mut lines = data.lines();
    let mut stacks = vec![];
    let mut stacks_created = false;
    for line in lines.by_ref().take_while(|&line| line.len() > 0) {
        let mut i = 1;
        if line.chars().nth(1).unwrap() == '1' {
            continue;
        }
        while (i < line.len()) {
            if !stacks_created {
                stacks.push(vec![]);
            }
            let c = line.chars().nth(i).unwrap();
            if c != ' ' {
                stacks[i / 4].push(c);
            }
            i += 4;
        }
        stacks_created = true;
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    let mut moves = vec![];
    for line in lines {
        let move_str: Vec<&str> = line.split(" ").collect();
        if move_str.len() < 6 {
            break;
        }
        moves.push(Move {
             count: usize::from_str(move_str[1]).unwrap(),
             from: usize::from_str(move_str[3]).unwrap() - 1,
             to: usize::from_str(move_str[5]).unwrap() - 1
        });
    }
    (stacks, moves)
}

fn simulate(stacks:&mut Vec<Vec<char>>, m: &Move) {
    let mut count = m.count.clone();
    let mut i = stacks[m.from].len().clone() - count;
    while count > 0 {
        let item = stacks[m.from][i].clone();
        stacks[m.to].push(item);
        i += 1;
        count -= 1;
    }
    count = m.count.clone();
    println!("Second part {}", count);
    while count > 0 {
        stacks[m.from].pop();
        count -= 1;
    }
}

fn main() {
    let (mut stacks, moves) = parse_args();
    for m in moves {
        println!("Stacks before move {:?}", &stacks);
        println!("{:?}", &m);
        simulate(&mut stacks, &m);
    }
    println!("Stacks at end {:?}", &stacks);
    let mut s = vec![];
    for stack in &stacks {
        s.push(stack[stack.len() - 1].clone());
    }
    println!("{:?}", s);
}
