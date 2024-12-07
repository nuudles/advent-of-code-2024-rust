use regex::Regex;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid Regex");
    regex
        .captures_iter(&input)
        .map(|capture| {
            let (_, [x, y]) = capture.extract();
            x.parse::<u64>().unwrap_or_default() * y.parse::<u64>().unwrap_or_default()
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don\'t\(\))").expect("Invalid Regex");
    let mut enabled = true;
    regex
        .captures_iter(&input)
        .map(|capture| {
            if !capture.get(3).is_none() {
                enabled = true;
                0
            } else if !capture.get(4).is_none() {
                enabled = false;
                0
            } else if enabled {
                let x = capture
                    .get(1)
                    .and_then(|m| m.as_str().parse::<u64>().ok())
                    .unwrap_or_default();
                let y = capture
                    .get(2)
                    .and_then(|m| m.as_str().parse::<u64>().ok())
                    .unwrap_or_default();
                x * y
            } else {
                0
            }
        })
        .sum::<u64>()
        .print();
}
