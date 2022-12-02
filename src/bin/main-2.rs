use std::{collections::HashMap, fs};

fn main() {
    let score_shape: HashMap<&str, u64> = [("X", 1), ("Y", 2), ("Z", 3)].into_iter().collect();
    let score_outcome: HashMap<&str, u64> = [("loose", 0), ("draw", 3), ("win", 6)]
        .into_iter()
        .collect();

    let file_path = "inputs/2.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let score = determine_score(&contents, &score_shape, &score_outcome);
    println!("Your final score is: {}", score);
}

fn determine_score(
    list: &str,
    score_shape: &HashMap<&str, u64>,
    score_outcome: &HashMap<&str, u64>,
) -> u64 {
    let mut score = 0;
    for l in list.lines() {
        let l_split = l.split_whitespace().collect::<Vec<&str>>();
        let opponent_choice = l_split[0];
        // let my_choice = l_split[1];  // for part1
        let desired_outcome = l_split[1]; // for part2
        let my_choice = determine_my_choice(&opponent_choice, &desired_outcome);
        score += score_shape[&my_choice as &str];
        let outcome = determine_outcome(opponent_choice, &my_choice);
        score += score_outcome[&outcome as &str];
    }
    score
}

fn determine_outcome(choice0: &str, choice1: &str) -> String {
    if (choice0 == "A" && choice1 == "X")
        || (choice0 == "B" && choice1 == "Y")
        || (choice0 == "C" && choice1 == "Z")
    {
        return "draw".to_string();
    } else if (choice0 == "A" && choice1 == "Y")
        || (choice0 == "B" && choice1 == "Z")
        || (choice0 == "C" && choice1 == "X")
    {
        return "win".to_string();
    }
    "loose".to_string()
}

fn determine_my_choice(opponent_choice: &str, desired_outcome: &str) -> String {
    match opponent_choice {
        "A" => match desired_outcome {
            "X" => "Z".to_string(),
            "Y" => "X".to_string(),
            "Z" => "Y".to_string(),
            _ => todo!(),
        },
        "B" => match desired_outcome {
            "X" => "X".to_string(),
            "Y" => "Y".to_string(),
            "Z" => "Z".to_string(),
            _ => todo!(),
        },
        "C" => match desired_outcome {
            "X" => "Y".to_string(),
            "Y" => "Z".to_string(),
            "Z" => "X".to_string(),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
