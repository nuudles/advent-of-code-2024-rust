use itertools::Itertools;
use pathfinding::num_traits::pow;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let mut num_iter = parse_nums::<u64>(line);
            let target = num_iter.next().expect("No target found");
            let numbers = num_iter.collect_vec();
            for mut mask in 0..1 << numbers.len() - 1 {
                let mut value = numbers[0];
                for n in (&numbers).iter().skip(1) {
                    if mask & 1 == 1 {
                        value *= *n;
                    } else {
                        value += *n;
                    }
                    mask = mask >> 1;
                }
                if value == target {
                    return target;
                }
            }
            0
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let mut num_iter = parse_nums::<u64>(line);
            let target = num_iter.next().expect("No target found");
            let numbers = num_iter.collect_vec();
            'outer: for mut i in 0..pow(3, numbers.len() - 1) {
                let mut value = numbers[0];
                for n in (&numbers).iter().skip(1) {
                    let mask = i % 3;
                    if mask == 0 {
                        value += *n;
                    } else if mask == 1 {
                        value *= *n;
                    } else {
                        value = format!("{}{}", value, n).parse().unwrap_or_default();
                    }
                    i /= 3;
                    if value > target {
                        continue 'outer;
                    }
                }
                if value == target {
                    return target;
                }
            }
            0
        })
        .sum::<u64>()
        .print();
}
