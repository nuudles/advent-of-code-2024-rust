use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut stones = parse_nums::<u64>(&input).collect_vec();
    for _ in 0..25 {
        let mut next = vec![];
        for stone in stones {
            if stone == 0 {
                next.push(1);
            } else {
                let as_str = stone.to_string();
                if as_str.len() % 2 == 0 {
                    let (left, right) = as_str.split_at(as_str.len() / 2);
                    next.push(left.parse().expect("Could not parse left"));
                    next.push(right.parse().expect("Could not parse left"));
                } else {
                    next.push(stone * 2024);
                }
            }
        }
        stones = next;
    }
    println!("{}", stones.len());
}

pub fn part2(input: String) {
    let mut stones = parse_nums::<u64>(&input)
        .map(|n| (n, 1u64))
        .collect::<BTreeMap<_, _>>();
    for _ in 0..75 {
        let mut next = BTreeMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                *next.entry(1).or_default() += count;
            } else {
                let as_str = stone.to_string();
                if as_str.len() % 2 == 0 {
                    let (left, right) = as_str.split_at(as_str.len() / 2);
                    *next
                        .entry(left.parse().expect("Could not parse left"))
                        .or_default() += count;
                    *next
                        .entry(right.parse().expect("Could not parse right"))
                        .or_default() += count;
                } else {
                    *next.entry(stone * 2024).or_default() += count;
                }
            }
        }
        stones = next;
    }
    stones.values().sum::<u64>().print();
}
