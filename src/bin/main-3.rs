use std::{collections::HashSet, fs, str::Chars};

fn main() {
    let file_path = "inputs/3.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let priorities = compute_priorities_of_duplicated_items(&contents);
    println!(
        "The sum of all (shared item) priorities is {}.",
        priorities.iter().sum::<u32>()
    );
    let priorities = compute_priorities_of_badges(&contents);
    println!(
        "The sum of all (badge) priorities is {}.",
        priorities.iter().sum::<u32>()
    );
}

fn compute_priorities_of_duplicated_items(list: &str) -> Vec<u32> {
    let mut priorities = Vec::new();
    for l in list.lines() {
        let shared_item = find_shared_item(l).expect("Should have found duplicate item");
        priorities.push(char_to_value(shared_item));
    }
    priorities
}

fn find_shared_item(line: &str) -> Option<char> {
    let n_2 = line.len() / 2;
    let first_half: HashSet<char> = line[..n_2].chars().collect();
    let second_half: HashSet<char> = line[n_2..].chars().collect();
    if let Some(shared_item) = first_half.intersection(&second_half).next() {
        return Some(*shared_item);
    }
    None
}

fn char_to_value(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 'a' as u32 + 1;
    } else {
        return c as u32 - 'A' as u32 + 1 + 26;
    }
}

fn compute_priorities_of_badges(list: &str) -> Vec<u32> {
    let mut priorities = Vec::new();
    let mut lines = list.lines();
    loop {
        let group0 = lines.next();
        if group0.is_none() {
            break;
        }
        let group0 = group0.unwrap();
        let group1 = lines.next().unwrap();
        let group2 = lines.next().unwrap();
        let badge = determine_badge(group0, group1, group2);
        priorities.push(char_to_value(badge));
    }
    priorities
}

fn determine_badge(group0: &str, group1: &str, group2: &str) -> char {
    let content0: HashSet<char> = group0.chars().collect();
    let content1: HashSet<char> = group1.chars().collect();
    let content2: HashSet<char> = group2.chars().collect();
    *(content0
        .intersection(&content1)
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&content2)
        .next()
        .unwrap())
}
