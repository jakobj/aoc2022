use std::fs;

use regex::Regex;

fn main() {
    let filename = "inputs/15.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let sensors_and_beacons = parse_sensors_and_beacons(content);
    let n_covered = count_covered_positions(&sensors_and_beacons, 2000000);
    println!(
        "There are {} positions where a beacon can not be present.",
        n_covered
    );
    let position = find_beacon(&sensors_and_beacons);
    let tuning_frequency = position.0 * 4_000_000 + position.1;
    println!(
        "The beacon is located at {:?} and its tuning frequency is {}.",
        position, tuning_frequency
    );
}

fn parse_sensors_and_beacons(content: String) -> Vec<((i64, i64), (i64, i64))> {
    let mut sb = Vec::new();
    let re = Regex::new(
        r"Sensor at x=([0-9]+), y=([0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)",
    )
    .unwrap();
    for l in content.lines() {
        let caps = re.captures(l).unwrap();
        let s = (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        );
        let b = (
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        );
        sb.push((s, b));
    }
    sb
}

fn determine_x_min_max(sensors_and_beacons: &Vec<((i64, i64), (i64, i64))>) -> (i64, i64) {
    let mut x_min = i64::MAX;
    let mut x_max = -i64::MAX;
    for (s, b) in sensors_and_beacons.iter() {
        let x_s = s.0;
        let dist = distance(s, b) as i64;
        if x_min > x_s - dist {
            x_min = x_s - dist;
        }
        if x_max < x_s + dist {
            x_max = x_s + dist;
        }
    }
    (x_min, x_max)
}

fn distance(p0: &(i64, i64), p1: &(i64, i64)) -> usize {
    ((p0.0 - p1.0).abs() + (p0.1 - p1.1).abs()) as usize
}

fn count_covered_positions(sensors_and_beacons: &Vec<((i64, i64), (i64, i64))>, y: i64) -> usize {
    let (x_min, x_max) = determine_x_min_max(sensors_and_beacons);
    let mut n_covered = 0;
    let mut x = x_min;
    while x < x_max + 1 {
        let p = (x, y);
        if is_beacon(sensors_and_beacons, &p) {
            x += 1;
            continue;
        }
        let (covered, delta_x) = determine_covered_and_delta_x(sensors_and_beacons, &p);
        if covered {
            n_covered += delta_x as usize;
        }
        x += delta_x;
    }
    n_covered
}

fn is_beacon(sensors_and_beacons: &Vec<((i64, i64), (i64, i64))>, p: &(i64, i64)) -> bool {
    for (_s, b) in sensors_and_beacons.iter() {
        if *p == *b {
            return true;
        }
    }
    false
}

fn determine_covered_and_delta_x(
    sensors_and_beacons: &Vec<((i64, i64), (i64, i64))>,
    p: &(i64, i64),
) -> (bool, i64) {
    let mut x_new = None;
    for (s, b) in sensors_and_beacons.iter() {
        let dist = distance(s, &p);
        if dist > distance(s, b) {
            // position not covered by this beacon
            continue;
        }
        // potential new x value is largest x value still covered by sensor at
        // this y value
        let tmp = s.0 + (distance(s, b) - (s.1 - p.1).abs() as usize) as i64;
        if x_new.is_none() || (tmp > x_new.unwrap()) {
            x_new = Some(tmp);
        }
    }
    if x_new.is_some() {
        // position covered by some beacon
        let x_new = x_new.unwrap();
        let delta_x = std::cmp::max(1, x_new - p.0);
        return (true, delta_x);
    } else {
        return (false, 1);
    }
}

fn find_beacon(sensors_and_beacons: &Vec<((i64, i64), (i64, i64))>) -> (i64, i64) {
    for y in 0..4_000_000 {
        let mut x = 0;
        while x < 4_000_000 {
            let p = (x, y);
            let (covered, delta_x) = determine_covered_and_delta_x(sensors_and_beacons, &p);
            if !covered {
                return p;
            }
            x += delta_x;
        }
    }
    panic!("Beacon not found");
}
