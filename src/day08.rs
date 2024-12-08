use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let (mut max_x, mut max_y) = (i64::MIN, i64::MIN);
    let mut by_freq = BTreeMap::<char, Vec<Point<i64>>>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let p = Point {
                    x: x as i64,
                    y: y as i64,
                };
                by_freq.entry(c).or_default().push(p);
            }
            if x as i64 > max_x {
                max_x = x as i64;
            }
        }
        if y as i64 > max_y {
            max_y = y as i64;
        }
    }

    let mut antinodes = BTreeSet::new();
    let mut part_2 = BTreeSet::new();
    for (_, nodes) in by_freq {
        nodes.iter().tuple_combinations().for_each(|(a, b)| {
            let delta = Point {
                x: b.x - a.x,
                y: b.y - a.y,
            };

            for p in [*a - delta, *b + delta] {
                if p.x < 0 || p.x > max_x || p.y < 0 || p.y > max_y {
                    continue;
                }
                antinodes.insert(p);
            }
            let mut p = *a;
            while p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y {
                part_2.insert(p);
                p = p - delta;
            }
            p = *b;
            while p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y {
                part_2.insert(p);
                p = p + delta;
            }
        });
    }
    antinodes.len().print();
    part_2.len().print();
}
