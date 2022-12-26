use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/14.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let all_paths = read_scan(content.clone());
    let mut map = create_map_from_paths(all_paths);
    print_map(&map, 500, 50, 33);
    let bottom = map.iter().map(|(p, _c)| p.0).max().unwrap();
    simulate(&mut map, bottom);
    println!("==========================");
    print_map(&map, 500, 50, 33);
    let sand_units = map
        .iter()
        .map(|(_p, c)| if let Point::Sand = c { 1 } else { 0 })
        .sum::<usize>();
    println!("{} units of sand come to rest.", sand_units);
}

fn print_map(map: &HashMap<(usize, usize), Point>, offset: usize, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            let position = (i, j + offset - width / 2);
            if map.contains_key(&position) {
                match map[&position] {
                    Point::Rock => print!("#"),
                    Point::Sand => print!("o"),
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn read_scan(content: String) -> Vec<Vec<(usize, usize)>> {
    let mut all_paths = Vec::new();
    for l in content.lines() {
        let mut path = Vec::new();
        for p in l.split(" -> ") {
            let xy = p
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            path.push((xy[1], xy[0]));
        }
        all_paths.push(path);
    }
    all_paths
}

fn create_map_from_paths(all_paths: Vec<Vec<(usize, usize)>>) -> HashMap<(usize, usize), Point> {
    let mut map = HashMap::new();
    for p in all_paths {
        fill_path(&mut map, p, Point::Rock);
    }
    map
}

#[derive(Clone, Copy)]
enum Point {
    Rock,
    Sand,
}

fn fill_path(map: &mut HashMap<(usize, usize), Point>, path: Vec<(usize, usize)>, content: Point) {
    for i in 1..path.len() {
        let diff = (
            path[i - 1].0 as i64 - path[i].0 as i64,
            path[i - 1].1 as i64 - path[i].1 as i64,
        );
        if diff.0 == 0 {
            // draw horizontal path
            let start = std::cmp::min(path[i - 1].1, path[i].1);
            let end = std::cmp::max(path[i - 1].1, path[i].1);
            for j in start..end + 1 {
                let position = (path[i].0, j);
                map.insert(position, content);
            }
        } else if diff.1 == 0 {
            // draw vertical path
            let start = std::cmp::min(path[i - 1].0, path[i].0);
            let end = std::cmp::max(path[i - 1].0, path[i].0);
            for j in start..end + 1 {
                let position = (j, path[i].1);
                map.insert(position, content);
            }
        } else {
            panic!("Not a horizontal or vertical path");
        }
    }
}

fn simulate(map: &mut HashMap<(usize, usize), Point>, bottom: usize) {
    loop {
        let p = (0, 500);
        let final_position = sink(map, p, bottom);
        if let Some(final_position) = final_position {
            map.insert(final_position, Point::Sand);
        } else {
            break;
        }
    }
}

fn sink(
    map: &HashMap<(usize, usize), Point>,
    p: (usize, usize),
    bottom: usize,
) -> Option<(usize, usize)> {
    if p.0 == bottom + 1 {
        // part2: hit the bottom
        return Some(p);
    }

    let y = p.0 + 1;
    // if y > bottom { // part1: fell out the bottom
    //     return None;
    // }

    let x = p.1;
    if !map.contains_key(&(y, x)) {
        return sink(map, (y, x), bottom);
    }

    let x = p.1 - 1;
    if !map.contains_key(&(y, x)) {
        return sink(map, (y, x), bottom);
    }

    let x = p.1 + 1;
    if !map.contains_key(&(y, x)) {
        return sink(map, (y, x), bottom);
    }

    if p == (0, 500) {
        // part2: clogged the source
        return None;
    }

    Some(p)
}
