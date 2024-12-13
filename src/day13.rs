use itertools::{iproduct, Itertools};

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let (ax, ay) = parse_nums::<u64>(lines.next().expect("A line not found"))
                .next_tuple()
                .expect("Could not parse A");
            let (bx, by) = parse_nums::<u64>(lines.next().expect("B line not found"))
                .next_tuple()
                .expect("Could not parse B");
            let (px, py) = parse_nums::<u64>(lines.next().expect("Prize line not found"))
                .next_tuple()
                .expect("Could not parse Prize");
            let min = iproduct!(0..=100, 0..=100)
                .map(|(a, b)| {
                    if a * ax + b * bx == px && a * ay + b * by == py {
                        return a * 3 + b;
                    } else {
                        return u64::MAX;
                    }
                })
                .min()
                .expect("No min found");
            if min == u64::MAX {
                return 0;
            } else {
                min
            }
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let (ax, ay) = parse_nums::<f64>(lines.next().expect("A line not found"))
                .next_tuple()
                .expect("Could not parse A");
            let (bx, by) = parse_nums::<f64>(lines.next().expect("B line not found"))
                .next_tuple()
                .expect("Could not parse B");
            let (mut px, mut py) = parse_nums::<f64>(lines.next().expect("Prize line not found"))
                .next_tuple()
                .expect("Could not parse Prize");
            px += 10000000000000f64;
            py += 10000000000000f64;

            let a = ((py - px * by / bx) / (ay - ax * by / bx)).round();
            let b = ((px - a * ax) / bx).round();
            if ax * a + bx * b == px && ay * a + by * b == py {
                return a as i64 * 3 + b as i64;
            } else {
                return 0;
            }
        })
        .sum::<i64>()
        .print();
}
