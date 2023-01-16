use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

use regex::Regex;

fn main() {
    let filename = "inputs/19.txt";
    let content = fs::read_to_string(filename).unwrap();
    let blueprints = parse_blueprints(&content);

    let mut total_quality_level = 0;
    for (id, bp) in blueprints.iter().enumerate() {
        let max_geodes = determine_max_geodes(&State::new(), 24, bp);
        total_quality_level += (id + 1) * max_geodes;
    }
    println!("The total quality level is {}.", total_quality_level);

    let mut product = 1;
    for i in 0..3 {
        let max_geodes = determine_max_geodes(&State::new(), 32, &blueprints[i]);
        product *= max_geodes;
    }
    println!("The product of the first three blueprints is {}.", product);
}

fn parse_blueprints(content: &str) -> Vec<HashMap<String, [usize; 3]>> {
    let mut blueprints = Vec::new();
    let re = Regex::new(
        r"Each (ore|clay|obsidian|geode) robot costs ([0-9]+) ore( and ([0-9]+) (clay|obsidian))?",
    )
    .unwrap();
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

fn determine_max_geodes(
    initial_state: &State,
    max_time: usize,
    robot_costs: &HashMap<String, [usize; 3]>,
) -> usize {
    // maintain a queue of states sorted by their resources (effectively
    // implements a kind of DFS for resources; also see `Ordering`
    // implementation for `State` below; doing a DFS in resources is used to
    // prune lots of branches)
    let mut queue = BinaryHeap::new();
    queue.push(initial_state.clone());
    let mut visited = HashSet::new();
    let mut max_geodes = 0;
    while let Some(current) = queue.pop() {
        if current.minute > max_time {
            continue;
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        if current.resources[3] > max_geodes {
            max_geodes = current.resources[3];
        }

        for neighbor in generate_neighbors(&current, max_time, robot_costs) {
            if let Some(neighbor) = neighbor {
                // prune branches: only add nodes from which it is possible to
                // beat the current maximum by optimal production of geodes
                let upper_bound = compute_upper_bound(&neighbor, max_time);
                if upper_bound >= max_geodes {
                    queue.push(neighbor);
                }
            }
        }
    }
    max_geodes
}

#[derive(Debug, Clone, Copy, Hash)]
struct State {
    minute: usize,
    resources: [usize; 4],
    robots: [usize; 4],
}

impl State {
    fn new() -> Self {
        State {
            minute: 0,
            resources: [0; 4],
            robots: [1, 0, 0, 0],
        }
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let s = [self.resources[3], self.resources[2], self.resources[1]];
        let o = [other.resources[3], other.resources[2], other.resources[1]];
        s.eq(&o)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = [self.resources[3], self.resources[2], self.resources[1]];
        let o = [other.resources[3], other.resources[2], other.resources[1]];
        s.cmp(&o)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = [self.resources[3], self.resources[2], self.resources[1]];
        let o = [other.resources[3], other.resources[2], other.resources[1]];
        Some(s.cmp(&o))
    }
}

fn generate_neighbors(
    current: &State,
    max_time: usize,
    robot_costs: &HashMap<String, [usize; 3]>,
) -> Vec<Option<State>> {
    let mut neighbors = Vec::with_capacity(5);

    // generate state without building robots
    let remaining = max_time - current.minute;
    let mut resources = current.resources.clone();
    for i in 0..resources.len() {
        resources[i] += remaining * current.robots[i];
    }
    let state = State {
        minute: current.minute + remaining,
        resources,
        robots: current.robots.clone(),
    };
    neighbors.push(Some(state));

    // generate states with building robots
    'outer: for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().enumerate()
    {
        let mut max_rounds = 0;
        for i in 0..3 {
            if robot_costs[robot_type][i] > current.resources[i] {
                let missing = robot_costs[robot_type][i] - current.resources[i];
                if current.robots[i] > 0 {
                    let mut rounds = missing / current.robots[i];
                    if missing % current.robots[i] != 0 {
                        rounds += 1;
                    }
                    if rounds > max_rounds {
                        max_rounds = rounds;
                    }
                } else {
                    neighbors.push(None);
                    continue 'outer;
                }
            }
        }
        let delta_minute = max_rounds + 1;
        if current.minute + delta_minute > max_time {
            neighbors.push(None);
            continue;
        }
        let mut resources = current.resources.clone();
        for i in 0..4 {
            resources[i] += delta_minute * current.robots[i];
        }
        for i in 0..3 {
            assert!(resources[i] >= robot_costs[robot_type][i]);
            resources[i] -= robot_costs[robot_type][i];
        }
        let mut robots = current.robots.clone();
        robots[robot_idx] += 1;

        let state = State {
            minute: current.minute + delta_minute,
            resources,
            robots,
        };
        neighbors.push(Some(state));
    }

    neighbors
}

fn compute_upper_bound(state: &State, max_time: usize) -> usize {
    let mut r = state.robots[3];
    let mut m = state.resources[3];
    for _ in state.minute..max_time {
        m += r;
        r += 1;
    }
    m
}
