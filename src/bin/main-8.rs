use std::fs;

fn main() {
    let file_name = "inputs/8.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let map = parse_map(&content);
    print(&map);
    let visibility_map = compute_visibility_map(&map);
    print(&visibility_map);
    let n_visible = visibility_map
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .sum::<u32>();
    println!("{} trees are visible from the outside.", n_visible);
}

fn parse_map(content: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    for (i, l) in content.lines().enumerate() {
        map.push(Vec::new());
        for c in l.chars() {
            map[i].push(c.to_digit(10).unwrap() as u32);
        }
    }
    map
}

fn compute_visibility_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut visibility_map = Vec::new();
    for i in 0..map.len() {
        visibility_map.push(Vec::new());
        visibility_map[i].resize(map[i].len(), 0);
    }
    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            if is_visible(map, i, j) {
                visibility_map[i][j] = 1;
            }
        }
    }
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 || i == map.len() - 1 || j == 0 || j == map[i].len() - 1 {
                visibility_map[i][j] = 1;
            }
        }
    }
    visibility_map
}

fn is_visible(map: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let height = map[i][j];
    let mut visible_from_left = true;
    for k in 0..j {
        if map[i][k] >= height {
            visible_from_left = false;
            break;
        }
    }
    let mut visible_from_right = true;
    for k in j + 1..map[i].len() {
        if map[i][k] >= height {
            visible_from_right = false;
            break;
        }
    }
    let mut visible_from_bottom = true;
    for l in 0..i {
        if map[l][j] >= height {
            visible_from_bottom = false;
            break;
        }
    }
    let mut visible_from_top = true;
    for l in i + 1..map.len() {
        if map[l][j] >= height {
            visible_from_top = false;
            break;
        }
    }
    visible_from_left || visible_from_right || visible_from_bottom || visible_from_top
}

fn print(map: &Vec<Vec<u32>>) {
    for l in map {
        for e in l {
            print!("{}", e);
        }
        print!("\n");
    }
}
