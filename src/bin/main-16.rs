use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs};

use regex::Regex;

// 2071 -- 3056

// static WITH_ELEPHANT: bool = true;
// static MAX_TIME: usize = 26;
static WITH_ELEPHANT: bool = false;
static MAX_TIME: usize = 30;

// AA II JJ JJo II AA BB BBo CC CCo
// AA DD DDo EE FF GG HH HHo GG FF EE EEo

fn main() {
    let filename = "inputs/16.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let nodes = parse_graph(content);
    // println!("{}", determine_score(
    //     &nodes,
    //     &[vec!["AA".to_string(), "II".to_string(), "JJ".to_string(), "JJo".to_string(), "II".to_string(), "AA".to_string(), "BB".to_string(), "BBo".to_string(), "CC".to_string(), "CCo".to_string()],
    //       vec!["AA".to_string(), "DD".to_string(), "DDo".to_string(), "EE".to_string(), "FF".to_string(), "GG".to_string(), "HH".to_string(), "HHo".to_string(), "GG".to_string(), "FF".to_string(), "EE".to_string(), "EEo".to_string()]]));
    // println!("{}", compute_heuristic(&nodes, &[vec![], vec![]]));
    let most_releasable_pressure = determine_most_releasable_pressure(nodes);
    println!("The most pressure possible to release is {}.", most_releasable_pressure);
}

fn parse_graph(content: String) -> HashMap::<String, Node> {
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

#[derive(Clone, Debug)]
struct Path {
    score: usize,
    history: [Vec<String>; 2],
}

impl Path {
    fn hashable_history(&self) -> String {
        if self.history[1].len() > 0 {
            assert_eq!(self.history[0].len(), self.history[1].len());
        }
        self.history[0].join("") + &self.history[1].join("")
    }
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

fn determine_most_releasable_pressure(nodes: HashMap<String, Node>) -> usize {
    let path;
    if !WITH_ELEPHANT {
        path = Path{ score: 0, history: [vec!["AA".to_string()], vec![]] };
    } else {
        path = Path{ score: 0, history: [vec!["AA".to_string()], vec!["AA".to_string()]] };
    }
    let mut queue = BinaryHeap::new();
    queue.push(path);
    let mut explored = HashSet::new();
    let mut max_score = 0;
    while let Some(path) = queue.pop() {
        if path.history[0].len() == MAX_TIME {
            continue;
        }

        if !WITH_ELEPHANT {
            let current_node = &nodes[path.history[0].last().unwrap()];
            for edge in current_node.edges.iter() {
                let mut history = path.history.clone();
                history[0].push(edge.to_string());
                let score = determine_score(&nodes, &history);
                let heuristic = compute_heuristic(&nodes, &history);
                queue.push(Path{ score: heuristic, history: history.clone() });
                if score > max_score {
                    max_score = score;
                    println!("{:?} {} {}", history, history.len(), max_score);
                }
            }
        } else {
            let current_node = &nodes[path.history[0].last().unwrap()];
            let current_node_elephant = &nodes[path.history[1].last().unwrap()];
            for edge in current_node.edges.iter() {
                for edge_elephant in current_node_elephant.edges.iter() {
                    let mut history = path.history.clone();
                    history[0].push(edge.to_string());
                    history[1].push(edge_elephant.to_string());
                    let score = determine_score(&nodes, &history);
                    let heuristic = compute_heuristic(&nodes, &history);
                    let p = Path{ score: heuristic, history: history.clone() };
                    let hash = p.hashable_history();
                    if explored.contains(&hash) {
                        continue;
                    }
                    explored.insert(hash);
                    queue.push(p);
                    if score > max_score {
                        max_score = score;
                        println!("{:?} {} {}", history, history[0].len(), max_score);
                    }
                }
            }
        }
    }
    max_score
}

fn determine_score(nodes: &HashMap<String, Node>, history: &[Vec<String>]) -> usize {
    assert!(history[0].len() <= MAX_TIME);
    let mut visited = HashSet::new();
    let mut score = 0;
    let mut baseline = 0;
    for minute in 0..MAX_TIME {
        for i in 0..2 {
            if minute < history[i].len() {
                let label = &history[i][minute];
                if visited.contains(label) {
                    continue;
                }
                visited.insert(label);
                baseline += nodes[label].rate;
            }
        }
        score += baseline;
    }
    score
}

fn compute_heuristic(nodes: &HashMap<String, Node>, history: &[Vec<String>]) -> usize {
    // check which valves are unopened and assume valves with most flow could be
    // openend one after the other; overestimates the achievable score, i.e., it
    // is an admissible heuristic
    let mut visited = HashSet::new();
    for i in 0..2 {
        for label in history[i].iter() {
            visited.insert(label.clone());
        }
    }

    let mut remaining = Vec::new();
    for label in nodes.keys() {
        if !visited.contains(label) && label.chars().last().unwrap() == 'o' {
            remaining.push((label.to_string(), nodes[label].rate));
        }
    }
    remaining.sort_unstable_by_key(|(_l, r)| *r);
    let mut remaining = remaining.into_iter().map(|(l, _r)| l).collect::<Vec<String>>();

    let heuristic;
    if history[1].len() == 0 {
        let mut h = history.to_owned();
        while let Some(n) = remaining.pop() {
            if h[0].len() < MAX_TIME {
                h[0].push(n.clone());
                if h[0].len() < MAX_TIME {
                    h[0].push(n.clone());
                }
            } else {
                break;
            }
        }
        heuristic = determine_score(nodes, &h);
    } else {
        let mut h = history.to_owned();
        'outer: loop {
            for i in 0..2 {
                if h[i].len() == MAX_TIME {
                    break 'outer;
                }
                let n = remaining.pop();
                if n.is_none() {
                    break 'outer;
                }
                let n = n.unwrap();
                h[i].push(n.clone());
                if h[i].len() < MAX_TIME {
                    h[i].push(n);
                }
            }
        }
        heuristic = determine_score(nodes, &h);
    }
    heuristic
}
