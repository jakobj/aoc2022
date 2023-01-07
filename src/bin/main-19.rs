use std::{fs, collections::{HashMap, BinaryHeap, HashSet}, cmp::Ordering};

use regex::Regex;

fn main() {
    let filename = "inputs/19b.txt";
    let content = fs::read_to_string(filename).unwrap();
    let blueprints = parse_blueprints(&content);

    let mut total_quality_level = 0;
    for (idx, bp) in blueprints.iter().enumerate() {
        let max_opened_geodes = determine_max_opened_geodes(&bp);
        let quality_level = (idx + 1) * max_opened_geodes;
        println!("{}: {}", idx + 1, quality_level);
        total_quality_level += quality_level;
    }
    println!("The total quality level of all blueprints together is {}.", total_quality_level);
}

fn parse_blueprints(content: &str) -> Vec<HashMap::<String, [usize; 3]>> {
    let mut blueprints = Vec::new();
    let re = Regex::new(
        r"Each (ore|clay|obsidian|geode) robot costs ([0-9]+) ore( and ([0-9]+) (clay|obsidian))?",
    ).unwrap();
    for l in content.lines() {
        if l.len() == 0 {
            continue;
        }
        let split = l.split(".");

        let mut bp = HashMap::new();
        for e in split.into_iter() {
            if e.len() == 0 {
                continue;
            }
            let caps = re.captures(e).unwrap();
            let robot_type = &caps[1];
            let ore_cost = caps[2].parse::<usize>().unwrap();
            let clay_cost;
            let obsidian_cost;
            if caps.get(5).is_some() {
                if &caps[5] == "clay" {
                    clay_cost = caps[4].parse::<usize>().unwrap();
                    obsidian_cost = 0;
                } else {
                    clay_cost = 0;
                    obsidian_cost = caps[4].parse::<usize>().unwrap();
                }
            } else {
                clay_cost = 0;
                obsidian_cost = 0;
            }

            bp.insert(robot_type.to_string(), [ore_cost, clay_cost, obsidian_cost]);
        }
        blueprints.push(bp);
    }
    blueprints
}

fn determine_max_opened_geodes(robot_costs: &HashMap<String, [usize; 3]>) -> usize {
    let max_time = 24;
    let mut opened_geodes = 0;
    let mut queue = Vec::with_capacity(10_000_000);
    queue.push(State::new());
    let mut visited: HashMap<String, State> = HashMap::with_capacity(10_000_000);
    while let Some(current) = queue.pop() {
        if current.minute > max_time {
            continue;
        }

        // check if we reached this state (minute, robots) before with more
        // resources; if so, we don't need to consider this state
        let s = current.to_string();
        if visited.contains_key(&s) {
            if current.resources[0] <= visited[&s].resources[0] &&
                current.resources[1] <= visited[&s].resources[1] &&
                current.resources[2] <= visited[&s].resources[2] &&
                current.resources[3] <= visited[&s].resources[3]{
                    continue;
                }
        }
        visited.insert(s, current.clone());

        // tick
        let minute = current.minute + 1;

        if current.resources[3] > opened_geodes {
            // println!("{:?}", current);
            opened_geodes = current.resources[3];
        }

        // add state without building robots
        let mut resources = current.resources.clone();
        for i in 0..resources.len() {
            resources[i] += current.robots[i];
        }
        queue.push(State{
            minute,
            resources,
            robots: current.robots.clone(),
        });

        // add states with building robots
        for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().enumerate() {
            if current.resources[0] >= robot_costs[robot_type][0] &&
                current.resources[1] >= robot_costs[robot_type][1] &&
                current.resources[2] >= robot_costs[robot_type][2] {
                    // build robots
                    let mut resources = current.resources.clone();
                    for i in 0..resources.len() - 1 {
                        resources[i] -= robot_costs[robot_type][i];
                    }
                    // gather resources
                    for i in 0..resources.len() {
                        resources[i] += current.robots[i];
                    }

                    // robot is done
                    let mut robots = current.robots.clone();
                    robots[robot_idx] += 1;

                    queue.push(State{
                        minute,
                        resources,
                        robots,
                    });
                }
        }
    }
    opened_geodes
}

#[derive(Debug, Clone, Copy, Hash)]
struct State {
    minute: usize,
    resources: [usize; 4],
    robots: [usize; 4],
}

impl State {
    fn new() -> Self {
        State { minute: 0, resources: [0; 4], robots: [1, 0, 0, 0] }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.minute.to_string());
        // s.push_str(&self.resources.iter().map(|d| d.to_string()).collect::<String>());
        s.push_str(&self.robots.iter().map(|d| d.to_string()).collect::<String>());
        s
    }
}
