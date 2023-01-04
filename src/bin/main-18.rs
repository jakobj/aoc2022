use std::{collections::HashSet, fs};

fn main() {
    let filename = "inputs/18.txt";
    let content = fs::read_to_string(filename).unwrap();
    let cubes = parse_cubes(content);
    let area = measure_total_area(&cubes);
    println!("The surface area of the lava droplet is {}.", area);
    let enclosed_air_cubes = determine_enclosed_air_cubes(&cubes);
    let enclosed_air_area = measure_total_area(&enclosed_air_cubes);
    println!(
        "The exterior surface area of the lava droplet is {}.",
        area - enclosed_air_area
    );
}

fn parse_cubes(content: String) -> Vec<(i32, i32, i32)> {
    let mut cubes = Vec::new();
    for l in content.lines() {
        let xyz = l
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        cubes.push((xyz[0], xyz[1], xyz[2]));
    }
    cubes
}

fn measure_total_area(cubes: &Vec<(i32, i32, i32)>) -> i32 {
    let cube_set: HashSet<(i32, i32, i32)> = cubes.clone().into_iter().collect();
    let mut cc = cube_set.clone();
    let mut area = 0;
    for e in cube_set.iter() {
        cc.remove(&e);
        for (delta_x, delta_y, delta_z) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let f = (e.0 + delta_x, e.1 + delta_y, e.2 + delta_z);
            if !cc.contains(&f) {
                area += 1;
            }
        }
        cc.insert(*e);
    }
    area
}

fn determine_enclosed_air_cubes(cubes: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let cube_set: HashSet<(i32, i32, i32)> = cubes.clone().into_iter().collect();
    let mut enclosed_air_cubes = Vec::new();
    let mut free_air_cube_set = HashSet::new();
    let x_min = cubes.iter().map(|&(x, _y, _z)| x).min().unwrap();
    let x_max = cubes.iter().map(|&(x, _y, _z)| x).max().unwrap();
    let y_min = cubes.iter().map(|&(_x, y, _z)| y).min().unwrap();
    let y_max = cubes.iter().map(|&(_x, y, _z)| y).max().unwrap();
    let z_min = cubes.iter().map(|&(_x, _y, z)| z).min().unwrap();
    let z_max = cubes.iter().map(|&(_x, _y, z)| z).max().unwrap();
    for x in x_min..x_max {
        for y in y_min..y_max {
            for z in z_min..z_max {
                let c = (x, y, z);
                if !cube_set.contains(&c) && is_enclosed(c, &cube_set, &free_air_cube_set) {
                    enclosed_air_cubes.push(c);
                } else {
                    free_air_cube_set.insert(c);
                }
            }
        }
    }
    enclosed_air_cubes
}

fn is_enclosed(
    p: (i32, i32, i32),
    cube_set: &HashSet<(i32, i32, i32)>,
    free_air_cube_set: &HashSet<(i32, i32, i32)>,
) -> bool {
    let x_min = cube_set.iter().map(|&(x, _y, _z)| x).min().unwrap();
    let x_max = cube_set.iter().map(|&(x, _y, _z)| x).max().unwrap();
    let y_min = cube_set.iter().map(|&(_x, y, _z)| y).min().unwrap();
    let y_max = cube_set.iter().map(|&(_x, y, _z)| y).max().unwrap();
    let z_min = cube_set.iter().map(|&(_x, _y, z)| z).min().unwrap();
    let z_max = cube_set.iter().map(|&(_x, _y, z)| z).max().unwrap();
    let goal = (x_min - 1, y_min - 1, z_min - 1);
    let mut queue = Vec::new();
    queue.push(p);
    let mut visited = HashSet::new();
    while let Some(current) = queue.pop() {
        if current == goal || free_air_cube_set.contains(&current) {
            return false;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        for (delta_x, delta_y, delta_z) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let c = (
                current.0 + delta_x,
                current.1 + delta_y,
                current.2 + delta_z,
            );
            if c.0 < x_min - 1
                || c.0 > x_max + 1
                || c.1 < y_min - 1
                || c.1 > y_max + 1
                || c.2 < z_min - 1
                || c.2 > z_max + 1
            {
                continue;
            }
            if !cube_set.contains(&c) {
                queue.push(c);
            }
        }
    }
    true
}
