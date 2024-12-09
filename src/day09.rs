use std::collections::VecDeque;

use crate::selfprint::SelfPrint;

fn checksum(files: &Vec<(u64, u64, u64)>) -> u64 {
    let mut total = 0;
    for (index, file_no, length) in files {
        for i in 0..*length {
            total += (index + i) * file_no;
        }
    }
    total
}

pub fn part1(input: String) {
    let mut files = VecDeque::new();
    let mut free = vec![];
    let mut file_no = 0u64;
    let mut is_file = true;
    let mut index = 0;
    for b in input.bytes() {
        let length = (b - b'0') as u64;
        if is_file {
            files.push_back((index, file_no, length));
            file_no += 1;
        } else {
            free.push((index, length));
        }
        is_file = !is_file;
        index += length;
    }

    'outer: for (mut index, mut length) in free {
        while length > 0 {
            let (ti, file_no, tl) = files.pop_back().expect("No target found");
            if ti < index {
                files.push_back((ti, file_no, tl));
                break 'outer;
            }
            if tl < length {
                files.push_front((index, file_no, tl));
                index += tl;
                length -= tl;
            } else {
                files.push_front((index, file_no, length));
                files.push_back((ti, file_no, tl - length));
                break;
            }
        }
    }

    checksum(&files.iter().map(|x| *x).collect()).print();
}

pub fn part2(input: String) {
    let mut files = VecDeque::new();
    let mut free = vec![];
    let mut file_no = 0u64;
    let mut is_file = true;
    let mut index = 0;
    for b in input.bytes() {
        let length = (b - b'0') as u64;
        if is_file {
            files.push_back((index, file_no, length));
            file_no += 1;
        } else {
            free.push((index, length));
        }
        is_file = !is_file;
        index += length;
    }

    for _ in 0..files.len() {
        let (index, file_no, length) = files.pop_back().expect("No file found");
        if let Some((i, (free_index, free_length))) = free
            .iter()
            .enumerate()
            .find(|(_, (p, l))| l >= &length && p < &index)
        {
            files.push_front((*free_index, file_no, length));

            let mut new_free = free.clone();
            new_free.remove(i);
            if free_length > &length {
                new_free.insert(i, (free_index + length, free_length - length));
            }
            free = new_free;
        } else {
            files.push_front((index, file_no, length));
        }
    }

    checksum(&files.iter().map(|x| *x).collect()).print();
}
