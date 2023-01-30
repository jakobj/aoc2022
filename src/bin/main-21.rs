use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let filename = "inputs/21b.txt";
    let content = fs::read_to_string(filename).unwrap();
    let nodes = parse_nodes(&content);
    println!("{:?}", nodes["root"].eval(&nodes));
}

fn parse_nodes(content: &str) -> HashMap<String, Node> {
    let re_op = Regex::new(r"([a-z]+): ([a-z]+) ([+\-*/]{1}) ([a-z]+)").unwrap();
    let re_leaf = Regex::new(r"([a-z]+): ([0-9]+)").unwrap();
    let mut nodes = HashMap::new();
    for l in content.lines() {
        let caps = re_op.captures(l);
        if let Some(caps) = caps {
            let name = caps[1].to_string();
            let node = Node::new_op(&caps[3], &caps[2], &caps[4]);
            nodes.insert(name, node);
            continue;
        }

        let caps = re_leaf.captures(l);
        if let Some(caps) = caps {
            let name = caps[1].to_string();
            let node = Node::new_leaf(caps[2].parse().unwrap());
            nodes.insert(name, node);
        }
    }
    nodes
}

#[derive(Debug)]
enum Node {
    Op {
        op: Operator,
        fst: String,
        snd: String,
    },
    Leaf {
        value: i64,
    },
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Node {
    fn new_op(operator: &str, fst: &str, snd: &str) -> Self {
        let op = match operator {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => panic!("unknown operator"),
        };
        Node::Op {
            op,
            fst: fst.to_string(),
            snd: snd.to_string(),
        }
    }

    fn new_leaf(value: i64) -> Self {
        Node::Leaf { value }
    }

    fn eval(&self, nodes: &HashMap<String, Node>) -> i64 {
        match self {
            Node::Op { op, fst, snd } => match op {
                Operator::Add => nodes[fst].eval(nodes) + nodes[snd].eval(nodes),
                Operator::Sub => nodes[fst].eval(nodes) - nodes[snd].eval(nodes),
                Operator::Mul => nodes[fst].eval(nodes) * nodes[snd].eval(nodes),
                Operator::Div => nodes[fst].eval(nodes) / nodes[snd].eval(nodes),
            },
            Node::Leaf { value } => *value,
        }
    }
}
