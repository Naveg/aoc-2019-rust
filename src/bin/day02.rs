use std::io;

fn run_program(program: &mut Vec<i32>, noun: i32, verb: i32) {
    program[1] = noun;
    program[2] = verb;
    let mut ptr = 0;
    while program[ptr] != 99 {
        let op = program[ptr];
        let in1 = program[program[ptr + 1] as usize];
        let in2 = program[program[ptr + 2] as usize];
        let dest = program[ptr + 3] as usize;
        program[dest] = match op {
            1 => in1 + in2,
            2 => in1 * in2,
            _ => 0,
        };
        ptr += 4;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i32> = input
        .trim()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();
    let mut part1 = program.to_vec();
    run_program(&mut part1, 12, 2);
    println!("Part 1: {}", part1[0]);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut prg = program.to_vec();
            run_program(&mut prg, noun, verb);
            if prg[0] == 19690720 {
                println!("noun: {}", noun);
                println!("verb: {}", verb);
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}
