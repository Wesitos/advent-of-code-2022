mod helpers;

use std::io;
use itertools::Itertools;
use crate::helpers::read_lines;

const INPUT_PATH:&str = "./input.txt";

fn line_parser(line_res: io::Result<String>) -> (char, char) {
    if let Ok(line) = line_res {
        let (opponent, player) = line.split_whitespace().next_tuple().unwrap();

        return (opponent.chars().next().unwrap(), player.chars().next().unwrap());
    } else {
        panic!("IO error");
    }
}

fn pick_score(pick: char) -> i32 {
    match pick {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn match_score(opponent_pick: char, player_pick: char) -> i32 {
    match (opponent_pick, player_pick) {
        ('A', 'B') | ('B', 'C') | ('C', 'A') => 6,
        ('A', 'A') | ('B', 'B') | ('C', 'C') => 3,
        ('A', 'C') | ('B', 'A') | ('C', 'B') => 0,
        _ => 0,
    }
}

fn match_score_mapper_1(opponent_pick: char, player_pick_r: char) -> i32 {
    let player_pick = match player_pick_r {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => 'D',
    };

    pick_score(player_pick) + match_score(opponent_pick, player_pick)
}

fn match_score_mapper_2(opponent_pick: char, match_result: char) -> i32 {
    let player_pick = match (opponent_pick, match_result) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'A',
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'B',
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'C',
        _ => 'D',
    };

    return pick_score(player_pick) + match_score(opponent_pick, player_pick);
}

fn main() {
    if let Ok(lines) = read_lines(INPUT_PATH) {
        let score = lines
            .filter(|x| !(x.as_ref().unwrap().is_empty()))
            .map(line_parser)
            .fold(0, |res, x| res + match_score_mapper_1(x.0, x.1));

        println!("Score: {}", score);
    }

    if let Ok(lines) = read_lines(INPUT_PATH) {
        let score = lines
            .filter(|x| !(x.as_ref().unwrap().is_empty()))
            .map(line_parser)
            .fold(0, |res, x| res + match_score_mapper_2(x.0, x.1));

        println!("Score: {}", score);
    }
}
