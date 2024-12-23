use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let mut connections = BTreeMap::<&str, BTreeSet<&str>>::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").expect("- not found");
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }

    let mut found = BTreeSet::new();
    for (key, set) in &connections {
        for (a, b) in set.iter().tuple_combinations() {
            if !key.starts_with('t') && !a.starts_with('t') && !b.starts_with('t') {
                continue;
            }

            let a_connections = connections.get(*a).expect("A not found");
            let b_connections = connections.get(*b).expect("B not found");

            if a_connections.contains(b) && b_connections.contains(a) {
                found.insert(BTreeSet::from([*key, *a, *b]));
            }
        }
    }
    found.len().print();
}

pub fn part2(input: String) {
    let mut connections = BTreeMap::<&str, BTreeSet<&str>>::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").expect("- not found");
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }

    let max_count = connections
        .values()
        .map(|s| s.len())
        .max()
        .expect("Max count not found");

    let mut found = BTreeSet::new();
    'length: for length in 3..max_count {
        for (key, set) in &connections {
            for combination in set.iter().combinations(length) {
                if combination.iter().any(|other| {
                    let other_connections = connections
                        .get(*other)
                        .expect("Other connections not found");
                    combination
                        .iter()
                        .any(|s| s != other && !other_connections.contains(*s))
                }) {
                    continue;
                }
                let mut result = BTreeSet::new();
                result.insert(*key);
                for s in combination {
                    result.insert(*s);
                }
                found.insert(result);
                continue 'length;
            }
        }
    }
    found
        .iter()
        .max_by_key(|s| s.len())
        .expect("No max found")
        .iter()
        .sorted()
        .join(",")
        .print();
}
