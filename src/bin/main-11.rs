use regex::Regex;
use std::{fs, ops::DivAssign};

#[derive(Debug)]
struct Monkey {
    queue: Vec<usize>,
    operator: char,
    factor: Option<usize>, // None indicates that the previous value is the factor, e.g., old * old
    divisor: usize,
    targets: [usize; 2],
    n_inspections: usize,
}

impl Monkey {
    pub fn new(
        starting_values: Vec<usize>,
        operator: char,
        factor: Option<usize>,
        divisor: usize,
        targets: [usize; 2],
    ) -> Self {
        Self {
            queue: starting_values,
            operator,
            factor,
            divisor,
            targets,
            n_inspections: 0,
        }
    }

    pub fn resume(&mut self, reducer: &dyn Fn(usize) -> usize) -> Vec<(usize, usize)> {
        let mut items_to_send = Vec::new();
        for value in self.queue.drain(..) {
            self.n_inspections += 1;

            let factor;
            if let Some(f) = self.factor {
                factor = f;
            } else {
                factor = value;
            }

            let worry_level;
            match self.operator {
                // '+' => worry_level = (item.value + factor) / 3,
                // '*' => worry_level = item.value * factor / 3,
                '+' => worry_level = value + factor,
                '*' => worry_level = value * factor,
                _ => panic!("Unexpected operator {}", self.operator),
            }
            // keep worry level under control: substract a value divisible by
            // all divisors
            // worry_level %= 13 * 17 * 19 * 23;
            let worry_level = reducer(worry_level);

            let target;
            if worry_level % self.divisor == 0 {
                target = self.targets[0];
            } else {
                target = self.targets[1];
            }

            items_to_send.push((target, worry_level));
        }
        items_to_send
    }

    pub fn send(&mut self, item: usize) {
        self.queue.push(item);
    }
}

fn main() {
    let filename = "inputs/11.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");

    let mut monkeys = parse_monkeys(&content);

    // let reducer = |v| v / 3; // part1
    let m = monkeys.iter().map(|m| m.divisor).product::<usize>();
    let reducer = move |v| v % m; // part2
    for _ in 0..10_000 {
        run_round(&mut monkeys, &reducer);
    }

    let mut n_inspections_per_monkey = monkeys
        .iter()
        .map(|m| m.n_inspections)
        .collect::<Vec<usize>>();
    n_inspections_per_monkey.sort_unstable();
    n_inspections_per_monkey.reverse();
    println!(
        "The level of monkey business is {} after 20 rounds.",
        n_inspections_per_monkey[0] * n_inspections_per_monkey[1]
    );
}

fn parse_monkeys(content: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = content.lines();
    loop {
        let _monkey_id = lines.next().unwrap();
        let starting_values = parse_starting_items(lines.next().unwrap());
        let (operator, factor) = parse_operator_and_factor(lines.next().unwrap());
        let divisor = parse_divisor(lines.next().unwrap());
        let target0 = parse_target(lines.next().unwrap());
        let target1 = parse_target(lines.next().unwrap());

        monkeys.push(Monkey::new(
            starting_values,
            operator,
            factor,
            divisor,
            [target0, target1],
        ));

        // if we can't skip the next blank line, we're done
        if let None = lines.next() {
            break;
        }
    }
    monkeys
}

fn parse_starting_items(l: &str) -> Vec<usize> {
    let re = Regex::new(r"Starting items: ([0-9, ]+)").unwrap();
    let caps = re.captures(l);
    if caps.is_none() {
        return Vec::new();
    }
    let mut items = caps.unwrap().get(1).unwrap().as_str().to_string();
    items.retain(|c| c != ',');
    items
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_operator_and_factor(l: &str) -> (char, Option<usize>) {
    let re = Regex::new(r"Operation: new = old ([+*]) ([0-9]+|old)").unwrap();
    let caps = re.captures(l).unwrap();
    let op = caps.get(1).unwrap().as_str().chars().next().unwrap();
    if let Ok(multiplier) = caps.get(2).unwrap().as_str().parse() {
        (op, Some(multiplier))
    } else {
        (op, None)
    }
}

fn parse_divisor(l: &str) -> usize {
    let re = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();
    re.captures(l)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn parse_target(l: &str) -> usize {
    let re = Regex::new(r"throw to monkey ([0-9]+)").unwrap();
    re.captures(l)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn run_round(monkeys: &mut Vec<Monkey>, reducer: &dyn Fn(usize) -> usize) {
    for i in 0..monkeys.len() {
        let m = &mut monkeys[i];
        let items_to_send = m.resume(reducer);
        for item in items_to_send {
            let (target, item) = item;
            monkeys[target].send(item);
        }
    }
}
