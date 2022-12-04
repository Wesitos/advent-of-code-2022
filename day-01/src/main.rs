use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const INPUT_PATH:&str = "./input.txt";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn iterate_calories(max_counts: &mut [i32], current_count: i32, line: String) -> i32 {
    if line.is_empty() {
        max_counts[max_counts.len() - 1] = current_count;
        max_counts.sort();
        max_counts.reverse();
        max_counts[max_counts.len() - 1] = 0;

        return 0;
    }

    let count: i32 = line.parse().unwrap();
    return current_count + count;
}

fn main() {
    let mut max_counts = [0; 4];
    let mut current_count: i32 = 0;

    if let Ok(lines) = read_lines(INPUT_PATH) {
        for line in lines {
            if let Ok(calorie_str) = line {
                current_count = iterate_calories(&mut max_counts, current_count, calorie_str);
            }
        }
    }

    let total_count: i32 = max_counts.into_iter().take(max_counts.len() - 1).sum();

    println!("Max: {}  Sum of 3 max: {}", max_counts[0],  total_count);
}
