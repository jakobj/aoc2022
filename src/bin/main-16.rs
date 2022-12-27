use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs};

use rand::{distributions::Uniform, Rng, SeedableRng};
use rand::distributions::Distribution;
use regex::Regex;

// 2071 -- 3056

// static WITH_ELEPHANT: bool = true;
// static MAX_TIME: usize = 26;
static WITH_ELEPHANT: bool = false;
static MAX_TIME: usize = 30;

// AA II JJ JJo II AA BB BBo CC CCo
// AA DD DDo EE FF GG HH HHo GG FF EE EEo

fn main() {
    let filename = "inputs/16b.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let nodes = parse_graph(content);
    let nodes = determine_effective_graph(&nodes);
    let most_releasable_pressure = determine_most_releasable_pressure(nodes);
    println!("The most pressure possible to release is {}.", most_releasable_pressure);
}

fn parse_graph(content: String) -> HashMap<String, Node> {
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=([0-9]{1,2}); tunnel[s]? lead[s]? to valve[s]? ([A-Z ,]+)",
    ).unwrap();
    let mut nodes = HashMap::new();
    for l in content.lines() {
        let caps = re.captures(l).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let mut edges = caps.get(3).unwrap().as_str().split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        if rate > 0 {
            // if a valve has a non-zero rate when it's opened, add it as an
            // additional node in the graph reachable only from the respective
            // closed-valve node
            let node_open = Node{ rate, edges: edges.clone() };
            let label_open = label.clone() + "o";
            nodes.insert(label_open.clone(), node_open);
            edges.push(label_open);
            let node_closed = Node{ rate: 0, edges };
            nodes.insert(label, node_closed);
        } else {
            nodes.insert(label, Node{ rate: 0, edges });
        }
    }
    nodes
}

#[derive(Clone, Debug)]
struct Node {
    rate: usize,
    edges: Vec<String>,
}

fn determine_effective_graph(nodes: &HashMap<String, Node>) -> HashMap<String, EffectiveNode> {
    let mut effective_nodes = HashMap::new();
    let mut weights: HashMap<(String, String), usize> = HashMap::new();
    for source in nodes.keys() {
        let mut edges = Vec::new();
        for target in nodes.keys() {
            if source == target {
                continue;
            }

            let weight;
            let key = (source.clone(), target.clone());
            if weights.contains_key(&key) {
                weight = weights[&key];
            } else {
                weight = determine_length_of_shortest_path(source, target, nodes);
                weights.insert(key, weight);
            }
            edges.push((target.clone(), weight));
        }
        effective_nodes.insert(source.clone(), EffectiveNode{ rate: nodes[source].rate, edges });
    }
    effective_nodes
}

#[derive(Clone, Debug)]
struct EffectiveNode {
    rate: usize,
    edges: Vec<(String, usize)>,
}

fn determine_length_of_shortest_path(source: &str, target: &str, nodes: &HashMap<String, Node>) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Path{ label: source.to_string(), length: 0});
    let mut visited = HashSet::new();
    while let Some(current) = queue.pop() {
        if current.label == target {
            return current.length;
        }
        if visited.contains(&current.label) {
            continue;
        }
        visited.insert(current.label.clone());
        for edge in nodes[&current.label].edges.iter() {
            queue.push(Path{ label: edge.clone(), length: current.length + 1 });
        }
    }
    panic!("No path found ({} -> {})", source, target);
}

struct Path{
    label: String,
    length: usize,
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.length.eq(&other.length)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.length.cmp(&self.length)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.length.cmp(&self.length))
    }
}

fn determine_most_releasable_pressure(nodes: HashMap<String, EffectiveNode>) -> usize {
    let mut sequence = nodes.iter().map(|(l, _n)| l.to_string()).filter(|l| l.chars().last().unwrap() == 'o').collect::<Vec<String>>();
    sequence.sort_unstable();
    let mut old_pressure = 0;
    let mut max_pressure = 0;
    let between = Uniform::from(0..sequence.len());
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
    for _a in 0..10_000_000 {

        let i = between.sample(&mut rng);
        let j = between.sample(&mut rng);
        if i == j {
            continue;
        }
        let mut s = sequence.clone();
        swap(&mut s, i, j);

        let pressure = determine_pressure(&s, &nodes);
        if (pressure < old_pressure && rng.gen::<f64>() < 0.1) || pressure >= old_pressure {
            sequence = s.clone();
            old_pressure = pressure;
        }
        if pressure > max_pressure {
            max_pressure = pressure;
            println!("new max {} {} {:?}", _a, max_pressure, sequence);
        }
    }
    max_pressure
}

fn swap(s: &mut [String], i: usize, j: usize) {
    let tmp = s[i].clone();
    s[i] = s[j].clone();
    s[j] = tmp;
}

fn determine_pressure(sequence: &[String], nodes: &HashMap<String, EffectiveNode>) -> usize {
    let mut position = "AA".to_string();
    let mut pressure = 0;
    let mut minute = 0;
    'outer: for l in sequence {
        for (target, weight) in nodes[&position].edges.iter() {
            if target == l {
                minute += weight;
                if minute > MAX_TIME {
                    break 'outer;
                }
                position = target.clone();
                pressure += (MAX_TIME - minute) * nodes[target].rate;
                // println!("{}{}{}", minute, position, pressure);
                continue 'outer;
            }
        }
        panic!("Should never be reached");
    }
    pressure
}

///////////////////////////////////////////

// fn determine_most_releasable_pressure(nodes: HashMap<String, EffectiveNode>) -> usize {
//     let path = ActionSequence{ label: "AA".to_string(), minute: 0, pressure: 0, history: vec!["AA".to_string()], visited: HashSet::new() };
//     let mut queue = BinaryHeap::new();
//     queue.push(path);
//     let mut expanded = HashSet::new();
//     let mut max_pressure = 0;
//     while let Some(current) = queue.pop() {
//         if expanded.contains(&current.history.join("")) {
//             continue;
//         }
//         expanded.insert(current.history.join(""));
//         // println!("{:?} {} {}", current.history, current.minute, current.pressure);
//         for (edge, weight) in nodes[&current.label].edges.iter() {
//             if current.visited.contains(edge) {
//                 continue;
//             }
//             let mut opened = current.visited.clone();
//             opened.insert(edge.clone());
//             let minute = current.minute + weight;
//             if minute > 30 {
//                 continue;
//             }
//             let pressure = current.pressure + (30 - minute) * nodes[edge].rate;
//             let mut history = current.history.clone();
//             history.push(edge.clone());
//             if pressure > max_pressure {
//                 max_pressure = pressure;
//                 println!("new max {} {:?}", max_pressure, history);
//             }
//             queue.push(ActionSequence { label: edge.clone(), minute, pressure, history, visited: opened });
//         }
//     }
//     max_pressure
// }

// #[derive(Clone, Debug)]
// struct ActionSequence {
//     label: String,
//     minute: usize,
//     pressure: usize,
//     history: Vec<String>,
//     visited: HashSet<String>,
// }

// impl Eq for ActionSequence {}

// impl PartialEq for ActionSequence {
//     fn eq(&self, other: &Self) -> bool {
//         // self.pressure.eq(&other.pressure)
//         other.minute.eq(&self.minute)
//     }
// }

// impl Ord for ActionSequence {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // self.pressure.cmp(&other.pressure)
//         other.minute.cmp(&self.minute)
//     }
// }

// impl PartialOrd for ActionSequence {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // Some(self.pressure.cmp(&other.pressure))
//         Some(other.minute.cmp(&self.minute))
//     }
// }

///////////////////////////////////////////

// #[derive(Clone, Debug)]
// struct Path {
//     score: usize,
//     history: [Vec<String>; 2],
// }

// impl Path {
//     fn hashable_history(&self) -> String {
//         if self.history[1].len() > 0 {
//             assert_eq!(self.history[0].len(), self.history[1].len());
//         }
//         self.history[0].join("") + &self.history[1].join("")
//     }
// }

// impl Eq for Path {}

// impl PartialEq for Path {
//     fn eq(&self, other: &Self) -> bool {
//         self.score.eq(&other.score)
//     }
// }

// impl Ord for Path {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.score.cmp(&other.score)
//     }
// }

// impl PartialOrd for Path {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.score.cmp(&other.score))
//     }
// }

// fn determine_most_releasable_pressure(nodes: HashMap<String, Node>) -> usize {
//     let path;
//     if !WITH_ELEPHANT {
//         path = Path{ score: 0, history: [vec!["AA".to_string()], vec![]] };
//     } else {
//         path = Path{ score: 0, history: [vec!["AA".to_string()], vec!["AA".to_string()]] };
//     }
//     let mut queue = BinaryHeap::new();
//     queue.push(path);
//     let mut explored = HashSet::new();
//     let mut max_score = 0;
//     while let Some(path) = queue.pop() {
//         if path.history[0].len() == MAX_TIME {
//             continue;
//         }

//         if !WITH_ELEPHANT {
//             let current_node = &nodes[path.history[0].last().unwrap()];
//             for edge in current_node.edges.iter() {
//                 let mut history = path.history.clone();
//                 history[0].push(edge.to_string());
//                 let score = determine_score(&nodes, &history);
//                 let heuristic = compute_heuristic(&nodes, &history);
//                 queue.push(Path{ score: heuristic, history: history.clone() });
//                 if score > max_score {
//                     max_score = score;
//                     println!("{:?} {} {}", history, history.len(), max_score);
//                 }
//             }
//         } else {
//             let current_node = &nodes[path.history[0].last().unwrap()];
//             let current_node_elephant = &nodes[path.history[1].last().unwrap()];
//             for edge in current_node.edges.iter() {
//                 for edge_elephant in current_node_elephant.edges.iter() {
//                     let mut history = path.history.clone();
//                     history[0].push(edge.to_string());
//                     history[1].push(edge_elephant.to_string());
//                     let score = determine_score(&nodes, &history);
//                     let heuristic = compute_heuristic(&nodes, &history);
//                     let p = Path{ score: heuristic, history: history.clone() };
//                     let hash = p.hashable_history();
//                     if explored.contains(&hash) {
//                         continue;
//                     }
//                     explored.insert(hash);
//                     queue.push(p);
//                     if score > max_score {
//                         max_score = score;
//                         println!("{:?} {} {}", history, history[0].len(), max_score);
//                     }
//                 }
//             }
//         }
//     }
//     max_score
// }

// fn determine_score(nodes: &HashMap<String, Node>, history: &[Vec<String>]) -> usize {
//     assert!(history[0].len() <= MAX_TIME);
//     let mut visited = HashSet::new();
//     let mut score = 0;
//     let mut baseline = 0;
//     for minute in 0..MAX_TIME {
//         for i in 0..2 {
//             if minute < history[i].len() {
//                 let label = &history[i][minute];
//                 if visited.contains(label) {
//                     continue;
//                 }
//                 visited.insert(label);
//                 baseline += nodes[label].rate;
//             }
//         }
//         score += baseline;
//     }
//     score
// }

// fn compute_heuristic(nodes: &HashMap<String, Node>, history: &[Vec<String>]) -> usize {
//     // check which valves are unopened and assume valves with most flow could be
//     // openend one after the other; overestimates the achievable score, i.e., it
//     // is an admissible heuristic
//     let mut visited = HashSet::new();
//     for i in 0..2 {
//         for label in history[i].iter() {
//             visited.insert(label.clone());
//         }
//     }

//     let mut remaining = Vec::new();
//     for label in nodes.keys() {
//         if !visited.contains(label) && label.chars().last().unwrap() == 'o' {
//             remaining.push((label.to_string(), nodes[label].rate));
//         }
//     }
//     remaining.sort_unstable_by_key(|(_l, r)| *r);
//     let mut remaining = remaining.into_iter().map(|(l, _r)| l).collect::<Vec<String>>();

//     let heuristic;
//     if history[1].len() == 0 {
//         let mut h = history.to_owned();
//         while let Some(n) = remaining.pop() {
//             if h[0].len() < MAX_TIME {
//                 h[0].push(n.clone());
//                 if h[0].len() < MAX_TIME {
//                     h[0].push(n.clone());
//                 }
//             } else {
//                 break;
//             }
//         }
//         heuristic = determine_score(nodes, &h);
//     } else {
//         let mut h = history.to_owned();
//         'outer: loop {
//             for i in 0..2 {
//                 if h[i].len() == MAX_TIME {
//                     break 'outer;
//                 }
//                 let n = remaining.pop();
//                 if n.is_none() {
//                     break 'outer;
//                 }
//                 let n = n.unwrap();
//                 h[i].push(n.clone());
//                 if h[i].len() < MAX_TIME {
//                     h[i].push(n);
//                 }
//             }
//         }
//         heuristic = determine_score(nodes, &h);
//     }
//     heuristic
// }
