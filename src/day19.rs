use std::collections::{HashMap, HashSet};

use crate::selfprint::SelfPrint;

fn is_possible(string: &str, patterns: &HashSet<&str>, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(is_possible) = cache.get(string) {
        return *is_possible;
    }

    if string.len() == 0 {
        return true;
    }

    let is_possible = patterns.iter().any(|pattern| {
        if string.starts_with(*pattern) {
            is_possible(&string[pattern.len()..], patterns, cache)
        } else {
            false
        }
    });
    cache.insert(string.to_string(), is_possible);
    is_possible
}

pub fn part1(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("Separator not found");
    let patterns = top.split(", ").collect::<HashSet<_>>();
    let mut cache = HashMap::new();

    bottom
        .lines()
        .filter(|s| is_possible(*s, &patterns, &mut cache))
        .count()
        .print();
}

fn possibility_count(
    string: &str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(count) = cache.get(string) {
        return *count;
    }

    if string.len() == 0 {
        return 1;
    }

    let count = patterns
        .iter()
        .map(|pattern| {
            if string.starts_with(*pattern) {
                possibility_count(&string[pattern.len()..], patterns, cache)
            } else {
                0
            }
        })
        .sum::<usize>();
    cache.insert(string.to_string(), count);
    count
}

pub fn part2(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("Separator not found");
    let patterns = top.split(", ").collect::<HashSet<_>>();
    let mut cache = HashMap::new();

    bottom
        .lines()
        .map(|s| possibility_count(s, &patterns, &mut cache))
        .sum::<usize>()
        .print();
}
