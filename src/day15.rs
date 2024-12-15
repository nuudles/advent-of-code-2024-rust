use std::collections::BTreeSet;

use crate::{point::Point, selfprint::SelfPrint};

fn try_move(
    target: &Point<i64>,
    direction: Point<i64>,
    boxes: &mut BTreeSet<Point<i64>>,
    walls: &BTreeSet<Point<i64>>,
) -> bool {
    let next = *target + direction;
    if walls.contains(&next) {
        return false;
    } else if boxes.contains(&next) {
        if try_move(&next, direction, boxes, walls) {
            if boxes.remove(&target) {
                boxes.insert(next);
            }
            return true;
        } else {
            return false;
        }
    }
    if boxes.remove(&target) {
        boxes.insert(next);
    }
    true
}

pub fn part1(input: String) {
    let (map, moves) = input.split_once("\n\n").expect("Separator not found");
    let mut walls = BTreeSet::new();
    let mut boxes = BTreeSet::new();
    let mut robot = Point { x: 0, y: 0 };
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            if c == '#' {
                walls.insert(p);
            } else if c == 'O' {
                boxes.insert(p);
            } else if c == '@' {
                robot = p;
            }
        }
    }

    for c in moves.chars() {
        let direction = match c {
            'v' => Point { x: 0, y: 1i64 },
            '^' => Point { x: 0, y: -1 },
            '<' => Point { x: -1, y: 0 },
            '>' => Point { x: 1, y: 0 },
            _ => continue,
        };
        if try_move(&robot, direction, &mut boxes, &walls) {
            robot = robot + direction;
        }
    }
    boxes.iter().map(|b| 100 * b.y + b.x).sum::<i64>().print();
}

#[allow(dead_code)]
fn dump(
    width: usize,
    height: usize,
    walls: &BTreeSet<Point<i64>>,
    left_boxes: &BTreeSet<Point<i64>>,
    right_boxes: &BTreeSet<Point<i64>>,
    robot: &Point<i64>,
) {
    for y in 0..height {
        for x in 0..width {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            if walls.contains(&p) {
                print!("#");
            } else if left_boxes.contains(&p) {
                print!("[");
            } else if right_boxes.contains(&p) {
                print!("]");
            } else if robot == &p {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn try_move_left(
    target: &Point<i64>,
    left_boxes: &mut BTreeSet<Point<i64>>,
    right_boxes: &mut BTreeSet<Point<i64>>,
    walls: &BTreeSet<Point<i64>>,
) -> bool {
    let next = target.left();
    if walls.contains(&next) {
        return false;
    } else if right_boxes.contains(&next) {
        if try_move_left(&next.left(), left_boxes, right_boxes, walls) {
            if left_boxes.remove(&target) && right_boxes.remove(&target.right()) {
                left_boxes.insert(next);
                right_boxes.insert(next.right());
            }
            return true;
        } else {
            return false;
        }
    }
    if left_boxes.remove(&target) && right_boxes.remove(&target.right()) {
        left_boxes.insert(next);
        right_boxes.insert(next.right());
    }
    true
}

fn try_move_right(
    target: &Point<i64>,
    left_boxes: &mut BTreeSet<Point<i64>>,
    right_boxes: &mut BTreeSet<Point<i64>>,
    walls: &BTreeSet<Point<i64>>,
) -> bool {
    let next = target.right();
    if walls.contains(&next) {
        return false;
    } else if left_boxes.contains(&next) {
        if try_move_right(&next.right(), left_boxes, right_boxes, walls) {
            if right_boxes.remove(&target) && left_boxes.remove(&target.left()) {
                right_boxes.insert(next);
                left_boxes.insert(next.left());
            }
            return true;
        } else {
            return false;
        }
    }
    if right_boxes.remove(&target) && left_boxes.remove(&target.left()) {
        right_boxes.insert(next);
        left_boxes.insert(next.left());
    }
    true
}

fn can_move_vertically(
    target: &Point<i64>,
    direction: Point<i64>,
    left_boxes: &mut BTreeSet<Point<i64>>,
    right_boxes: &mut BTreeSet<Point<i64>>,
    walls: &BTreeSet<Point<i64>>,
    targets: &mut BTreeSet<Point<i64>>,
) -> bool {
    let next = *target + direction;
    if walls.contains(&next) {
        return false;
    } else if left_boxes.contains(&next) {
        targets.insert(next);
        return can_move_vertically(&next, direction, left_boxes, right_boxes, walls, targets)
            && can_move_vertically(
                &next.right(),
                direction,
                left_boxes,
                right_boxes,
                walls,
                targets,
            );
    } else if right_boxes.contains(&next) {
        targets.insert(next.left());
        return can_move_vertically(&next, direction, left_boxes, right_boxes, walls, targets)
            && can_move_vertically(
                &next.left(),
                direction,
                left_boxes,
                right_boxes,
                walls,
                targets,
            );
    }
    true
}

fn try_move_vertically(
    target: &Point<i64>,
    direction: Point<i64>,
    left_boxes: &mut BTreeSet<Point<i64>>,
    right_boxes: &mut BTreeSet<Point<i64>>,
    walls: &BTreeSet<Point<i64>>,
) -> bool {
    let mut targets = BTreeSet::new();
    if can_move_vertically(
        target,
        direction,
        left_boxes,
        right_boxes,
        walls,
        &mut targets,
    ) {
        let mut next = BTreeSet::new();
        for p in targets {
            if left_boxes.remove(&p) && right_boxes.remove(&p.right()) {
                next.insert(p + direction);
            }
        }
        for p in next {
            left_boxes.insert(p);
            right_boxes.insert(p.right());
        }
        true
    } else {
        false
    }
}

pub fn part2(input: String) {
    let (map, moves) = input.split_once("\n\n").expect("Separator not found");
    let mut walls = BTreeSet::new();
    let mut left_boxes = BTreeSet::new();
    let mut right_boxes = BTreeSet::new();
    let mut robot = Point { x: 0, y: 0 };
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let l = Point {
                x: (x * 2) as i64,
                y: y as i64,
            };
            let r = Point {
                x: (x * 2 + 1) as i64,
                y: y as i64,
            };
            if c == '#' {
                walls.insert(l);
                walls.insert(r);
            } else if c == 'O' {
                left_boxes.insert(l);
                right_boxes.insert(r);
            } else if c == '@' {
                robot = l;
            }
        }
    }

    // let height = map.lines().count();
    // let width = map.lines().next().unwrap().chars().count() * 2;

    // dump(width, height, &walls, &left_boxes, &right_boxes, &robot);

    for c in moves.chars() {
        match c {
            'v' => {
                if try_move_vertically(
                    &robot,
                    Point { x: 0, y: 1 },
                    &mut left_boxes,
                    &mut right_boxes,
                    &walls,
                ) {
                    robot = robot.down();
                }
            }
            '^' => {
                if try_move_vertically(
                    &robot,
                    Point { x: 0, y: -1 },
                    &mut left_boxes,
                    &mut right_boxes,
                    &walls,
                ) {
                    robot = robot.up();
                }
            }
            '<' => {
                if try_move_left(&robot, &mut left_boxes, &mut right_boxes, &walls) {
                    robot = robot.left();
                }
            }
            '>' => {
                if try_move_right(&robot, &mut left_boxes, &mut right_boxes, &walls) {
                    robot = robot.right();
                }
            }
            _ => continue,
        };
    }
    // dump(width, height, &walls, &left_boxes, &right_boxes, &robot);
    left_boxes
        .iter()
        .map(|b| 100 * b.y + b.x)
        .sum::<i64>()
        .print();
}
