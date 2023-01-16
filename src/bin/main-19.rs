use std::{cmp::Ordering, collections::{HashMap, BinaryHeap, HashSet}, fmt::Binary, fs};

use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng,SeedableRng};
use regex::Regex;

fn main() {
    let filename = "inputs/19.txt";
    let content = fs::read_to_string(filename).unwrap();
    let blueprints = parse_blueprints(&content);

    let max = best_path(&State::new(), 32, &blueprints[0]);
    println!("{}\n=====", max);
    let max = best_path(&State::new(), 32, &blueprints[1]);
    println!("{}\n=====", max);
    let max = best_path(&State::new(), 32, &blueprints[2]);
    println!("{}\n=====", max);

    // let max = dfs(&State::new(), 24, &blueprints[0]);
    // println!("{}", max);

    // TODO debug mcts
    // TODO alternative search? depth first? greedy? how to pick nodes?

    // println!("{:?}", blueprints[0]);
    // let current = State{ minute: 13, resources: [0, 0, 0, 0], robots: [1, 1, 1, 1] };
    // let n = generate_neighbors2(&current, &blueprints[0]);
    // for s in n {
    //     println!("{:?}", s);
    // }

    // let max_time = 24;
    // // let max_time = 32;
    // let mut initial_states = vec![State::new()];
    // for target_geode_count in 1.. {
    //     let mut new_states = Vec::new();
    //     for state in initial_states.iter() {
    //         let mut s = shortest_path(state, target_geode_count, max_time, &blueprints[0]);
    //         new_states.append(&mut s);
    //     }
    //     if new_states.len() == 0 {
    //         println!("couldn't find any state with {} geodes", target_geode_count);
    //         break;
    //     }
    //     // let min_minute = new_states.iter().map(|s| s.minute).min().unwrap();
    //     let max_geodes = new_states.iter().map(|s| s.resources[3]).max().unwrap();
    //     println!("{}", max_geodes);
    //     // initial_states = new_states.into_iter().filter(|s| s.minute == min_minute).collect::<Vec<State>>();
    //     initial_states = new_states;
    //     // for s in initial_states.iter() {
    //     //     println!("{}: {:?}", target_geode_count, s);
    //     // }
    // }

    // let max_time = 24;
    // let max_time = 32;
    // for (i, robot_costs) in blueprints.iter().enumerate() {
    //     let mut max_geodes = 0;
    //     let intermediate = shortest_path(&State::new(), 1, max_time, robot_costs);
    //     // println!("{:?}", intermediate);
    //     for s in intermediate {
    //         let best = best_path(&s, max_time, robot_costs);
    //         if best > max_geodes {
    //             max_geodes = best;
    //         }
    //     }
    //     println!("{}: {}", i, max_geodes);
    // }

    // search(&State::new(), 32, &blueprints[0]);
    // search(&State::new(), 32, &blueprints[1]);
    // search(&State::new(), 32, &blueprints[2]);
    // for bp in blueprints {
    //     search(&State::new(), 32, &bp);
    //     println!("");
    // }
    // search(&State::new(), 32, &blueprints[0]);
    // println!("");
    // search(&State::new(), 32, &blueprints[1]);

    // let mut total_quality_level = 0;
    // for (idx, bp) in blueprints.iter().enumerate() {
    //     let max_opened_geodes = determine_max_opened_geodes2(&bp);
    //     println!("{} {}", idx + 1, max_opened_geodes);
    //     let quality_level = (idx + 1) * max_opened_geodes;
    //     // println!("{}: {}", idx + 1, quality_level);
    //     total_quality_level += quality_level;
    // }
    // println!("The total quality level of all blueprints together is {}.", total_quality_level);
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

// fn determine_max_opened_geodes(robot_costs: &HashMap<String, [usize; 3]>) -> usize {
//     // let max_time = 24;
//     let max_time = 32;
//     let mut opened_geodes = 0;
//     // let mut queue = Vec::with_capacity(10_000_000);
//     let mut queue = BinaryHeap::new();
//     queue.push(State::new());
//     let mut visited: HashMap<String, State> = HashMap::with_capacity(10_000_000);
//     let mut iteration: usize = 0;
//     let mut best = (0, 0, 0);
//     'outer: while let Some(current) = queue.pop() {
//         // println!("{:?}", current);
//         iteration += 1;
//         if iteration == 5_000_000_000 {
//             break;
//         }
//         if current.minute > max_time {
//             continue;
//         }

//         if current.minute >= best.0 && current.resources[3] < best.1 && current.robots[3] < best.2 {
//             continue;
//         }
//         best = (current.minute, current.resources[3], current.robots[3]);

//         // check if we reached this state (minute, robots) before with more
//         // resources; if so, we don't need to consider this state
//         let s = current.to_string();
//         if visited.contains_key(&s) {
//             if current.resources[0] <= visited[&s].resources[0] &&
//                 current.resources[1] <= visited[&s].resources[1] &&
//                 current.resources[2] <= visited[&s].resources[2] &&
//                 current.resources[3] <= visited[&s].resources[3] {
//                     continue;
//                 }
//         }
//         visited.insert(s, current.clone());

//         // tick
//         let minute = current.minute + 1;

//         if current.resources[3] > opened_geodes {
//             // println!("{} {:?}", iteration, current);
//             opened_geodes = current.resources[3];
//         }

//         // add states with building robots
//         // for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().enumerate() {
//         for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().rev().enumerate() {
//             let robot_idx = 3 - robot_idx;
//             if current.resources[0] >= robot_costs[robot_type][0] &&
//                 current.resources[1] >= robot_costs[robot_type][1] &&
//                 current.resources[2] >= robot_costs[robot_type][2] {
//                     // build robots
//                     let mut resources = current.resources.clone();
//                     for i in 0..resources.len() - 1 {
//                         resources[i] -= robot_costs[robot_type][i];
//                     }
//                     // gather resources
//                     for i in 0..resources.len() {
//                         resources[i] += current.robots[i];
//                     }

//                     // robot is done
//                     let mut robots = current.robots.clone();
//                     robots[robot_idx] += 1;

//                     queue.push(State{
//                         minute,
//                         resources,
//                         robots,
//                     });

//                     // if we can build a genode breaking robot, do it and skip
//                     // the other options
//                     if robot_type == "geode" {
//                         continue 'outer;
//                     }
//                 }
//         }

//         // add state without building robots
//         let mut resources = current.resources.clone();
//         for i in 0..resources.len() {
//             resources[i] += current.robots[i];
//         }
//         queue.push(State{
//             minute,
//             resources,
//             robots: current.robots.clone(),
//         });
//     }
//     opened_geodes
// }

// #[derive(Debug, Clone, Copy, Hash)]
// struct State {
//     minute: usize,
//     resources: [usize; 4],
//     robots: [usize; 4],
// }

// impl State {
//     fn new() -> Self {
//         State { minute: 0, resources: [0; 4], robots: [1, 0, 0, 0] }
//     }

//     // fn to_string(&self) -> String {
//     //     // let mut s = String::new();
//     //     // s.push_str(&self.minute.to_string());
//     //     // // s.push_str(&self.resources.iter().map(|d| d.to_string()).collect::<String>());
//     //     // s.push_str(&self.robots.iter().map(|d| d.to_string()).collect::<String>());
//     //     // s
//     //     format!("{},{},{},{},{}", self.minute, self.robots[0], self.robots[1], self.robots[2], self.robots[3])
//     // }
// }

// impl Eq for State {}

// impl PartialEq for State {
//     fn eq(&self, other: &Self) -> bool {
//         // self.robots.eq(&other.robots) &&
//         // self.resources.eq(&other.resources) &&
//         // self.minute.eq(&other.minute)
//         self.robots[3].eq(&other.robots[3]) &&
//         self.resources[3].eq(&other.resources[3]) &&
//         self.minute.eq(&other.minute)
//     }
// }

// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // let s_self = [
//         //     self.robots[3], self.robots[2], self.robots[1], self.robots[0],
//         //     self.resources[3], self.resources[2], self.resources[1], self.resources[0],
//         //     usize::MAX-self.minute
//         // ];
//         // let s_other = [
//         //     other.robots[3], other.robots[2], other.robots[1], other.robots[0],
//         //     other.resources[3], other.resources[2], other.resources[1], other.resources[0],
//         //     usize::MAX-other.minute
//         // ];
//         let s_self = [
//             self.robots[3],
//             self.resources[3],
//             usize::MAX-self.minute
//         ];
//         let s_other = [
//             other.robots[3],
//             other.resources[3],
//             usize::MAX-other.minute
//         ];
//         s_self.cmp(&s_other)
//     }
// }

// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // let s_self = [
//         //     self.robots[3], self.robots[2], self.robots[1], self.robots[0],
//         //     self.resources[3], self.resources[2], self.resources[1], self.resources[0],
//         //     usize::MAX-self.minute
//         // ];
//         // let s_other = [
//         //     other.robots[3], other.robots[2], other.robots[1], other.robots[0],
//         //     other.resources[3], other.resources[2], other.resources[1], other.resources[0],
//         //     usize::MAX-other.minute
//         // ];
//         let s_self = [
//             self.robots[3],
//             self.resources[3],
//             usize::MAX-self.minute
//         ];
//         let s_other = [
//             other.robots[3],
//             other.resources[3],
//             usize::MAX-other.minute
//         ];
//         Some(s_self.cmp(&s_other))
//     }
// }

// fn determine_max_opened_geodes2(robot_costs: &HashMap<String, [usize; 3]>) -> usize {
//     let max_time = 32;
//     let mut states = vec![State::new(); max_time + 1];
//     for minute in 0..max_time + 1 {
//         states[minute].minute = minute;
//     }

//     // TODO new strategy: always make end of rollout deterministic? (requires
//     // caching?)

//     let mut rng = StdRng::seed_from_u64(1234);

//     let mut best_final_state = State::new();
//     for idx in 0..states.len() - 1 {
//         let current = states[idx];

//         let mut best_reachable_state = State::new();
//         for state in generate_neighbors(&current, robot_costs) {
//             for _ in 0..1_000_000 {
//                 let reachable_state = stochastic_rollout(&state, max_time, robot_costs, &mut rng);
//                 if reachable_state.resources[3] > best_reachable_state.resources[3] {
//                     best_reachable_state = reachable_state.clone();
//                     states[idx + 1] = state.clone();
//                 }
//                 if reachable_state.resources[3] > best_final_state.resources[3] {
//                     println!("{:?}->{:?}...->{:?}", current, states[idx + 1], reachable_state);
//                     best_final_state = reachable_state.clone();
//                 }
//             }
//         }
//     }

//     best_final_state.resources[3]
// }

// fn generate_neighbors(current: &State, robot_costs: &HashMap<String, [usize; 3]>) -> Vec<Option<State>> {
//     let mut neighbors = Vec::with_capacity(5);

//     // generate state without building robots
//     let mut resources = current.resources.clone();
//     for i in 0..resources.len() {
//         resources[i] += current.robots[i];
//     }
//     let state = State{
//         minute: current.minute + 1,
//         resources,
//         robots: current.robots.clone(),
//     };
//     neighbors.push(Some(state));

//     for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().enumerate() {
//         if current.resources[0] >= robot_costs[robot_type][0] &&
//             current.resources[1] >= robot_costs[robot_type][1] &&
//             current.resources[2] >= robot_costs[robot_type][2] {
//                 // build robots
//                 let mut resources = current.resources.clone();
//                 for i in 0..resources.len() - 1 {
//                     resources[i] -= robot_costs[robot_type][i];
//                 }
//                 // gather resources
//                 for i in 0..resources.len() {
//                     resources[i] += current.robots[i];
//                 }

//                 // robot is done
//                 let mut robots = current.robots.clone();
//                 robots[robot_idx] += 1;

//                 let state = State{
//                     minute: current.minute + 1,
//                     resources,
//                     robots,
//                 };
//                 neighbors.push(Some(state));
//             } else {
//                 neighbors.push(None);
//             }
//     }

//     neighbors
// }

fn generate_neighbors2(current: &State, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> Vec<Option<State>> {
    let mut neighbors = Vec::with_capacity(5);

    // generate state without building robots
    let remaining = max_time - current.minute;
    let mut resources = current.resources.clone();
    for i in 0..resources.len() {
        resources[i] += remaining * current.robots[i];
    }
    let state = State{
        minute: current.minute + remaining,
        resources,
        robots: current.robots.clone(),
    };
    neighbors.push(Some(state));

    'outer: for (robot_idx, &robot_type) in ["ore", "clay", "obsidian", "geode"].iter().enumerate() {
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

        let state = State{
            minute: current.minute + delta_minute,
            resources,
            robots,
        };
        neighbors.push(Some(state));
    }

    neighbors
}

// fn search(initial_state: &State, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> State {
//     let mut root = Node::new(initial_state);
//     let mut rng = StdRng::seed_from_u64(1234);
//     let mut best = 0;
//     for round in 0..50_000_000 {
//         // println!("\n round {}", round);
//         // print_node(&Some(root.clone()), 0);
//         // selection
//         let mut node_indices = Vec::new();
//         let leaf = select_leaf(&mut root, &mut node_indices, &mut rng);
//         if leaf.state.minute < max_time {
//             // expansion
//             let new = expand(leaf, &mut node_indices, max_time, robot_costs);
//             assert!(new.total_playouts == 0);
//             // simulation
//             let result = stochastic_rollout(&new.state, max_time, robot_costs, &mut rng);
//             if result.resources[3] > best {
//                 println!("{:?}", result);
//                 best = result.resources[3];
//             }
//             // backpropagation
//             backprop(&mut root, &node_indices, result);
//         } else {
//             assert!(leaf.state.minute == max_time);
//             let result = leaf.state.clone();
//             backprop(&mut root, &node_indices, result);
//         }
//     }
//     // print_node(&Some(root), 0);
//     State::new()
// }

// #[derive(Debug, Clone)]
// struct Node {
//     state: State,
//     children: Vec<Option<Self>>,
//     score: usize,
//     total_playouts: usize,
// }

// impl Node {
//     fn new(state: &State) -> Self {
//         Node { state: state.clone(), children: Vec::new(), score: 0, total_playouts: 0 }
//     }
// }

// fn select_leaf<'a>(parent: &'a mut Node, node_indices: &mut Vec<usize>, rng: &mut StdRng) -> &'a mut Node {
//     if parent.children.len() == 0 {
//         return parent;
//     }

//     for c in parent.children.iter_mut() {
//         if let Some(c) = c {
//             if c.total_playouts == 0 {
//                 return parent;
//             }
//         }
//     }

//     let ucts = parent.children.iter().enumerate().filter(|(_i, s)| s.is_some()).map(|(i, s)| (i, compute_uct(parent, s.as_ref().unwrap()))).collect::<Vec<(usize, f64)>>();
//     let max_uct = ucts.iter().map(|(_i, uct)| uct).max_by(|a, b| a.total_cmp(b)).unwrap();
//     let max_indices = ucts.iter().filter(|(_i, uct)| uct >= max_uct).map(|(i, _uct)| *i).collect::<Vec<usize>>();
//     let idx = *max_indices.choose(rng).unwrap();

//     node_indices.push(idx);
//     select_leaf(parent.children[idx].as_mut().unwrap(), node_indices, rng)
// }

// fn compute_uct(parent: &Node, child: &Node) -> f64 {
//     if child.total_playouts == 0 {
//         return f64::MAX;
//     }
//     let max_score = 500_f64;
//     let w = child.score as f64 / max_score;
//     let t0 = w / child.total_playouts as f64;
//     assert!(0.0 <= t0 && t0 <= 1.0);
//     t0 + 2_f64.sqrt() * ((parent.total_playouts as f64).ln() / child.total_playouts as f64).sqrt()
//     // parent.total_playouts as f64 / child.total_playouts as f64
//     // 1.0
// }

// fn expand<'a>(leaf: &'a mut Node, node_indices: &mut Vec<usize>, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> &'a mut Node {
//     if leaf.children.len() == 0 {
//         let neighbors = generate_neighbors2(&leaf.state, max_time, robot_costs);
//         for n in neighbors {
//             if n.is_some() && n.as_ref().unwrap().minute <= max_time {
//                 leaf.children.push(Some(Node::new(&n.unwrap())));
//             } else {
//                 leaf.children.push(None);
//             }
//         }
//     }

//     for (i, c) in leaf.children.iter_mut().enumerate() {
//         if let Some(c) = c {
//             if c.total_playouts == 0 {
//                 node_indices.push(i);
//                 return c;
//             }
//         }
//     }

//     panic!("should never be reached");
// }

// fn backprop(root: &mut Node, node_indices: &Vec<usize>, result: State) {
//     root.score += result.resources[3];
//     root.total_playouts += 1;
//     let mut n = root;
//     for i in node_indices {
//         n = n.children[*i].as_mut().unwrap();
//         n.score += result.resources[3];
//         n.total_playouts += 1;
//     }
// }

// fn print_node(node: &Option<Node>, offset: usize) {
//     for _ in 0..offset {
//         print!(" ");
//     }
//     if let Some(node) = node {
//         println!("{:?} {} {}", node.state, node.score, node.total_playouts);
//     } else {
//         println!("{:?}", node);
//     }
//     if let Some(node) = node {
//         for c in node.children.iter() {
//             print_node(c, offset + 2);
//         }
//     }
// }

// fn deterministic_rollout(initial_state: &State, depth: usize, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> State {
//     if depth == 0 || initial_state.minute >= max_time {
//         return initial_state.clone();
//     }
//     let mut best_final_state = State::new();
//     let m = best_final_state.resources.clone().into_iter().rev().collect::<Vec<usize>>();
//     for state in generate_neighbors2(initial_state, robot_costs) {
//         let final_state = deterministic_rollout(&state, depth - 1, max_time, robot_costs);
//         let n = final_state.resources.clone().into_iter().rev().collect::<Vec<usize>>();
//         if n > m {
//             best_final_state = final_state;
//         }

//     }
//     best_final_state
// }

// fn stochastic_rollout(initial_state: &State, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>, rng: &mut StdRng) -> State {
//     // TODO deterministic rollout if close to finish with lookup table?
//     if initial_state.minute >= max_time {
//         return initial_state.clone();
//     }
//     assert!(initial_state.minute < max_time);
//     let neighbors = generate_neighbors2(initial_state, max_time, robot_costs);
//     let some_indices = neighbors.iter().enumerate().filter(|(_i, s)| s.is_some() && s.as_ref().unwrap().minute <= max_time).map(|(i, _s)| i).collect::<Vec<usize>>();
//     let idx = some_indices.choose(rng).unwrap();
//     let state = neighbors[*idx];
//     assert!(state.as_ref().unwrap().minute <= max_time);
//     stochastic_rollout(state.as_ref().unwrap(), max_time, robot_costs, rng)
// }

// fn shortest_path(initial_state: &State, target_geode_count: usize, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> Vec<State> {
//     let mut front = Vec::new();
//     let mut queue = BinaryHeap::new();
//     queue.push(initial_state.clone());
//     let mut visited = HashSet::new();
//     let mut best_time = usize::MAX - 1;
//     while let Some(current) = queue.pop() {
//         if current.minute > max_time || current.minute > best_time + 1{
//             continue;
//         }

//         let hsh = current.to_string();
//         if visited.contains(&hsh) {
//             continue;
//         }
//         visited.insert(hsh);

//         if current.robots[3] == target_geode_count {
//             if current.minute < best_time {
//                 best_time = current.minute;
//             }
//             front.push(current);
//             continue;
//         }

//         for neighbor in generate_neighbors(&current, robot_costs) {
//             if let Some(state) = neighbor {
//                 queue.push(state);
//             }
//         }
//     }
//     front
// }

fn best_path(initial_state: &State, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> usize {
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
            println!("{} {}", max_geodes, queue.len());
        }

        for neighbor in generate_neighbors2(&current, max_time, robot_costs) {
            if let Some(state) = neighbor {
                let mut r = state.robots[3];
                let mut m = state.resources[3];
                for _ in state.minute..max_time {
                    m += r;
                    r += 1;
                }
                if m >= max_geodes {
                    queue.push(state);
                }
            }
        }
    }
    max_geodes
}

fn dfs(initial_state: &State, max_time: usize, robot_costs: &HashMap<String, [usize; 3]>) -> usize {
    assert!(initial_state.minute <= max_time);
    if initial_state.minute == max_time {
        return initial_state.resources[3];
    }

    let mut max_geodes = 0;
    for neighbor in generate_neighbors2(initial_state, max_time, robot_costs) {
        if let Some(state) = neighbor {
            let mut r = state.robots[3];
            let mut m = state.resources[3];
            for _ in state.minute..max_time {
                m += r;
                r += 1;
            }
            if m >= max_geodes {
                let geodes = dfs(&state, max_time, robot_costs);
                if geodes > max_geodes {
                    max_geodes = geodes;
                }
            }
        }
    }
    max_geodes

    // let mut queue = BinaryHeap::new();
    // queue.push(initial_state.clone());
    // let mut visited = HashSet::new();
    // let mut max_geodes = 0;
    // while let Some(current) = queue.pop() {
    //     if current.minute > max_time {
    //         continue;
    //     }

    //     if visited.contains(&current) {
    //         continue;
    //     }
    //     visited.insert(current.clone());

    //     if current.resources[3] > max_geodes {
    //         max_geodes = current.resources[3];
    //         println!("{} {}", max_geodes, queue.len());
    //     }

    //     for neighbor in generate_neighbors2(&current, max_time, robot_costs) {
    //         if let Some(state) = neighbor {
    //             let mut r = state.robots[3];
    //             let mut m = state.resources[3];
    //             for _ in state.minute..max_time {
    //                 m += r;
    //                 r += 1;
    //             }
    //             if m >= max_geodes {
    //                 queue.push(state);
    //             }
    //         }
    //     }
    // }
    // max_geodes
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

    // fn to_string(&self) -> String {
    //     let mut s = String::new();
    //     s.push_str(&self.minute.to_string());
    //     s.push_str(&self.resources.iter().map(|d| d.to_string()).collect::<String>());
    //     s.push_str(&self.robots.iter().map(|d| d.to_string()).collect::<String>());
    //     s
    // }
}

// impl Eq for State {}

// impl PartialEq for State {
//     fn eq(&self, other: &Self) -> bool {
//         other.minute.eq(&self.minute)
//     }
// }

// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.minute.cmp(&self.minute)
//     }
// }

// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(other.minute.cmp(&self.minute))
//     }
// }

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
