use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
};

use itertools::{iproduct, Itertools};
use pathfinding::prelude::astar;

use crate::{point::Point, selfprint::SelfPrint};

fn path(
    walls: &BTreeSet<Point<i64>>,
    start: &Point<i64>,
    end: &Point<i64>,
) -> Option<(Vec<Point<i64>>, i64)> {
    astar(
        start,
        |p| {
            p.neighbors()
                .iter()
                .filter(|n| !walls.contains(*n))
                .map(|n| (*n, 1))
                .collect_vec()
        },
        |p| p.manhattan_distance(end),
        |p| p == end,
    )
}

#[allow(dead_code)]
fn print_it(
    walls: &BTreeSet<Point<i64>>,
    start: &Point<i64>,
    end: &Point<i64>,
    max_x: i64,
    max_y: i64,
    neighbor: &Point<i64>,
    wall_neighbor: &Point<i64>,
) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            if neighbor == &p {
                print!("1");
            } else if wall_neighbor == &p {
                print!("2");
            } else if start == &p {
                print!("S");
            } else if end == &p {
                print!("E");
            } else if walls.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

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

    let (original_path, cost_without_cheating) =
        path(&walls, &start, &end).expect("No original path found");

    let costs = original_path
        .iter()
        .rev()
        .enumerate()
        .map(|(i, p)| (*p, i as i64))
        .collect::<BTreeMap<_, _>>();

    let mut best_cheats = BTreeMap::<i64, u64>::new();
    let mut steps = 0;
    for point in original_path {
        for (y, x) in iproduct!(-2..=2, -2..=2) {
            let possible_end = point + Point { x, y };
            let distance = point.manhattan_distance(&possible_end);
            if distance > 2 {
                continue;
            }

            if let Some(cost) = costs.get(&possible_end) {
                let time_saved = max(cost_without_cheating - (*cost + steps + distance), 0);
                *best_cheats.entry(time_saved).or_default() += 1;
            }
        }
        steps += 1;
    }
    best_cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, c)| *c)
        .sum::<u64>()
        .print();
}

/*
fn find_best_cheats(
    walls: &BTreeSet<Point<i64>>,
    max_x: i64,
    max_y: i64,
    seen: BTreeSet<Point<i64>>,
    start: &Point<i64>,
    end: &Point<i64>,
    steps_taken: i64,
    max_steps: i64,
    cheat_start: Option<Point<i64>>,
    cheat_end: Option<Point<i64>>,
    best_cheats: &mut BTreeSet<(Point<i64>, Point<i64>)>,
) {
    if start.x < 0 || start.x > max_x || start.y < 0 || start.y > max_y {
        return;
    }
    if start == end {
        let cheat_start = cheat_start.unwrap_or(*end);
        let cheat_end = cheat_end.unwrap_or(*end);
        if steps_taken <= max_steps {
            best_cheats.insert((cheat_start, cheat_end));
        }
        return;
    }
    if steps_taken > max_steps {
        return;
    }
    for neighbor in start.neighbors() {
        if seen.contains(&neighbor) {
            continue;
        }
        let mut seen = seen.clone();
        seen.insert(neighbor);

        if let Some(cheat_start) = cheat_start {
            if cheat_end.is_some() {
                if walls.contains(&neighbor) {
                    continue;
                }
                find_best_cheats(
                    walls,
                    max_x,
                    max_y,
                    seen.clone(),
                    &neighbor,
                    end,
                    steps_taken + 1,
                    max_steps,
                    Some(cheat_start),
                    cheat_end,
                    best_cheats,
                );
            } else {
                if walls.contains(&neighbor) {
                    if cheat_start.manhattan_distance(&neighbor) < 20 {
                        find_best_cheats(
                            walls,
                            max_x,
                            max_y,
                            seen.clone(),
                            &neighbor,
                            end,
                            steps_taken + 1,
                            max_steps,
                            Some(cheat_start),
                            cheat_end,
                            best_cheats,
                        );
                    }
                } else {
                    // Try ending the cheat
                    if !best_cheats.contains(&(cheat_start, neighbor)) {
                        find_best_cheats(
                            walls,
                            max_x,
                            max_y,
                            seen.clone(),
                            &neighbor,
                            end,
                            steps_taken + 1,
                            max_steps,
                            Some(cheat_start),
                            Some(neighbor),
                            best_cheats,
                        );
                    }

                    // Or continuing the cheat
                    if cheat_start.manhattan_distance(&neighbor) < 20 {
                        find_best_cheats(
                            walls,
                            max_x,
                            max_y,
                            seen.clone(),
                            &neighbor,
                            end,
                            steps_taken + 1,
                            max_steps,
                            Some(cheat_start),
                            cheat_end,
                            best_cheats,
                        );
                    }
                }
            }
        } else {
            // Haven't started cheating. Either continue not cheating or start cheating
            // find_best_cheats(
            //     walls,
            //     max_x,
            //     max_y,
            //     seen.clone(),
            //     &neighbor,
            //     end,
            //     steps_taken + 1,
            //     max_steps,
            //     Some(neighbor),
            //     cheat_end,
            //     best_cheats,
            // );
            if !walls.contains(&neighbor) {
                find_best_cheats(
                    walls,
                    max_x,
                    max_y,
                    seen.clone(),
                    &neighbor,
                    end,
                    steps_taken + 1,
                    max_steps,
                    cheat_start,
                    cheat_end,
                    best_cheats,
                );
            } else {
                find_best_cheats(
                    walls,
                    max_x,
                    max_y,
                    seen.clone(),
                    &neighbor,
                    end,
                    steps_taken + 1,
                    max_steps,
                    Some(*start),
                    cheat_end,
                    best_cheats,
                );
            }
        }
    }
}
*/

pub fn part2(input: String) {
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

    let (original_path, cost_without_cheating) =
        path(&walls, &start, &end).expect("No original path found");

    let costs = original_path
        .iter()
        .rev()
        .enumerate()
        .map(|(i, p)| (*p, i as i64))
        .collect::<BTreeMap<_, _>>();

    let mut best_cheats = BTreeMap::<i64, u64>::new();
    let mut steps = 0;
    for point in original_path {
        for (y, x) in iproduct!(-20..=20i64, -20..=20i64) {
            let possible_end = point + Point { x, y };
            let distance = point.manhattan_distance(&possible_end);
            if distance > 20 {
                continue;
            }

            if let Some(cost) = costs.get(&possible_end) {
                let time_saved = max(cost_without_cheating - (*cost + steps + distance), 0);
                *best_cheats.entry(time_saved).or_default() += 1;
            }
        }
        steps += 1;
    }
    best_cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, c)| *c)
        .sum::<u64>()
        .print();
}
