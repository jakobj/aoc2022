use std::{collections::HashSet, fs};

fn main() {
    let file_name = "inputs/6.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let marker_position = determine_marker_position(&content, 4);
    println!(
        "The first packet marker appears after character {}.",
        marker_position
    );
    let marker_position = determine_marker_position(&content, 14);
    println!(
        "The first message marker appears after character {}.",
        marker_position
    );
}

fn determine_marker_position(content: &str, n_distinct_characters: usize) -> usize {
    let chars = content.chars().collect::<Vec<char>>();
    let mut anker = 0;
    let mut current = 0;
    let mut markers: HashSet<char> = HashSet::new();
    while markers.len() < n_distinct_characters && anker < content.len() {
        let c = chars[anker + current];
        if markers.contains(&c) {
            markers.clear();
            anker += 1;
            current = 0;
        } else {
            markers.insert(c);
            current += 1;
        }
    }
    anker + current
}
