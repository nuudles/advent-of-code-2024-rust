use std::collections::BTreeSet;

use itertools::Itertools;
use pathfinding::prelude::astar_bag;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut walls = BTreeSet::new();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            match c {
                '#' => _ = walls.insert(p),
                'S' => start = p,
                'E' => end = p,
                _ => (),
            }
        }
    }
    let (solution, cost) = astar_bag(
        &(start, Point { x: 1i64, y: 0 }),
        |(p, d)| {
            let mut options = vec![];
            if !walls.contains(&(*p + *d)) {
                options.push(((*p + *d, *d), 1i64));
            }
            if d.x == 0 {
                options.push(((*p, Point { x: -1, y: 0 }), 1000));
                options.push(((*p, Point { x: 1, y: 0 }), 1000));
            } else {
                options.push(((*p, Point { x: 0, y: -1 }), 1000));
                options.push(((*p, Point { x: 0, y: 1 }), 1000));
            }
            options
        },
        |(p, _)| p.manhattan_distance(&end),
        |(p, _)| *p == end,
    )
    .expect("No solution found");

    cost.print();

    solution
        .flat_map(|s| s.iter().map(|(p, _)| *p).collect_vec())
        .collect::<BTreeSet<_>>()
        .len()
        .print();
}
