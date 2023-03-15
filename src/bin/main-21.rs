use std::{
    collections::HashMap,
    fs,
    ops::{Add, Div, Mul, Sub},
};

use regex::Regex;

fn main() {
    let filename = "inputs/21.txt";
    let content = fs::read_to_string(filename).unwrap();
    let mut nodes = parse_nodes(&content);
    let v = nodes["root"].eval(&nodes).value.round() as i64;
    println!("The monkey name `root` will yell {:?}.", v);

    let mut humn = match &nodes["humn"] {
        Node::Leaf { value } => value.value,
        _ => panic!(),
    };
    let (fst, snd) = match &nodes["root"] {
        Node::Op { op: _, fst, snd } => (fst.clone(), snd.clone()),
        _ => panic!(),
    };

    let lr = 0.0001;
    loop {
        nodes.insert(
            "humn".to_string(),
            Node::Leaf {
                value: Dual {
                    value: humn,
                    dual: 1.0,
                },
            },
        );
        let v_fst = nodes[&fst].eval(&nodes);
        let v_snd = nodes[&snd].eval(&nodes);
        let error = (v_fst - v_snd) * (v_fst - v_snd);
        let grad = error.dual;
        humn -= lr * grad;
        if (v_fst - v_snd).value.abs() < 1e-1 {
            break;
        }
    }
    let humn = humn.round() as i64;
    println!("You need to yell {} to pass `root`'s equality test.", humn);
}

fn parse_nodes(content: &str) -> HashMap<String, Node> {
    let re_op = Regex::new(r"([a-z]+): ([a-z]+) ([+\-*/]{1}) ([a-z]+)").unwrap();
    let re_leaf = Regex::new(r"([a-z]+): ([0-9]+)").unwrap();
    let mut nodes = HashMap::new();
    for l in content.lines() {
        if let Some(caps) = re_op.captures(l) {
            let name = caps[1].to_string();
            let fst = caps[2].to_string();
            let op = Operator::from(&caps[3]);
            let snd = caps[4].to_string();
            let node = Node::new_op(op, fst, snd);
            nodes.insert(name, node);
        } else if let Some(caps) = re_leaf.captures(l) {
            let name = caps[1].to_string();
            let value = Dual::from(caps[2].parse::<f64>().unwrap());
            let node = Node::new_leaf(value);
            nodes.insert(name, node);
        } else {
            panic!("couldn't parse line: {:?}", l);
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
        value: Dual,
    },
}

impl Node {
    fn new_op(op: Operator, fst: String, snd: String) -> Self {
        Node::Op { op, fst, snd }
    }

    fn new_leaf(value: Dual) -> Self {
        Node::Leaf { value }
    }

    fn eval(&self, nodes: &HashMap<String, Node>) -> Dual {
        match self {
            Node::Op { op, fst, snd } => {
                let value_fst = nodes[fst].eval(nodes);
                let value_snd = nodes[snd].eval(nodes);
                match op {
                    Operator::Add => value_fst + value_snd,
                    Operator::Sub => value_fst - value_snd,
                    Operator::Mul => value_fst * value_snd,
                    Operator::Div => value_fst / value_snd,
                }
            }
            Node::Leaf { value } => *value,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operator {
    fn from(operator: &str) -> Self {
        match operator {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => panic!("unknown operator"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Dual {
    value: f64,
    dual: f64,
}

impl From<f64> for Dual {
    fn from(value: f64) -> Self {
        Dual { value, dual: 0.0 }
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            dual: self.dual + other.dual,
        }
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            dual: self.dual - other.dual,
        }
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
            dual: self.dual * other.value + self.value * other.dual,
        }
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
            dual: (self.dual * other.value - self.value * other.dual) / other.value.powi(2),
        }
    }
}
