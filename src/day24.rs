use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::selfprint::SelfPrint;

fn simulate<'a>(
    inputs: &mut HashMap<&'a str, u8>,
    operations: &HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>>,
) {
    let mut pending = operations.clone();

    while !pending.is_empty() {
        let input_keys = inputs.keys().collect::<HashSet<_>>();
        let available_inputs = pending
            .keys()
            .cloned()
            .filter(|(l, r)| input_keys.contains(l) && input_keys.contains(r))
            .collect_vec();

        if available_inputs.is_empty() {
            return;
        }

        for (left, right) in available_inputs {
            for (operation, output) in pending.get(&(left, right)).expect("Pending not found") {
                let left_operand = inputs.get(left).expect("Left operand not found");
                let right_operand = inputs.get(right).expect("Right operand not found");
                let value = match operation {
                    &"AND" => *left_operand & *right_operand,
                    &"OR" => *left_operand | *right_operand,
                    &"XOR" => *left_operand ^ *right_operand,
                    _ => panic!("Operation not found"),
                };
                inputs.insert(*output, value);
            }
            pending.remove(&(left, right));
        }
    }
}

fn read_binary(starting_char: char, inputs: &HashMap<&str, u8>) -> u64 {
    inputs
        .iter()
        .filter(|(k, _)| k.starts_with(starting_char))
        .sorted_by_key(|(k, _)| *k)
        .rev()
        .fold(0u64, |o, (_, v)| o << 1 | (*v as u64))
}

pub fn part1(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("No separator found");

    let mut inputs = HashMap::new();
    for line in top.lines() {
        let (left, right) = line.split_once(": ").expect("Color delimiter not found");
        inputs.insert(left, right.parse::<u8>().expect("Could not parse u8"));
    }

    let mut operations = HashMap::<(&str, &str), Vec<(&str, &str)>>::new();
    let regex = Regex::new(r"\w+").expect("Invalid regex");
    for line in bottom.lines() {
        let (left_operand, operation, right_operand, output) = regex
            .find_iter(line)
            .map(|m| m.as_str())
            .next_tuple()
            .expect("Could not parse evaluation");
        operations
            .entry((left_operand, right_operand))
            .or_default()
            .push((operation, output));
    }
    simulate(&mut inputs, &operations);

    read_binary('z', &inputs).print();
}

#[allow(dead_code)]
fn z_output_diffs(mut target: u64, mut current: u64) -> Vec<u8> {
    let mut result = Vec::new();
    let mut index = 0;
    while target != current {
        if current & 1 != target & 1 {
            result.push(index);
        }
        target = target >> 1;
        current = current >> 1;
        index += 1;
    }
    result
}

#[allow(dead_code)]
fn all_dependencies<'a>(
    output: &'a str,
    dependencies: &HashMap<&'a str, (&'a str, &'a str)>,
    starting_inputs: &HashMap<&'a str, u8>,
) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    if starting_inputs.contains_key(output) {
        return result;
    }

    result.insert(output);

    if let Some((left, right)) = dependencies.get(output) {
        for dependency in all_dependencies(*left, dependencies, starting_inputs) {
            result.insert(dependency);
        }
        for dependency in all_dependencies(*right, dependencies, starting_inputs) {
            result.insert(dependency);
        }
    }

    result
}

#[allow(dead_code)]
fn swap_outputs<'a>(
    operations: &HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>>,
    a: &&'a str,
    b: &&'a str,
) -> HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>> {
    operations
        .iter()
        .map(|((left, right), list)| {
            (
                (*left, *right),
                list.iter()
                    .map(|(operation, output)| {
                        if output == a {
                            // println!("Swapping {} for {}", a, b);
                            (*operation, *b)
                        } else if output == b {
                            // println!("Swapping {} for {}", b, a);
                            (*operation, *a)
                        } else {
                            (*operation, *output)
                        }
                    })
                    .collect_vec(),
            )
        })
        .collect()
}

fn format_dependencies<'a>(
    wire: &'a str,
    dependencies: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) -> String {
    if wire.starts_with('x') || wire.starts_with('y') {
        return wire.to_string();
    }

    let (left_operand, operation, right_operand) =
        dependencies.get(wire).expect("Wire dependency not found");
    format!(
        "({} {} {})",
        format_dependencies(*left_operand, dependencies),
        operation,
        format_dependencies(*right_operand, dependencies)
    )
}

pub fn part2(input: String) {
    let (_, bottom) = input.split_once("\n\n").expect("No separator found");

    let mut dependencies = HashMap::new();

    let regex = Regex::new(r"\w+").expect("Invalid regex");
    for line in bottom.lines() {
        let (left_operand, operation, right_operand, output) = regex
            .find_iter(line)
            .map(|m| m.as_str())
            .next_tuple()
            .expect("Could not parse evaluation");

        dependencies.insert(output, (left_operand, operation, right_operand));
    }

    // for (output, (left_operand, operation, right_operand)) in dependencies
    //     .iter()
    //     .filter(|(_, (l, _, r))| {
    //         l.starts_with('x') || l.starts_with('y') || r.starts_with('x') || r.starts_with('y')
    //     })
    //     .sorted_by_key(|(_, (l, _, _))| *l)
    // {
    //     println!(
    //         "{} = {} {} {}",
    //         output, left_operand, operation, right_operand
    //     );
    // }

    // Full adder: z00 = (x00 XOR y00)
    //             c00 = (x00 AND y00)
    //             z01 = ((x01 XOR y01) XOR c00)
    //             c01 = ((x01 XOR y01) AND c00) OR (x01 AND y01)

    for (output, (left_operand, operation, right_operand)) in dependencies
        .iter()
        .filter(|(_, (l, o, r))| {
            o == &"XOR"
                && !l.starts_with('x')
                && !l.starts_with('y')
                && !r.starts_with('x')
                && !r.starts_with('y')
        })
        .sorted_by_key(|(o, _)| **o)
    {
        if !output.starts_with('z') {
            println!(
                "{} = {} {} {}",
                output,
                format_dependencies(*left_operand, &dependencies),
                operation,
                format_dependencies(*right_operand, &dependencies),
            );
        } else {
            let left = dependencies
                .get(left_operand)
                .expect("Left operand not found");
            let right = dependencies
                .get(right_operand)
                .expect("Left operand not found");
            println!(
                "{} = ({} {} {}) {} ({} {} {})",
                output, left.0, left.1, left.2, operation, right.0, right.1, right.2
            );
        }
        // This tells us these 3 wire swaps based on the full adder,
        // since they are not hooked up to z outputs properly
        // wss, mvb, bmn
        // z17, z08, z23

        // And z14 looks weird
        // z14 = scs XOR rds
        // z14 = (hmt OR pdq) XOR (x14 AND y14)
        // z14 should have a x14 XOR y14 in it
        // Looking at the input, x14 XOR y14 gets output to jss
        // jss = x14 XOR y14
        // rds = x14 AND y14
        // So this is our last swap...
        // Sorted, it gives us:
        // bmn,jss,mvb,rds,wss,z08,z18,z23
    }
}

/*
    Old stuff that didn't work
    let x = read_binary('x', &starting_inputs);
    let y = read_binary('y', &starting_inputs);

    let target = x + y;

    let mut inputs = starting_inputs.clone();
    simulate(&mut inputs, &operations);

    let z = read_binary('z', &inputs);

    let diffs = z_output_diffs(target, z)
        .into_iter()
        .collect::<HashSet<_>>();
    let diff_names = diffs
        .iter()
        .map(|i| format!("z{:0>2}", i))
        .collect_vec()
        .print();
    let all_dependencies = diff_names
        .iter()
        .flat_map(|o| all_dependencies(o, &dependencies, &starting_inputs))
        .collect::<HashSet<_>>();

    let mut potentials = HashMap::<Vec<u8>, Vec<(&str, &str)>>::new();
    for (a, b) in all_dependencies.iter().tuple_combinations() {
        let test_operations = swap_outputs(&operations, a, b);
        let mut inputs = starting_inputs.clone();

        simulate(&mut inputs, &test_operations);

        let z = read_binary('z', &inputs);
        let test_diffs = z_output_diffs(target, z)
            .into_iter()
            .collect::<HashSet<_>>();

        let total_diffs = diffs
            .union(&test_diffs)
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&test_diffs)
            .cloned()
            .collect::<HashSet<_>>();
        if total_diffs.len() > 0 && total_diffs.len() < 5 {
            // potentials.insert((*a, *b), total_diffs);
            potentials
                .entry(total_diffs.iter().sorted().cloned().collect_vec())
                .or_default()
                .push((*a, *b));
        }
    }

    potentials
        .values()
        .flat_map(|x| x.iter().flat_map(|(a, b)| [*a, *b]))
        .sorted()
        .dedup()
        .collect_vec()
        .print();

    // potentials.print();

    // Tried: jss,pjf,qmd,rds,tqk,vmq,z18,z23

    // let mut test_operations = swap_outputs(&operations, &"qmd", &"tqk");
    // test_operations = swap_outputs(&test_operations, &"pjf", &"z23");
    // test_operations = swap_outputs(&test_operations, &"vmq", &"z18");
    // test_operations = swap_outputs(&test_operations, &"jss", &"rds");

    // let mut inputs = starting_inputs.clone();
    // simulate(&mut inputs, &test_operations);

    // inputs
    //     .iter()
    //     .filter(|(k, _)| k.starts_with('z'))
    //     .sorted_by_key(|(k, _)| **k)
    //     .collect_vec()
    //     .print();
    // let z = read_binary('z', &inputs);
    // println!("{} {} {}", z, target, z == target);

    let target_diffs = diffs.iter().sorted().cloned().collect_vec();
    'outer: for combination in potentials.keys().combinations(4).filter(|k| {
        k.iter()
            .flat_map(|x| x.iter().cloned())
            .sorted()
            .collect_vec()
            == target_diffs
    }) {
        for a in potentials.get(combination[0]).expect("A not found") {
            for b in potentials.get(combination[1]).expect("B not found") {
                for c in potentials.get(combination[2]).expect("C not found") {
                    for d in potentials.get(combination[3]).expect("D not found") {
                        let mut test_operations = swap_outputs(&operations, &a.0, &a.1);
                        test_operations = swap_outputs(&test_operations, &b.0, &b.1);
                        test_operations = swap_outputs(&test_operations, &c.0, &c.1);
                        test_operations = swap_outputs(&test_operations, &d.0, &d.1);

                        let mut inputs = starting_inputs.clone();
                        simulate(&mut inputs, &test_operations);

                        let z = read_binary('z', &inputs);
                        if target == z {
                            [a.0, a.1, b.0, b.1, c.0, c.1, d.0, d.1]
                                .iter()
                                .sorted()
                                .join(",")
                                .print();
                            // break 'outer;
                        }
                        // println!("Trying {:?} {:?} {:?} {:?}: {}", a, b, c, d, z);
                    }
                }
            }
        }
    }

    // potentials
    //     .iter()
    //     .sorted_by_key(|(_, set)| set.len())
    //     .collect_vec()
    //     .print();

    // println!("Done!");
    // for (a, b, c, d) in potentials.iter().tuple_combinations() {
    //     let mut test_operations = swap_outputs(&operations, &a.0, &a.1);
    //     test_operations = swap_outputs(&test_operations, &b.0, &b.1);
    //     test_operations = swap_outputs(&test_operations, &c.0, &c.1);
    //     test_operations = swap_outputs(&test_operations, &d.0, &d.1);

    //     let mut inputs = starting_inputs.clone();
    //     simulate(&mut inputs, &test_operations);

    //     let z = read_binary('z', &inputs);
    //     let test_diffs = z_output_diffs(target, z);
    //     if test_diffs.is_empty() {
    //         [a.0, a.1, b.0, b.1, c.0, c.1, d.0, d.1]
    //             .iter()
    //             .sorted()
    //             .join(",")
    //             .print();
    //         break;
    //     } else if test_diffs.len() < 10 {
    //         println!("{}: {}", z, test_diffs.len());
    //     }
    // }
*/
