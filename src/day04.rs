use std::collections::HashMap;

use pathfinding::num_traits::ToPrimitive;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut map = HashMap::new();
    let mut x_locations = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x.to_i64().expect("Could not convert x to i64"),
                y: y.to_i64().expect("Could not convert y to i64"),
            };
            map.insert(p, c);
            if c == 'X' {
                x_locations.push(p);
            }
        }
    }
    let search = ['M', 'A', 'S'];
    x_locations
        .iter()
        .map(|origin| {
            let mut total = 0;
            for dy in -1..=1 {
                'outer: for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let mut p = Point {
                        x: origin.x + dx,
                        y: origin.y + dy,
                    };
                    for i in 0..3 {
                        if map.get(&p) != Some(&search[i]) {
                            continue 'outer;
                        }
                        p = Point {
                            x: p.x + dx,
                            y: p.y + dy,
                        };
                    }
                    total += 1;
                }
            }
            total
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let mut map = HashMap::new();
    let mut a_locations = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x.to_i64().expect("Could not convert x to i64"),
                y: y.to_i64().expect("Could not convert y to i64"),
            };
            map.insert(p, c);
            if c == 'A' {
                a_locations.push(p);
            }
        }
    }
    a_locations
        .iter()
        .map(|origin| {
            let mut total = 0;
            if map.get(&origin.up().left()) == Some(&'M')
                && map.get(&origin.down().left()) == Some(&'M')
                && map.get(&origin.up().right()) == Some(&'S')
                && map.get(&origin.down().right()) == Some(&'S')
            {
                total += 1;
            }
            if map.get(&origin.up().left()) == Some(&'M')
                && map.get(&origin.up().right()) == Some(&'M')
                && map.get(&origin.down().left()) == Some(&'S')
                && map.get(&origin.down().right()) == Some(&'S')
            {
                total += 1;
            }
            if map.get(&origin.up().right()) == Some(&'M')
                && map.get(&origin.down().right()) == Some(&'M')
                && map.get(&origin.up().left()) == Some(&'S')
                && map.get(&origin.down().left()) == Some(&'S')
            {
                total += 1;
            }
            if map.get(&origin.down().right()) == Some(&'M')
                && map.get(&origin.down().left()) == Some(&'M')
                && map.get(&origin.up().left()) == Some(&'S')
                && map.get(&origin.up().right()) == Some(&'S')
            {
                total += 1;
            }
            total
        })
        .sum::<u64>()
        .print();
}
