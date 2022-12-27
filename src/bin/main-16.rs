use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

use rand::distributions::Distribution;
use rand::{distributions::Uniform, Rng, SeedableRng};
use regex::Regex;

static WITH_ELEPHANT: bool = true;
static MAX_TIME: usize = 26;
// static WITH_ELEPHANT: bool = false;
// static MAX_TIME: usize = 30;

fn main() {
    let filename = "inputs/16.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let nodes = parse_graph(content);
    let nodes = determine_effective_graph(&nodes);
    if !WITH_ELEPHANT {
        let most_releasable_pressure = determine_most_releasable_pressure(nodes);
        println!(
            "The most pressure possible to release is {}.",
            most_releasable_pressure
        );
    } else {
        let most_releasable_pressure = determine_most_releasable_pressure_with_elephant(nodes);
        println!(
            "The most pressure possible to release is {}.",
            most_releasable_pressure
        );
    }
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

fn determine_most_releasable_pressure(nodes: HashMap<String, EffectiveNode>) -> usize {
    let mut sequence = nodes
        .iter()
        .map(|(l, _n)| l.to_string())
        .filter(|l| l.chars().last().unwrap() == 'o')
        .collect::<Vec<String>>();
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

fn determine_most_releasable_pressure_with_elephant(
    nodes: HashMap<String, EffectiveNode>,
) -> usize {
    let sequence = nodes
        .iter()
        .map(|(l, _n)| l.to_string())
        .filter(|l| l.chars().last().unwrap() == 'o')
        .collect::<Vec<String>>();
    let mut sequence0 = sequence[..sequence.len() / 2].to_vec();
    sequence0.sort_unstable();
    let mut sequence1 = sequence[sequence.len() / 2..].to_vec();
    sequence1.sort_unstable();
    let mut old_pressure = 0;
    let mut max_pressure = 0;
    let between = Uniform::from(0..sequence0.len());
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
    for _a in 0..50_000_000 {
        let i = between.sample(&mut rng);
        let j = between.sample(&mut rng);
        if i == j {
            continue;
        }

        let mut s0 = sequence0.clone();
        let mut s1 = sequence1.clone();
        let p = rng.gen::<f64>();
        if p < 0.333 {
            swap(&mut s0, i, j);
        } else if p < 0.666 {
            swap(&mut s1, i, j);
        } else {
            swap_between(&mut s0, &mut s1, i, j);
        }

        let pressure = determine_pressure(&s0, &nodes) + determine_pressure(&s1, &nodes);
        if (pressure < old_pressure && rng.gen::<f64>() < 0.1) || pressure >= old_pressure {
            sequence0 = s0.clone();
            sequence1 = s1.clone();
            old_pressure = pressure;
        }
        if pressure > max_pressure {
            max_pressure = pressure;
            println!("new max {} {} {:?}", _a, max_pressure, sequence);
        }
    }
    max_pressure
}

fn swap_between(s0: &mut [String], s1: &mut [String], i: usize, j: usize) {
    let tmp = s0[i].clone();
    s0[i] = s1[j].clone();
    s1[j] = tmp;
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
                continue 'outer;
            }
        }
        panic!("Should never be reached");
    }
    pressure
}
