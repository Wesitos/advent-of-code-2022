mod helpers;

use std::collections::HashSet;
use std::io;
use std::iter::Iterator;

use itertools::Itertools;

use crate::helpers::read_lines;

const INPUT_PATH: &str = "./input.txt";

fn line_unwrapper(line_res: io::Result<String>) -> String {
    if let Ok(line) = line_res {
        line
    } else {
        panic!("IO error");
    }
}

fn line_container_parser(line_res: io::Result<String>) -> (Vec<char>, Vec<char>) {
    let line = line_unwrapper(line_res);
    let size = line.len() / 2;
    (
        line[..size].chars().collect(),
        line[size..].chars().collect(),
    )
}

fn find_common<'a>(vec_a: &'a [char], vec_b: &'a [char]) -> Vec<char> {
    let set_a: HashSet<char> = HashSet::from_iter(vec_a.iter().cloned());
    let set_b: HashSet<char> = HashSet::from_iter(vec_b.iter().cloned());

    let common_iter = set_a.intersection(&set_b);

    common_iter.copied().collect()
}

fn get_char_priority(c: char) -> i32 {
    (if c >= 'a' {
        c as u8 - 'a' as u8 + 1
    } else {
        c as u8 - 'A' as u8 + 27
    })
    .into()
}

fn find_group_badge(group: Vec<String>) -> char {
    *group
        .iter()
        .map(|rucksack| rucksack.chars().collect::<Vec<_>>())
        .reduce(|left, right| find_common(&left, &right))
        .unwrap()
        .last()
        .unwrap()
}

fn main() {
    if let Ok(lines) = read_lines(INPUT_PATH) {
        let total_priorities: i32 = lines
            .map(line_container_parser)
            .map(|(x, y)| find_common(&x, &y))
            .flatten()
            .map(get_char_priority)
            .sum();

        println!("Total priority sum: {}", total_priorities);
    }

    if let Ok(lines) = read_lines(INPUT_PATH) {
        let total_group_badge_sum: i32 = lines
            .map(line_unwrapper)
            .chunks(3)
            .into_iter()
            .map(|chunk| Vec::from_iter(chunk))
            .map(find_group_badge)
            .map(get_char_priority)
            .sum();

        println!("Total badge priority sum: {}", total_group_badge_sum);
    }
}
