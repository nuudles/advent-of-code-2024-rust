use std::collections::{BTreeMap, BTreeSet};

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

fn find_numeric_paths(code: &str, current: char) -> BTreeSet<String> {
    let coords = BTreeMap::from([
        ('7', Point { x: 0i64, y: 0 }),
        ('8', Point { x: 1, y: 0 }),
        ('9', Point { x: 2, y: 0 }),
        ('4', Point { x: 0, y: 1 }),
        ('5', Point { x: 1, y: 1 }),
        ('6', Point { x: 2, y: 1 }),
        ('1', Point { x: 0, y: 2 }),
        ('2', Point { x: 1, y: 2 }),
        ('3', Point { x: 2, y: 2 }),
        ('0', Point { x: 1, y: 3 }),
        ('A', Point { x: 2, y: 3 }),
    ]);
    let buttons = BTreeMap::from([
        (Point { x: 0i64, y: 0 }, '7'),
        (Point { x: 1, y: 0 }, '8'),
        (Point { x: 2, y: 0 }, '9'),
        (Point { x: 0, y: 1 }, '4'),
        (Point { x: 1, y: 1 }, '5'),
        (Point { x: 2, y: 1 }, '6'),
        (Point { x: 0, y: 2 }, '1'),
        (Point { x: 1, y: 2 }, '2'),
        (Point { x: 2, y: 2 }, '3'),
        (Point { x: 1, y: 3 }, '0'),
        (Point { x: 2, y: 3 }, 'A'),
    ]);

    if code.is_empty() {
        return BTreeSet::from(["".to_string()]);
    }

    let target = code.chars().next().expect("No target found");
    if target == current {
        let mut set = BTreeSet::new();
        for path in find_numeric_paths(&code[1..], current) {
            set.insert("A".to_string() + &path);
        }
        return set;
    }

    let target_coord = coords.get(&target).expect("Target coord not found");
    let current_coord = coords.get(&current).expect("Current coord not found");

    let mut set = BTreeSet::new();
    if target_coord.x > current_coord.x {
        if let Some(right) = buttons.get(&current_coord.right()) {
            for path in find_numeric_paths(code, *right) {
                set.insert(">".to_string() + &path);
            }
        }
    }
    if target_coord.y > current_coord.y {
        if let Some(down) = buttons.get(&current_coord.down()) {
            for path in find_numeric_paths(code, *down) {
                set.insert("v".to_string() + &path);
            }
        }
    }
    if target_coord.y < current_coord.y {
        if let Some(up) = buttons.get(&current_coord.up()) {
            for path in find_numeric_paths(code, *up) {
                set.insert("^".to_string() + &path);
            }
        }
    }
    if target_coord.x < current_coord.x {
        if let Some(left) = buttons.get(&current_coord.left()) {
            for path in find_numeric_paths(code, *left) {
                set.insert("<".to_string() + &path);
            }
        }
    }
    set
}

fn min_direction_path(
    pattern: String,
    depth: u64,
    cache: &mut BTreeMap<(String, u64), u64>,
) -> u64 {
    let mapping = BTreeMap::from([
        (('A', '<'), vec!["v<<A", "<v<A"]),
        (('<', 'A'), vec![">>^A", ">^>A"]),
        (('A', '^'), vec!["<A"]),
        (('^', 'A'), vec![">A"]),
        (('A', 'v'), vec!["<vA", "v<A"]),
        (('v', 'A'), vec![">^A", "^>A"]),
        (('A', '>'), vec!["vA"]),
        (('>', 'A'), vec!["^A"]),
        (('^', '<'), vec!["v<A"]),
        (('<', '^'), vec![">^A"]),
        (('^', 'v'), vec!["vA"]),
        (('v', '^'), vec!["^A"]),
        (('^', '>'), vec![">vA", "v>A"]),
        (('>', '^'), vec!["<^A", "^<A"]),
        (('<', 'v'), vec![">A"]),
        (('v', '<'), vec!["<A"]),
        (('<', '>'), vec![">>A"]),
        (('>', '<'), vec!["<<A"]),
        (('v', '>'), vec![">A"]),
        (('>', 'v'), vec!["<A"]),
    ]);

    if let Some(length) = cache.get(&(pattern.clone(), depth)) {
        return *length;
    }

    let mut length = 0;
    let mut current = 'A';
    for target in pattern.chars() {
        if current == target {
            length += 1;
            continue;
        }

        let possibilities = mapping.get(&(current, target)).expect("No mapping found");
        if depth > 1 {
            length += possibilities
                .iter()
                .map(|p| min_direction_path(p.to_string(), depth - 1, cache))
                .min()
                .expect("Min possibility not found");
        } else {
            length += possibilities
                .iter()
                .map(|p| p.len() as u64)
                .min()
                .expect("Min possibility not found at depth 0");
        }
        current = target;
    }
    cache.insert((pattern, depth), length);
    length
}

pub fn part1(input: String) {
    let mut cache = BTreeMap::new();

    input
        .lines()
        .map(|l| {
            let paths = find_numeric_paths(l, 'A');

            let min = paths
                .iter()
                .map(|p| min_direction_path(p.to_string(), 2, &mut cache))
                .min()
                .expect("No min found");

            min * parse_nums::<u64>(l).next().expect("First number not found")
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let mut cache = BTreeMap::new();

    input
        .lines()
        .map(|l| {
            let paths = find_numeric_paths(l, 'A');

            let min = paths
                .iter()
                .map(|p| min_direction_path(p.to_string(), 25, &mut cache))
                .min()
                .expect("No min found");

            min * parse_nums::<u64>(l).next().expect("First number not found")
        })
        .sum::<u64>()
        .print();
}

/*
Older attempts commented out

fn find_direction_path(code: &str, current: char) -> BTreeSet<String> {
    let coords = BTreeMap::from([
        ('^', Point { x: 1i64, y: 0 }),
        ('A', Point { x: 2, y: 0 }),
        ('<', Point { x: 0, y: 1 }),
        ('v', Point { x: 1, y: 1 }),
        ('>', Point { x: 2, y: 1 }),
    ]);
    let buttons = BTreeMap::from([
        (Point { x: 1i64, y: 0 }, '^'),
        (Point { x: 2, y: 0 }, 'A'),
        (Point { x: 0, y: 1 }, '<'),
        (Point { x: 1, y: 1 }, 'v'),
        (Point { x: 2, y: 1 }, '>'),
    ]);

    if code.is_empty() {
        return BTreeSet::from(["".to_string()]);
    }

    let target = code.chars().next().expect("No target found");
    if target == current {
        let mut set = BTreeSet::new();
        for path in find_direction_path(&code[1..], current) {
            set.insert("A".to_string() + &path);
        }
        return set;
    }

    let target_coord = coords.get(&target).expect("Target coord not found");
    let current_coord = coords.get(&current).expect("Current coord not found");

    let mut set = BTreeSet::new();
    if target_coord.x > current_coord.x {
        if let Some(right) = buttons.get(&current_coord.right()) {
            for path in find_direction_path(code, *right) {
                set.insert(">".to_string() + &path);
            }
        }
    }
    if target_coord.y > current_coord.y {
        if let Some(down) = buttons.get(&current_coord.down()) {
            for path in find_direction_path(code, *down) {
                set.insert("v".to_string() + &path);
            }
        }
    }
    if target_coord.y < current_coord.y {
        if let Some(up) = buttons.get(&current_coord.up()) {
            for path in find_direction_path(code, *up) {
                set.insert("^".to_string() + &path);
            }
        }
    }
    if target_coord.x < current_coord.x {
        if let Some(left) = buttons.get(&current_coord.left()) {
            for path in find_direction_path(code, *left) {
                set.insert("<".to_string() + &path);
            }
        }
    }
    set
}


fn find_direction_path(code: &str, current: char) -> BTreeSet<String> {
    let mapping = BTreeMap::from([
        (('A', '<'), "v<<A"),
        (('<', 'A'), ">>^A"),
        (('A', '^'), "<A"),
        (('^', 'A'), ">A"),
        (('A', 'v'), "<vA"),
        (('v', 'A'), ">^A"),
        (('A', '>'), "vA"),
        (('>', 'A'), "^A"),
        (('^', '<'), "v<A"),
        (('<', '^'), ">^A"),
        (('^', 'v'), "vA"),
        (('v', '^'), "^A"),
        (('^', '>'), ">vA"),
        (('>', '^'), "<^A"),
        (('<', 'v'), ">A"),
        (('v', '<'), "<A"),
        (('<', '>'), ">>A"),
        (('>', '<'), "<<A"),
        (('v', '>'), ">A"),
        (('>', 'v'), "<A"),
    ]);
    let mut set = BTreeSet::new();
    let mut string = String::new();
    let mut current = current;
    for target in code.chars() {
        if current == target {
            string += "A";
            continue;
        }

        let next = mapping.get(&(current, target)).expect("No mapping found");
        string += next;
        current = target;
    }
    // println!("{} {}", code, string);
    set.insert(string);
    set
}

fn find_direction_path(sequences: BTreeMap<String, u64>) -> BTreeMap<String, u64> {
    let mapping = BTreeMap::from([
        (('A', '<'), "v<<A"),
        (('<', 'A'), ">>^A"),
        (('A', '^'), "<A"),
        (('^', 'A'), ">A"),
        (('A', 'v'), "<vA"),
        (('v', 'A'), ">^A"),
        (('A', '>'), "vA"),
        (('>', 'A'), "^A"),
        (('^', '<'), "v<A"),
        (('<', '^'), ">^A"),
        (('^', 'v'), "vA"),
        (('v', '^'), "^A"),
        (('^', '>'), ">vA"),
        (('>', '^'), "<^A"),
        (('<', 'v'), ">A"),
        (('v', '<'), "<A"),
        (('<', '>'), ">>A"),
        (('>', '<'), "<<A"),
        (('v', '>'), ">A"),
        (('>', 'v'), "<A"),
    ]);
    let mut result = BTreeMap::new();

    for (code, count) in sequences {
        let mut current = 'A';
        for target in code.chars() {
            if current == target {
                *result.entry("A".to_string()).or_default() += count;
                continue;
            }
            let replacement = mapping.get(&(current, target)).expect("No mapping found");
            *result.entry(replacement.to_string()).or_default() += count;
            current = target;
        }
    }

    result
}

fn find_direction_path(sequences: BTreeMap<String, u64>) -> BTreeSet<BTreeMap<String, u64>> {
    let mapping = BTreeMap::from([
        (('A', '<'), vec!["v<<A"]), //vec!["v<<A", "<v<A"]),
        (('<', 'A'), vec![">>^A", ">^>A"]),
        (('A', '^'), vec!["<A"]),
        (('^', 'A'), vec![">A"]),
        (('A', 'v'), vec!["<vA", "v<A"]),
        (('v', 'A'), vec![">^A", "^>A"]),
        (('A', '>'), vec!["vA"]),
        (('>', 'A'), vec!["^A"]),
        (('^', '<'), vec!["v<A"]),
        (('<', '^'), vec![">^A"]),
        (('^', 'v'), vec!["vA"]),
        (('v', '^'), vec!["^A"]),
        (('^', '>'), vec![">vA", "v>A"]),
        (('>', '^'), vec!["<^A", "^<A"]),
        (('<', 'v'), vec![">A"]),
        (('v', '<'), vec!["<A"]),
        (('<', '>'), vec![">>A"]),
        (('>', '<'), vec!["<<A"]),
        (('v', '>'), vec![">A"]),
        (('>', 'v'), vec!["<A"]),
    ]);
    let mut result = BTreeSet::from([BTreeMap::<String, u64>::new()]);

    for (code, count) in sequences {
        let mut current = 'A';
        for target in code.chars() {
            if current == target {
                result = result
                    .iter()
                    .map(|m| {
                        let mut map = m.clone();
                        *map.entry("A".to_string()).or_default() += count;
                        map
                    })
                    .collect();
                continue;
            }
            let possibilities = mapping.get(&(current, target)).expect("No mapping found");
            if possibilities.len() == 1 {
                result = result
                    .iter()
                    .map(|m| {
                        let mut map = m.clone();
                        *map.entry(possibilities[0].to_string()).or_default() += count;
                        map
                    })
                    .collect();
            } else {
                result = result
                    .iter()
                    .flat_map(|m| {
                        let mut set = BTreeSet::new();
                        for possibility in possibilities {
                            let mut map = m.clone();
                            *map.entry(possibility.to_string()).or_default() += count;
                            set.insert(map);
                        }
                        set
                    })
                    .collect();
            }
            current = target;
        }
    }

    result
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|l| {
            let paths = find_numeric_path(l, 'A');
            let min = paths
                .iter()
                .map(|p| {
                    let mut sequences = BTreeSet::from([BTreeMap::from([(p.to_string(), 1u64)])]);

                    for i in 0..25 {
                        println!("{} {} {}", l, i, sequences.len());
                        sequences = sequences
                            .iter()
                            .flat_map(|sequence| find_direction_path(sequence.clone()))
                            .collect();
                        let min = sequences
                            .iter()
                            .map(|m| m.iter().map(|(k, v)| k.len() as u64 * *v).sum::<u64>())
                            .min()
                            .unwrap();
                        sequences = sequences
                            .iter()
                            .filter(|m| {
                                m.iter().map(|(k, v)| k.len() as u64 * *v).sum::<u64>() == min
                            })
                            .map(|m| m.clone())
                            .collect();
                    }
                    sequences
                        .iter()
                        .map(|sequence| {
                            sequence
                                .iter()
                                .map(|(k, v)| k.len() as u64 * *v)
                                .sum::<u64>()
                        })
                        .min()
                        .expect("ALWEKLAWKE")
                })
                .min()
                .expect("No min found");

            min * parse_nums::<u64>(l).next().expect("First number not found")
        })
        .sum::<u64>()
        .print();
}
*/
