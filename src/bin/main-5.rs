use std::fs;

fn main() {
    let file_path = "inputs/5.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (stack, instructions) = separate_stack_and_instructions(&contents);
    let mut stack = parse_stack(&stack);
    // apply_instructions(&mut stack, &instructions, false);
    // let topmost_crates = stack.iter().map(|s| s.last().expect("Should be able to check crate").to_string()).collect::<Vec<String>>();
    // println!("The topmost crates are {}.", topmost_crates.join(""));
    apply_instructions(&mut stack, &instructions, true);
    let topmost_crates = stack
        .iter()
        .map(|s| s.last().expect("Should be able to check crate").to_string())
        .collect::<Vec<String>>();
    println!(
        "The topmost crates (CrateMover9001) are {}.",
        topmost_crates.join("")
    );
}

fn separate_stack_and_instructions(content: &str) -> (Vec<String>, Vec<String>) {
    let mut stack = Vec::new();
    let mut instructions = Vec::new();
    let mut parsing_stack = true;
    for l in content.lines() {
        if l.len() == 0 {
            parsing_stack = false;
            continue;
        }
        if parsing_stack {
            stack.push(l.to_string());
        } else {
            instructions.push(l.to_string());
        }
    }
    (stack, instructions)
}

fn parse_stack(stack: &Vec<String>) -> Vec<Vec<char>> {
    let mut new_stack: Vec<Vec<char>> = Vec::new();
    // substract one to only take the lines containing information about crates
    for l in stack.iter().take(stack.len() - 1) {
        let l_chars = l.chars().collect::<Vec<char>>();
        // add one to make sure the last column is included
        for column in 0..l.len() / 4 + 1 {
            if column >= new_stack.len() {
                new_stack.resize(column + 1, Vec::new());
            }
            let cr = l_chars[column * 4 + 1];
            if cr != ' ' {
                // we're dealing with small vectors, so don't worry about moving
                // all elements
                new_stack[column].insert(0, cr);
            }
        }
    }
    new_stack
}

fn apply_instructions(
    stack: &mut Vec<Vec<char>>,
    instructions: &Vec<String>,
    is_cratemover9001: bool,
) {
    for l in instructions {
        let (count, source, target) = parse_instruction(&l);
        if is_cratemover9001 {
            let mut tmp_storage = Vec::new();
            for _ in 0..count {
                let cr = stack[source]
                    .pop()
                    .expect("Should be able to remove a crate");
                tmp_storage.push(cr);
            }
            for _ in 0..count {
                let cr = tmp_storage.pop().expect("Should be able to remove a crate");
                stack[target].push(cr);
            }
        } else {
            for _ in 0..count {
                let cr = stack[source]
                    .pop()
                    .expect("Should be able to remove a crate");
                stack[target].push(cr);
            }
        }
    }
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let instructions_split = instruction.split_whitespace().collect::<Vec<&str>>();
    let count = instructions_split[1]
        .parse()
        .expect("Should be able to convert to usize");
    let source: usize = instructions_split[3]
        .parse()
        .expect("Should be able to convert to usize");
    let target: usize = instructions_split[5]
        .parse()
        .expect("Should be able to convert to usize");
    (count, source - 1, target - 1)
}
