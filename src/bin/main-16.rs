use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

use regex::Regex;

fn main() {
    let filename = "inputs/16.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let nodes = parse_graph(content);
    let nodes = determine_effective_graph(&nodes);

    let sets = determine_pressure_for_subsets(&nodes, 30);
    let max_pressure = sets.iter().map(|(_k, p)| p).max().unwrap();
    println!(
        "The most pressure possible to release alone is {}.",
        max_pressure,
    );

    let sets = determine_pressure_for_subsets(&nodes, 26);
    let mut max_pressure = 0;
    for (s0, p0) in sets.iter() {
        for (s1, p1) in sets.iter() {
            if disjoint(s0, s1) {
                if p0 + p1 > max_pressure {
                    max_pressure = p0 + p1;
                }
            }
        }
    }
    println!(
        "The most pressure possible to release with an elephant is {}.",
        max_pressure,
    );
}

fn disjoint(s0: &str, s1: &str) -> bool {
    for l0 in s0.split(",") {
        for l1 in s1.split(",") {
            if l0 == l1 {
                return false;
            }
        }
    }
    true
}

fn parse_graph(content: String) -> HashMap<String, Node> {
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=([0-9]{1,2}); tunnel[s]? lead[s]? to valve[s]? ([A-Z ,]+)",
    )
    .unwrap();
    let mut nodes = HashMap::new();
    for l in content.lines() {
        let caps = re.captures(l).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let mut edges = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        if rate > 0 {
            // if a valve has a non-zero rate when it's opened, add it as an
            // additional node in the graph reachable only from the respective
            // closed-valve node
            let node_open = Node {
                rate,
                edges: edges.clone(),
            };
            let label_open = label.clone() + "o";
            nodes.insert(label_open.clone(), node_open);
            edges.push(label_open);
            let node_closed = Node { rate: 0, edges };
            nodes.insert(label, node_closed);
        } else {
            nodes.insert(label, Node { rate: 0, edges });
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

            if target.chars().last().unwrap() != 'o' {
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
        effective_nodes.insert(
            source.clone(),
            EffectiveNode {
                rate: nodes[source].rate,
                edges,
            },
        );
    }
    effective_nodes
}

#[derive(Clone, Debug)]
struct EffectiveNode {
    rate: usize,
    edges: Vec<(String, usize)>,
}

fn determine_length_of_shortest_path(
    source: &str,
    target: &str,
    nodes: &HashMap<String, Node>,
) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Path {
        label: source.to_string(),
        length: 0,
    });
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
            queue.push(Path {
                label: edge.clone(),
                length: current.length + 1,
            });
        }
    }
    panic!("No path found ({} -> {})", source, target);
}

struct Path {
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

fn determine_pressure_for_subsets(
    nodes: &HashMap<String, EffectiveNode>,
    max_time: usize,
) -> HashMap<String, usize> {
    // BFS
    let mut queue = BinaryHeap::new();
    queue.push(State {
        label: "AA".to_string(),
        minute: 0,
        pressure: 0,
        visited: HashSet::new(),
    });
    let mut sets: HashMap<String, usize> = HashMap::new();
    while let Some(current) = queue.pop() {
        for (target, weight) in nodes[&current.label].edges.iter() {
            if current.visited.contains(target) {
                // don't open any valve twice
                continue;
            }
            let minute = current.minute + weight;
            if minute > max_time {
                // stay below the time limit
                continue;
            }
            let pressure = current.pressure + (max_time - minute) * nodes[target].rate;
            let mut visited = current.visited.clone();
            visited.insert(target.to_string());
            let mut key = visited.clone().into_iter().collect::<Vec<String>>();
            key.sort_unstable();
            let key = key.join(",");
            if !sets.contains_key(&key) || sets[&key] < pressure {
                sets.insert(key, pressure);
            }
            queue.push(State {
                label: target.to_string(),
                minute,
                pressure,
                visited,
            });
        }
    }
    sets
}

struct State {
    label: String,
    minute: usize,
    pressure: usize,
    visited: HashSet<String>,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.visited.len().eq(&other.visited.len())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.visited.len().cmp(&self.visited.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.visited.len().cmp(&self.visited.len()))
    }
}
