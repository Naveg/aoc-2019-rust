use std::io;

fn get_param(program: &Vec<i32>, ptr: usize, ins: i32, pindex: usize) -> i32 {
    let mode = (ins / ((10 as i32).pow(pindex as u32 + 1))) % 10;
    return match mode {
        0 => program[program[ptr + pindex] as usize],
        1 => program[ptr + pindex],
        _ => 0,
    };
}

fn run_program(program: &mut Vec<i32>, input: i32) {
    let mut ptr: usize = 0;
    while program[ptr] != 99 {
        let ins = program[ptr];
        let op = ins % 100;
        let mut pindex = 1;
        match op {
            1 => {
                let dest = program[ptr + 3] as usize;
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                let in2 = get_param(program, ptr, ins, pindex);
                program[dest] = in1 + in2;
                ptr += 4;
            }
            2 => {
                let dest = program[ptr + 3] as usize;
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                let in2 = get_param(program, ptr, ins, pindex);
                program[dest] = in1 * in2;
                ptr += 4;
            }
            3 => {
                let dest = program[ptr + 1] as usize;
                println!("Input {}", input);
                program[dest] = input;
                ptr += 2;
            }
            4 => {
                println!("Output: {}", get_param(program, ptr, ins, pindex));
                ptr += 2;
            }
            5 => {
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                if in1 != 0 {
                    ptr = get_param(program, ptr, ins, pindex) as usize;
                } else {
                    ptr += 3;
                }
            }
            6 => {
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                if in1 == 0 {
                    ptr = get_param(program, ptr, ins, pindex) as usize;
                } else {
                    ptr += 3;
                }
            }
            7 => {
                let dest = program[ptr + 3] as usize;
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                let in2 = get_param(program, ptr, ins, pindex);
                program[dest] = if in1 < in2 { 1 } else { 0 };
                ptr += 4;
            }
            8 => {
                let dest = program[ptr + 3] as usize;
                let in1 = get_param(program, ptr, ins, pindex);
                pindex += 1;
                let in2 = get_param(program, ptr, ins, pindex);
                program[dest] = if in1 == in2 { 1 } else { 0 };
                ptr += 4;
            }
            _ => panic!("Unsupported opcode {}", op),
        };
    }
}

fn run_with_noun_verb(program: &mut Vec<i32>, noun: i32, verb: i32) {
    program[1] = noun;
    program[2] = verb;
    run_program(program, 0);
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
    //run_with_noun_verb(&mut part1, 12, 2);
    println!("Day 2, Part 1: {}", part1[0]);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut prg = program.to_vec();
            //run_with_noun_verb(&mut prg, noun, verb);
            if prg[0] == 19690720 {
                println!("noun: {}", noun);
                println!("verb: {}", verb);
                println!("Day 2, Part 2: {}", 100 * noun + verb);
            }
        }
    }

    let mut prg = program.to_vec();
    run_program(&mut prg, 1);
    let mut prg = program.to_vec();
    run_program(&mut prg, 5);
}
