use crate::day2::Opcodes::{Add, End, Error, Multiply};

#[derive(Debug, Eq, PartialEq)]
enum Opcodes {
    Add { x: i64, y: i64, store: i64 },
    Multiply { x: i64, y: i64, store: i64 },
    End { result: i64 },
    Error,
}

fn get_opcodes(index: usize, program: &[i64]) -> Option<Opcodes> {
    let position = program[index];
    let op = match position {
        1 => Add {
            x: program.get(index + 1)?.clone(),
            y: program.get(index + 2)?.clone(),
            store: program.get(index + 3)?.clone(),
        },
        2 => Multiply {
            x: program.get(index + 1)?.clone(),
            y: program.get(index + 2)?.clone(),
            store: program.get(index + 3)?.clone(),
        },
        99 => End {
            result: program.get(0)?.clone(),
        },

        _ => Error,
    };
    Some(op)
}

#[test]
fn test_get_opcodes() {
    let program = [1i64, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(
        get_opcodes(0, &program),
        Some(Add {
            x: 9,
            y: 10,
            store: 3
        })
    );
    assert_eq!(
        get_opcodes(4, &program),
        Some(Multiply {
            x: 3,
            y: 11,
            store: 0
        })
    );
    assert_eq!(get_opcodes(8, &program), Some(End { result: 1 }));
}

fn run_opcodes(op: Opcodes, program: &mut [i64]) {
    match op {
        Opcodes::Add { x, y, store } => {
            let first = program.get(x as usize).unwrap().clone();
            let second = program.get(y as usize).unwrap().clone();
            if let Some(elem) = program.get_mut(store as usize) {
                *elem = first + second;
            }
        }
        Opcodes::Multiply { x, y, store } => {
            let first = program.get(x as usize).unwrap().clone();
            let second = program.get(y as usize).unwrap().clone();
            if let Some(elem) = program.get_mut(store as usize) {
                *elem = first * second;
            }
        }
        _ => unreachable!(),
    }
}

fn run_program(mut program: &mut [i64]) -> Option<i64> {
    let mut pos: usize = 0;
    loop {
        let op = get_opcodes(pos, program)?;
        match op {
            End { result } => return Some(result),
            Add { x, y, store } => run_opcodes(Add { x, y, store }, &mut program),
            Multiply { x, y, store } => run_opcodes(Multiply { x, y, store }, &mut program),
            _ => (),
        }

        pos += 4;
    }
}

#[test]
fn test_run_opcodes() {
    let mut program = [1i64, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(run_program(&mut program), Some(3500));
}

pub fn puzzle() {
    let input = include_str!("../static/day2_prgram.txt");
    let mut prog: Vec<i64> = input.split(",").flat_map(str::parse::<i64>).collect();
    *prog.get_mut(1).unwrap() = 12;
    *prog.get_mut(2).unwrap() = 2;
    let res = run_program(&mut prog);
    println!("{}", res.unwrap());
}

pub fn try_all() {
    let input = include_str!("../static/day2_prgram.txt");
    let prog: Vec<i64> = input.split(",").flat_map(str::parse::<i64>).collect();
    let target = 19690720i64;

    println!("start");
    for noun in 1..100 {
        for verb in 1..100{
                let mut prog2 = prog.clone();
                *prog2.get_mut(1).unwrap() = noun;
                *prog2.get_mut(2).unwrap() = verb;
                let res = run_program(&mut prog2);
                if res.unwrap() == target
                {
                    println!("{2}  {1}   {0}", noun, verb,  100 * noun + verb);
                }
        }
    }
    println!("end");
}
