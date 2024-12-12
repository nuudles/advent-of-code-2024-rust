use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

fn region_area_and_perimeter(
    point: &Point<i64>,
    target: &char,
    seen: &mut BTreeSet<Point<i64>>,
    map: &BTreeMap<Point<i64>, char>,
) -> (u64, u64) {
    let (mut area, mut perimeter) = (
        1,
        point
            .neighbors()
            .iter()
            .filter(|n| map.get(*n) != Some(target))
            .count() as u64,
    );
    for neighbor in point.neighbors() {
        if seen.contains(&neighbor) {
            continue;
        } else if map.get(&neighbor) != Some(target) {
            continue;
        }
        seen.insert(neighbor);
        let (na, np) = region_area_and_perimeter(&neighbor, target, seen, map);
        area += na;
        perimeter += np;
    }
    (area, perimeter)
}

pub fn part1(input: String) {
    let mut map = BTreeMap::new();
    for (y, lines) in input.lines().enumerate() {
        for (x, c) in lines.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i64,
                    y: y as i64,
                },
                c,
            );
        }
    }

    let mut cost = 0;

    let mut seen = BTreeSet::new();
    for (p, c) in &map {
        if seen.contains(p) {
            continue;
        }
        seen.insert(*p);
        let (area, perimeter) = region_area_and_perimeter(p, c, &mut seen, &map);
        cost += area * perimeter;
    }
    cost.print();
}

fn region_area(
    point: &Point<i64>,
    target: &char,
    seen: &mut BTreeSet<Point<i64>>,
    fences: &mut BTreeMap<Point<i64>, BTreeSet<Point<i64>>>,
    map: &BTreeMap<Point<i64>, char>,
) -> u64 {
    let mut area = 1;
    for neighbor in point.neighbors() {
        if map.get(&neighbor) != Some(target) {
            fences.entry(neighbor - *point).or_default().insert(*point);
            continue;
        }

        if seen.contains(&neighbor) {
            continue;
        }
        seen.insert(neighbor);
        area += region_area(&neighbor, target, seen, fences, map);
    }
    area
}

fn fences_to_sides(fences: &BTreeMap<Point<i64>, BTreeSet<Point<i64>>>) -> u64 {
    let mut sides = 0;
    for (delta, points) in fences {
        let is_horizontal = delta.x == 0;
        for (_, group) in &points
            .iter()
            .into_group_map_by(|p| if is_horizontal { p.y } else { p.x })
        {
            let mut last = i64::MAX;
            for p in group
                .iter()
                .sorted_by_key(|p| if is_horizontal { p.x } else { p.y })
            {
                let relevant = if is_horizontal { p.x } else { p.y };
                if relevant - 1 != last {
                    sides += 1;
                }
                last = relevant;
            }
        }
    }
    sides
}

pub fn part2(input: String) {
    let mut map = BTreeMap::new();
    for (y, lines) in input.lines().enumerate() {
        for (x, c) in lines.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i64,
                    y: y as i64,
                },
                c,
            );
        }
    }

    let mut cost = 0;

    let mut seen = BTreeSet::new();
    for (p, c) in &map {
        if seen.contains(p) {
            continue;
        }
        seen.insert(*p);
        let mut fences = BTreeMap::new();
        let area = region_area(p, c, &mut seen, &mut fences, &map);
        let sides = fences_to_sides(&fences);
        cost += area * sides;
    }
    cost.print();
}
