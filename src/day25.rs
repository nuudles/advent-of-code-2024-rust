use std::collections::HashSet;

use itertools::iproduct;

use crate::point::Point;

pub fn part1(input: String) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in input.split("\n\n") {
        let is_lock = block
            .lines()
            .next()
            .expect("First line not found")
            .chars()
            .all(|c| c == '#');
        let mut set = HashSet::new();
        for (y, line) in block.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    set.insert(Point { x, y });
                }
            }
        }
        if is_lock {
            locks.push(set);
        } else {
            keys.push(set);
        }
    }

    let mut count = 0;
    for (lock, key) in iproduct!(locks, keys) {
        if lock.intersection(&key).count() == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
