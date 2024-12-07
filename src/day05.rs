use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn matches_rules(numbers: &Vec<u64>, rules: &Vec<(u64, u64)>) -> bool {
    let positions = numbers
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<HashMap<_, _>>();
    if rules.iter().all(|(left, right)| {
        if let Some(p1) = positions.get(left) {
            if let Some(p2) = positions.get(right) {
                return p1 < p2;
            }
            return true;
        }
        true
    }) {
        true
    } else {
        false
    }
}

pub fn part1(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("No divider?");
    let rules = top
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").expect("No | found");
            (
                left.parse::<u64>().unwrap_or_default(),
                right.parse::<u64>().unwrap_or_default(),
            )
        })
        .collect_vec();
    bottom
        .lines()
        .map(|line| {
            let numbers = line.split(',').flat_map(|n| n.parse::<u64>()).collect_vec();
            if matches_rules(&numbers, &rules) {
                numbers[numbers.len() / 2]
            } else {
                0
            }
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("No divider?");
    let rules = top
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").expect("No | found");
            (
                left.parse::<u64>().unwrap_or_default(),
                right.parse::<u64>().unwrap_or_default(),
            )
        })
        .collect_vec();
    bottom
        .lines()
        .map(|line| {
            let numbers = line.split(',').flat_map(|n| n.parse::<u64>()).collect_vec();
            if matches_rules(&numbers, &rules) {
                return 0;
            } else {
                let sorted = numbers
                    .iter()
                    .sorted_unstable_by(|a, b| {
                        for (l, r) in &rules {
                            if l == *a && r == *b {
                                return Ordering::Less;
                            } else if r == *a && l == *a {
                                return Ordering::Greater;
                            }
                        }
                        Ordering::Equal
                    })
                    .collect_vec();
                *sorted[numbers.len() / 2]
            }
        })
        .sum::<u64>()
        .print();
}
