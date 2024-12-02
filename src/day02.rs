use std::i64;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn is_safe(nums: &Vec<i64>) -> bool {
    let mut direction = i64::MAX;
    for (a, b) in nums.iter().tuple_windows() {
        if direction == i64::MAX {
            if a > b {
                direction = -1;
            } else if a < b {
                direction = 1;
            } else {
                return false;
            }
        }
        if direction < 0 && (a - b < 1 || a - b > 3) {
            return false;
        } else if direction > 0 && (b - a < 1 || b - a > 3) {
            return false;
        }
    }
    true
}

pub fn part1(input: String) {
    input
        .lines()
        .filter(|line| is_safe(&parse_nums(line).collect()))
        .count()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .filter(|line| {
            let nums = parse_nums::<i64>(line).collect_vec();
            if is_safe(&nums) {
                return true;
            }
            for i in 0..nums.len() {
                let mut removed = (&nums).clone();
                removed.remove(i);
                if is_safe(&removed) {
                    return true;
                }
            }
            false
        })
        .count()
        .print();
}
