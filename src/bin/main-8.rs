use std::fs;

fn main() {
    let file_name = "inputs/8.txt";
    let content = fs::read_to_string(file_name).expect("Should be able to read file");
    let map = parse_map(&content);
    // print(&map);
    let visibility_map = compute_visibility_map(&map);
    // print(&visibility_map);
    let n_visible = visibility_map
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .sum::<u32>();
    println!("{} trees are visible from the outside.", n_visible);

    let scenic_map = compute_scenic_map(&map);
    // print(&scenic_map);
    let scenic_score = scenic_map
        .iter()
        .map(|v| v.iter().max().unwrap())
        .max()
        .unwrap();
    println!("The best tree has a scenic score of {}.", scenic_score);
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

fn compute_scenic_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut scenic_map = Vec::new();
    for i in 0..map.len() {
        scenic_map.push(Vec::new());
        scenic_map[i].resize(map[i].len(), 0);
        for j in 0..map[i].len() {
            scenic_map[i][j] = compute_scenic_score(map, i, j);
        }
    }
    scenic_map
}

fn compute_scenic_score(map: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let height = map[row][col];
    let mut scenic_score_top = 0;
    for i in (0..row).rev() {
        scenic_score_top += 1;
        if map[i][col] >= height {
            break;
        }
    }
    let mut scenic_score_bottom = 0;
    for i in row + 1..map.len() {
        scenic_score_bottom += 1;
        if map[i][col] >= height {
            break;
        }
    }
    let mut scenic_score_left = 0;
    for j in (0..col).rev() {
        scenic_score_left += 1;
        if map[row][j] >= height {
            break;
        }
    }
    let mut scenic_score_right = 0;
    for j in col + 1..map[row].len() {
        scenic_score_right += 1;
        if map[row][j] >= height {
            break;
        }
    }
    scenic_score_top * scenic_score_bottom * scenic_score_left * scenic_score_right
}
