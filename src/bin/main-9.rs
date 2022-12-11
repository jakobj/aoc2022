use std::{collections::HashSet, fs};

fn main() {
    let file_name = "inputs/9.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let tail_positions = compute_tail_positions(&content);
    let unique_tail_positions = tail_positions
        .into_iter()
        .collect::<HashSet<(i64, i64)>>()
        .len();
    println!(
        "The tail has visited {} positions at least once.",
        unique_tail_positions
    );
}

fn compute_tail_positions(content: &str) -> Vec<(i64, i64)> {
    let mut head_position = (0, 0);
    let mut tail_positions = vec![(0, 0)];
    for l in content.lines() {
        let instruction = parse_instruction(l);
        apply_instruction(instruction, &mut head_position, &mut tail_positions);
    }
    tail_positions
}

fn parse_instruction(l: &str) -> (char, usize) {
    let mut l_split = l.split_whitespace();
    (
        l_split.next().unwrap().chars().next().unwrap(),
        l_split.next().unwrap().parse().unwrap(),
    )
}

fn apply_instruction(
    instruction: (char, usize),
    head_position: &mut (i64, i64),
    tail_positions: &mut Vec<(i64, i64)>,
) {
    for _ in 0..instruction.1 {
        match instruction.0 {
            'L' => head_position.1 -= 1,
            'R' => head_position.1 += 1,
            'U' => head_position.0 += 1,
            'D' => head_position.0 -= 1,
            _ => panic!("Unknown instruction"),
        }
        let position = update_tail_position(head_position, tail_positions.last().unwrap());
        tail_positions.push(position);
    }
}

fn update_tail_position(head_position: &(i64, i64), tail_position: &(i64, i64)) -> (i64, i64) {
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
