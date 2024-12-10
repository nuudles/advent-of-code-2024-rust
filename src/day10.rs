use std::collections::{BTreeMap, BTreeSet};

use itertools::{iproduct, Itertools};
use pathfinding::prelude::dijkstra;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut map = BTreeMap::new();
    let mut zeros = BTreeSet::new();
    let mut nines = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            map.insert(p, b - b'0');
            if b == b'0' {
                zeros.insert(p);
            } else if b == b'9' {
                nines.insert(p);
            }
        }
    }
    let mut score = 0;
    for (zero, nine) in iproduct!(zeros.iter(), nines.iter()) {
        let path = dijkstra(
            zero,
            |p| {
                let value = map.get(p).expect("No point found");
                p.neighbors()
                    .iter()
                    .filter(|n| map.get(*n) == Some(&(value + 1)))
                    .map(|n| (*n, 1))
                    .collect_vec()
            },
            |p| p == nine,
        );
        if path.is_some() {
            score += 1;
        }
    }
    println!("{}", score);
}

fn rating(point: &Point<i64>, map: &BTreeMap<Point<i64>, u8>) -> Option<u64> {
    let value = *map.get(point)?;
    if value == 9 {
        return Some(1);
    }
    return point
        .neighbors()
        .iter()
        .filter(|n| map.get(*n) == Some(&(value + 1)))
        .map(|n| rating(n, map))
        .sum();
}

pub fn part2(input: String) {
    let mut map = BTreeMap::new();
    let mut zeros = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            map.insert(p, b - b'0');
            if b == b'0' {
                zeros.insert(p);
            }
        }
    }

    zeros
        .iter()
        .flat_map(|z| rating(z, &map))
        .sum::<u64>()
        .print();
}
