use std::{collections::HashSet, fs};

fn main() {
    let file_name = "inputs/9.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let visited_positions = compute_visited_positions(&content, 10);
    let tail_positions = visited_positions
        .iter()
        .map(|v| v.last().unwrap().clone())
        .collect::<Vec<(i64, i64)>>();
    let unique_tail_positions = tail_positions
        .into_iter()
        .collect::<HashSet<(i64, i64)>>()
        .len();
    println!(
        "The tail has visited {} positions at least once.",
        unique_tail_positions
    );
}

fn compute_visited_positions(content: &str, rope_length: usize) -> Vec<Vec<(i64, i64)>> {
    let mut visited_positions = Vec::new();
    visited_positions.resize(1, Vec::new());
    for _ in 0..rope_length {
        visited_positions[0].push((0, 0));
    }
    for l in content.lines() {
        let (direction, count) = parse_instruction(l);
        for _ in 0..count {
            let positions = apply_instruction(direction, visited_positions.last().unwrap());
            visited_positions.push(positions.clone());
        }
    }
    visited_positions
}

fn parse_instruction(l: &str) -> (char, usize) {
    let mut l_split = l.split_whitespace();
    (
        l_split.next().unwrap().chars().next().unwrap(),
        l_split.next().unwrap().parse().unwrap(),
    )
}

fn apply_instruction(direction: char, positions: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut new_positions = Vec::new();
    let mut head_position = positions[0];
    match direction {
        'L' => head_position.1 -= 1,
        'R' => head_position.1 += 1,
        'U' => head_position.0 += 1,
        'D' => head_position.0 -= 1,
        _ => panic!("Unknown instruction"),
    }
    new_positions.push(head_position);
    for i in 1..positions.len() {
        let position = update_follower_position(&new_positions[i - 1], &positions[i]);
        new_positions.push(position);
    }
    new_positions
}

fn update_follower_position(head_position: &(i64, i64), tail_position: &(i64, i64)) -> (i64, i64) {
    if (head_position.0 - tail_position.0).abs() <= 1
        && (head_position.1 - tail_position.1).abs() <= 1
    {
        return tail_position.clone();
    }
    let mut new_tail_position = tail_position.clone();
    if head_position.0 < new_tail_position.0 {
        new_tail_position.0 -= 1;
    } else if head_position.0 > new_tail_position.0 {
        new_tail_position.0 += 1;
    }
    if head_position.1 < new_tail_position.1 {
        new_tail_position.1 -= 1;
    } else if head_position.1 > new_tail_position.1 {
        new_tail_position.1 += 1;
    }
    new_tail_position
}
