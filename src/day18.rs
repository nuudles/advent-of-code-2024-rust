use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    // let (width, height) = (7i64, 7i64);
    let (width, height) = (71i64, 71i64);
    let mut corrupted = HashSet::new();
    for (x, y) in parse_nums::<i64>(&input).tuples().take(1024) {
        corrupted.insert(Point { x, y });
    }
    let end = Point {
        x: width - 1,
        y: height - 1,
    };
    astar(
        &Point { x: 0, y: 0 },
        |p| {
            p.neighbors()
                .iter()
                .filter(|n| {
                    !corrupted.contains(n) && n.x >= 0 && n.y >= 0 && n.x < width && n.y < height
                })
                .map(|n| (*n, 1))
                .collect_vec()
        },
        |p| p.manhattan_distance(&end),
        |p| *p == end,
    )
    .expect("No path found")
    .1
    .print();
}

pub fn part2(input: String) {
    // let (width, height) = (7i64, 7i64);
    let (width, height) = (71i64, 71i64);
    let mut corrupted = HashSet::new();
    let end = Point {
        x: width - 1,
        y: height - 1,
    };
    let mut best_path = HashSet::new();
    for (x, y) in parse_nums::<i64>(&input).tuples() {
        let p = Point { x, y };
        corrupted.insert(p);

        if best_path.len() == 0 || best_path.contains(&p) {
            if let Some(path) = astar(
                &Point { x: 0, y: 0 },
                |p| {
                    p.neighbors()
                        .iter()
                        .filter(|n| {
                            !corrupted.contains(n)
                                && n.x >= 0
                                && n.y >= 0
                                && n.x < width
                                && n.y < height
                        })
                        .map(|n| (*n, 1))
                        .collect_vec()
                },
                |p| p.manhattan_distance(&end),
                |p| *p == end,
            ) {
                best_path = path.0.iter().cloned().collect();
            } else {
                println!("{},{}", x, y);
                break;
            }
        }
    }
}
