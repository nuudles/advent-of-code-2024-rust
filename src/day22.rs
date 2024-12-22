use std::collections::{HashMap, VecDeque};

use cached::proc_macro::cached;

use crate::selfprint::SelfPrint;

#[cached()]
fn secret_number(x: i64) -> i64 {
    let x = ((x * 64) ^ x) % 16777216;
    let x = ((x / 32) ^ x) % 16777216;
    ((x * 2048) ^ x) % 16777216
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|x| {
            let mut x = x.parse::<i64>().expect("Could not parse line");
            for _ in 0..2000 {
                x = secret_number(x);
            }
            x
        })
        .sum::<i64>()
        .print();
}

pub fn part2(input: String) {
    let mut sequences = HashMap::<VecDeque<i64>, HashMap<usize, i64>>::new();
    for (index, line) in input.lines().enumerate() {
        let mut x = line.parse::<i64>().expect("Could not parse line");
        let mut differences = VecDeque::new();
        for _ in 0..2000 {
            let next = secret_number(x);
            differences.push_back((next % 10) - (x % 10));
            if differences.len() == 4 {
                let map = sequences.entry(differences.clone()).or_default();
                if !map.contains_key(&index) {
                    map.insert(index, next % 10);
                }
                differences.pop_front();
            }
            x = next;
        }
    }
    sequences
        .iter()
        .map(|(_, v)| v.values().sum::<i64>())
        .max()
        .expect("No max found")
        .print();
}
