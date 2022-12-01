use std::fs;

fn main() {
    let file_path = "inputs/1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let most_calories = find_most_calories(&contents);
    println!(
        "The elf with the most food carries {} calories.",
        most_calories
    );
    let mut calories_per_elf = count_calories_per_elf(&contents);
    calories_per_elf.sort();
    let sum_top_three_calories: u64 = calories_per_elf.iter().rev().take(3).sum();
    println!(
        "The top three elf carry {} calories in total.",
        sum_top_three_calories
    );
}

fn find_most_calories(list: &str) -> u64 {
    let mut most_calories = 0;
    let mut cur_calories = 0;
    for l in list.lines() {
        if l.len() == 0 {
            if cur_calories > most_calories {
                most_calories = cur_calories;
            }
            cur_calories = 0;
        } else {
            cur_calories += l
                .parse::<u64>()
                .expect("Should have been able to convert line to u64");
        }
    }
    most_calories
}

fn count_calories_per_elf(list: &str) -> Vec<u64> {
    let mut calories_per_elf = Vec::new();
    let mut cur_calories = 0;
    for l in list.lines() {
        if l.len() == 0 {
            calories_per_elf.push(cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += l
                .parse::<u64>()
                .expect("Should have been able to convert line to u64");
        }
    }
    calories_per_elf
}
