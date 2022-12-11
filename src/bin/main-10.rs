use std::fs;

fn main() {
    let file_name = "inputs/10.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let register_contents = compute_register_contents(&content);
    let mut signal_strength = 0;
    for idx in [20, 60, 100, 140, 180, 220] {
        signal_strength += idx as i64 * register_contents[idx - 1];
    }
    println!("The total signal strength is {}.", signal_strength);

    let screen = compute_screen_content(&register_contents);
    for l in screen.chunks(40) {
        println!(
            "{}",
            l.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

fn compute_register_contents(content: &str) -> Vec<i64> {
    let mut register_contents = Vec::new();
    let mut x = 1;
    for l in content.lines() {
        let (instruction, arg) = parse_line(l);
        match instruction.as_str() {
            "noop" => register_contents.push(x),
            "addx" => {
                register_contents.push(x);
                register_contents.push(x);
                x += arg.unwrap();
            }
            _ => panic!("Unknown instruction"),
        }
    }
    register_contents
}

fn parse_line(l: &str) -> (String, Option<i64>) {
    let mut l_split = l.split_whitespace();
    let instruction = l_split.next().unwrap().to_string();
    if let Some(arg) = l_split.next() {
        (instruction, Some(arg.parse().unwrap()))
    } else {
        (instruction, None)
    }
}

fn compute_screen_content(register_contents: &Vec<i64>) -> Vec<char> {
    let mut screen = Vec::new();
    for (cycle, position) in register_contents.iter().enumerate() {
        let screen_position = (cycle % 40) as i64;
        if (screen_position - position).abs() <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
    }
    screen
}
