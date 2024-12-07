use std::collections::HashSet;

use crate::point::Point;

fn visited_before_leaving(
    obstructions: &HashSet<Point<i64>>,
    start: &Point<i64>,
    max: (i64, i64),
) -> usize {
    let mut loop_detector = HashSet::new();
    let mut visited = HashSet::new();
    let mut guard = start.clone();
    let mut direction = Point { x: 0i64, y: -1 };
    while guard.x >= 0 && guard.x <= max.0 && guard.y >= 0 && guard.y <= max.1 {
        if loop_detector.contains(&(guard, direction)) {
            return usize::MAX;
        }
        loop_detector.insert((guard, direction));
        visited.insert(guard);
        let mut next = Point {
            x: guard.x + direction.x,
            y: guard.y + direction.y,
        };
        while obstructions.contains(&next) {
            (direction.x, direction.y) = match (direction.x, direction.y) {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Invalid direction!"),
            };
            next = Point {
                x: guard.x + direction.x,
                y: guard.y + direction.y,
            };
        }
        guard = next;
    }
    visited.len()
}

pub fn part1(input: String) {
    let (mut max_x, mut max_y) = (i64::MIN, i64::MIN);
    let mut obstructions = HashSet::new();
    let mut guard = Point {
        x: i64::MIN,
        y: i64::MIN,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            match c {
                '#' => _ = obstructions.insert(p),
                '^' => guard = p,
                _ => (),
            }
            if p.x > max_x {
                max_x = p.x;
            }
        }
        if y as i64 > max_y {
            max_y = y as i64;
        }
    }

    println!(
        "{}",
        visited_before_leaving(&obstructions, &guard, (max_x, max_y))
    );

    let mut count = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            if obstructions.contains(&p) {
                continue;
            } else if guard == p {
                continue;
            }
            let mut new_obstructions = obstructions.clone();
            new_obstructions.insert(p);
            if visited_before_leaving(&new_obstructions, &guard, (max_x, max_y)) == usize::MAX {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
