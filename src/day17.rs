use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn combo_operand(operand: u64, registers: &BTreeMap<&str, u64>) -> u64 {
    match operand {
        4 => registers["A"],
        5 => registers["B"],
        6 => registers["C"],
        7 => panic!("Reserved operand"),
        _ => operand,
    }
}

fn run_program(program: &Vec<u64>, a: u64) -> Vec<u64> {
    let mut outputs = vec![];

    let mut registers = BTreeMap::new();
    registers.insert("A", a);
    registers.insert("B", 0);
    registers.insert("C", 0);

    let mut pointer = 0;
    while pointer < program.len() {
        let instruction = program[pointer];
        let operand = program[pointer + 1];

        match instruction {
            0 => {
                _ = registers.insert(
                    "A",
                    registers["A"] / (1 << combo_operand(operand, &registers)),
                )
            }
            1 => _ = registers.insert("B", registers["B"] ^ operand),
            2 => _ = registers.insert("B", combo_operand(operand, &registers) % 8),
            3 => {
                if registers["A"] != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => _ = registers.insert("B", registers["B"] ^ registers["C"]),
            5 => outputs.push(combo_operand(operand, &registers) % 8),
            6 => {
                _ = registers.insert(
                    "B",
                    registers["A"] / (1 << combo_operand(operand, &registers)),
                )
            }
            7 => {
                _ = registers.insert(
                    "C",
                    registers["A"] / (1 << combo_operand(operand, &registers)),
                )
            }
            _ => panic!("Unknown instruction"),
        }
        pointer += 2;
    }
    outputs
}

pub fn part1(input: String) {
    let mut num_iter = parse_nums::<u64>(&input);
    let mut registers = BTreeMap::new();
    registers.insert("A", num_iter.next().expect("A not found"));
    registers.insert("B", num_iter.next().expect("B not found"));
    registers.insert("C", num_iter.next().expect("C not found"));

    let program = num_iter.collect_vec();
    let outputs = run_program(&program, registers["A"]);

    outputs.iter().join(",").print();
}

/*
My original approach was to first determine the minimum number that would produce the number of outputs I needed. Then I ran my brute forcer starting at that number, while printing out candidates that matched at least the first 10 outputs. Then I looked at the binary of these candidates and saw that they shared the same last 24 bits. So I then had a brute forcer that kept those last 24 bits and I got an answer relatively quickly. However, that answer was wrong. Then I plugged the number into my original brute forcer, but subtracted like 10000, and it found the actual answer (somehow this answer did not share the same last 24 bits, which was why my faster brute forcer did not catch it). Later, I decided to take advantage of how the program worked, which operates on each 3 bits in order in order to reverse engineer the solution using this function:
*/
pub fn part2(input: String) {
    let num_iter = parse_nums::<u64>(&input);
    let program: Vec<u64> = num_iter.skip(3).collect_vec();

    let mut possibles = BTreeSet::from([0]);
    let mut seeking = VecDeque::new();
    for &num in program.iter().rev() {
        seeking.push_front(num);
        let mut next = BTreeSet::new();
        for i in 0..8 {
            for possibility in &possibles {
                let outputs = run_program(&program, possibility << 3 | i)
                    .iter()
                    .map(|x| *x)
                    .collect::<VecDeque<_>>();
                if outputs == seeking {
                    next.insert(possibility << 3 | i);
                }
            }
        }
        possibles = next;
    }
    possibles.iter().min().expect("No min found").print();
}

// fn seek_program(program: &Vec<u64>, a: u64) -> Vec<u64> {
//     let mut outputs = vec![];

//     let mut registers = BTreeMap::new();
//     registers.insert("A", a);
//     registers.insert("B", 0);
//     registers.insert("C", 0);

//     let mut seek_index = 0;

//     let mut pointer = 0;
//     while pointer < program.len() {
//         let instruction = program[pointer];
//         let operand = program[pointer + 1];

//         match instruction {
//             0 => {
//                 _ = registers.insert(
//                     "A",
//                     registers["A"] / (1 << combo_operand(operand, &registers)),
//                 )
//             }
//             1 => _ = registers.insert("B", registers["B"] ^ operand),
//             2 => _ = registers.insert("B", combo_operand(operand, &registers) % 8),
//             3 => {
//                 if registers["A"] != 0 {
//                     pointer = operand as usize;
//                     continue;
//                 }
//             }
//             4 => _ = registers.insert("B", registers["B"] ^ registers["C"]),
//             5 => {
//                 let value = combo_operand(operand, &registers) % 8;
//                 outputs.push(value);
//                 if seek_index >= program.len() || value != program[seek_index] {
//                     return outputs;
//                 } else {
//                     seek_index += 1;
//                 }
//             }
//             6 => {
//                 _ = registers.insert(
//                     "B",
//                     registers["A"] / (1 << combo_operand(operand, &registers)),
//                 )
//             }
//             7 => {
//                 _ = registers.insert(
//                     "C",
//                     registers["A"] / (1 << combo_operand(operand, &registers)),
//                 )
//             }
//             _ => panic!("Unknown instruction"),
//         }
//         pointer += 2;
//     }
//     outputs
// }

// fn test_program(program: &Vec<u64>, initial_a: u64) -> Vec<u64> {
//     let mut outputs = vec![];
//     let mut a = initial_a;
//     let mut b;
//     let mut c;

//     let mut seek_index = 0;

//     loop {
//         b = a % 8;
//         b = b ^ 1;
//         c = a >> b;
//         b = b ^ 5;
//         b = b ^ c;
//         a = a / 8;
//         outputs.push(b % 8);
//         if b % 8 != program[seek_index] {
//             return outputs;
//         } else {
//             seek_index += 1;
//         }
//         if a == 0 {
//             break;
//         }
//     }
//     outputs
// }

// pub fn part2(input: String) {
//     let mut num_iter = parse_nums::<u64>(&input);
//     let mut registers = BTreeMap::new();
//     registers.insert("A", num_iter.next().expect("A not found"));
//     registers.insert("B", num_iter.next().expect("B not found"));
//     registers.insert("C", num_iter.next().expect("C not found"));
//
//     let program: Vec<u64> = num_iter.collect_vec();
//     let mut a = 164540890000645;
//     let mut last_last = 0;
//     let mut last = 0;
//     loop {
//         let outputs = seek_program(&program, a);
//         // let outputs = test_program(&program, a);
//         if outputs == program {
//             break;
//         } else if outputs.len() >= 11 {
//             println!("{} {} {} {}", a, a - last, last - last_last, outputs.len());
//             // println!("{:?}", outputs);
//             last_last = last;
//             last = a;
//         }
//         a += 1;
//     }
//     println!("{}", a);
// }

// pub fn part2(input: String) {
//     let num_iter = parse_nums::<u64>(&input);
//     let program: Vec<u64> = num_iter.skip(3).collect_vec();
//
//     let mut a = 1;
//     loop {
//         let outputs = test_program(&program, (a << 24) | 7172029);
//         println!("{:?}", outputs);
//         if outputs.len() == 10 {
//             println!("{}", (a << 24) | 7172029);
//         }
//         if outputs == program {
//             break;
//         }
//         a += 1;
//     }
//     println!("{}", (a << 24) | 7172029);
// }
