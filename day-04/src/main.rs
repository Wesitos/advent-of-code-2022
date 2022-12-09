use std::io::Result;

use itertools::Itertools;

use helpers::read_lines;

const INPUT_PATH: &str = "./input.txt";

fn parse_range(range_str: &str) -> (i32, i32) {
    range_str
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .next_tuple()
        .unwrap()
}

fn line_parser(line_res: Result<String>) -> ((i32, i32), (i32, i32)) {
    if let Ok(line) = line_res {
        line.split(",")
            .map(parse_range)
            .take(2)
            .next_tuple()
            .unwrap()
    } else {
        panic!("IO error");
    }
}

fn simplify_ranges<'a>(ranges: &'a [(i32, i32)]) -> Vec<(i32, i32)> {
    let mut ranges_ = ranges.to_vec();

    ranges_.sort();

    let mut res: Vec<_> = ranges_.drain(0..1).collect();

    let (mut x1, mut x2) = res[0];

    for (y1, y2) in ranges_ {
        if y2 <= x2 {
            continue;
        }
        if x1 == y1 && x2 < y2 {
            res.pop();
            res.push((y1, y2));
            (x1, x2) = (y1, y2);
            continue;
        }
        res.push((y1, y2));
        (x1, x2) = (y1, y2);
    }

    res
}

fn is_overlapping<'a>(ranges: &'a [(i32, i32)]) -> bool {
    let mut ranges_ = ranges.to_vec();

    ranges_.sort();

    let mut x2: i32;

    (_, x2) = ranges_.remove(0);

    for (y1, y2) in ranges_ {
        if y1 <= x2 {
            return true;
        }
        x2 = y2;
    }
    false
}

fn main() {
    if let Ok(lines) = read_lines(INPUT_PATH) {
        let redundant_iter = lines
            .map(line_parser)
            .filter(|(left, right)| simplify_ranges(&[*left, *right]).len() == 1);

        println!("Redundant ranges: {}", redundant_iter.count());
    }

    if let Ok(lines) = read_lines(INPUT_PATH) {
        assert!(!is_overlapping(&[(1,2), (3,4)]));

        let overlapping_iter = lines
            .map(line_parser)
            .filter(|(left, right)| is_overlapping(&[*left, *right]));

        println!("Redundant ranges: {}", overlapping_iter.count());
    }
}
