use itertools::Itertools;

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

fn dump(robots: &Vec<(Point<i64>, Point<i64>)>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            if robots.iter().any(|(p, _)| p.x == x && p.y == y) {
                print!("1");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part1(input: String) {
    // let (width, height) = (11i64, 7i64);
    let (width, height) = (101, 103);
    let mut robots = input
        .lines()
        .map(|line| {
            let (px, py, vx, vy) = parse_nums::<i64>(line)
                .collect_tuple()
                .expect("Error parsing line");
            (Point { x: px, y: py }, Point { x: vx, y: vy })
        })
        .collect_vec();
    for _ in 0..100 {
        robots = robots
            .iter()
            .map(|(p, v)| {
                let mut n = *p + *v;
                if n.x < 0 {
                    n.x += width;
                } else if n.x >= width {
                    n.x -= width;
                }
                if n.y < 0 {
                    n.y += height;
                } else if n.y >= height {
                    n.y -= height;
                }
                (n, *v)
            })
            .collect();
    }

    robots
        .iter()
        .filter(|(p, _)| p.x != width / 2 && p.y != height / 2)
        .into_group_map_by(|(p, _)| (p.x < width / 2, p.y < height / 2))
        .values()
        .map(|v| v.len())
        .product::<usize>()
        .print();
}

pub fn part2(input: String) {
    let (width, height) = (101, 103);
    let mut robots = input
        .lines()
        .map(|line| {
            let (px, py, vx, vy) = parse_nums::<i64>(line)
                .collect_tuple()
                .expect("Error parsing line");
            (Point { x: px, y: py }, Point { x: vx, y: vy })
        })
        .collect_vec();
    for i in 0..10403 {
        robots = robots
            .iter()
            .map(|(p, v)| {
                let mut n = *p + *v;
                if n.x < 0 {
                    n.x += width;
                } else if n.x >= width {
                    n.x -= width;
                }
                if n.y < 0 {
                    n.y += height;
                } else if n.y >= height {
                    n.y -= height;
                }
                (n, *v)
            })
            .collect();

        // Look for a set of consecutive robots in a row
        if robots
            .iter()
            .into_group_map_by(|(p, _)| p.y)
            .values()
            .any(|row| {
                row.iter().map(|(p, _)| p.x).sorted().tuple_windows().any(
                    |(a, b, c, d, e, f, g, h)| {
                        b == a + 1
                            && c == b + 1
                            && d == c + 1
                            && e == d + 1
                            && f == e + 1
                            && g == f + 1
                            && h == g + 1
                    },
                )
            })
        {
            println!("After {} seconds", i + 1);
            dump(&robots, width, height);
        }
    }
}
