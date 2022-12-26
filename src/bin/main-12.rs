use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/12e.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let map = parse_map(&content);
    let path_length = compute_path_length(&map, 'S', 'E', false);
    println!("The shortest path has {} steps.\n", path_length);

    let path_length = compute_path_length(&map, 'E', 'a', true);
    println!("The shortest scenic cardio path has {} steps.", path_length);
}

fn parse_map(content: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for (row, l) in content.lines().enumerate() {
        map.push(Vec::new());
        for c in l.chars() {
            map[row].push(c);
        }
    }
    map
}

#[derive(Debug)]
struct Position {
    position: (usize, usize),
    cost: usize,
    history: Vec<(usize, usize)>,
}

impl Position {
    fn new(position: (usize, usize), cost: usize) -> Self {
        Self {
            position,
            cost,
            history: Vec::new(),
        }
    }
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn compute_path_length(
    map: &Vec<Vec<char>>,
    start_marker: char,
    end_marker: char,
    invert: bool,
) -> usize {
    // variant of Dijkstra
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start_position = find_position(map, start_marker);
    let mut e = Position::new(start_position, 0);
    e.history.push(e.position);
    queue.push(e);
    while let Some(current) = queue.pop() {
        if get_char(map, &current.position) == end_marker {
            print_map(map, &visited, &current.history);
            return current.cost;
        }
        let neighbors = compute_neighbors(map, &current.position, invert);
        for n in neighbors {
            if !visited.contains(&n) {
                let mut e = Position::new(n, current.cost + 1);
                e.history = current.history.clone();
                e.history.push(e.position);
                visited.insert(e.position);
                queue.push(e);
            }
        }
    }
    panic!("Couldn't find the destination");
}

fn find_position(map: &Vec<Vec<char>>, marker: char) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == marker {
                return (row, col);
            }
        }
    }
    panic!("Couldn't determine position");
}

fn get_char(map: &Vec<Vec<char>>, position: &(usize, usize)) -> char {
    map[position.0][position.1]
}

fn compute_neighbors(
    map: &Vec<Vec<char>>,
    position: &(usize, usize),
    invert: bool,
) -> Vec<(usize, usize)> {
    let map_size = (map.len(), map[0].len());
    let mut neighbors = Vec::new();
    if position.0 > 0 {
        neighbors.push((position.0 - 1, position.1));
    }
    if position.0 < map_size.0 - 1 {
        neighbors.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        neighbors.push((position.0, position.1 - 1));
    }
    if position.1 < map_size.1 - 1 {
        neighbors.push((position.0, position.1 + 1));
    }
    let neighbors = neighbors
        .into_iter()
        .filter(|n| is_reachable(get_char(map, n), get_char(map, &position), invert))
        .collect();
    neighbors
}

fn is_reachable(target: char, source: char, invert: bool) -> bool {
    let target = match target {
        'S' => 'a',
        'E' => 'z',
        _ => target,
    };
    let source = match source {
        'S' => 'a',
        'E' => 'z',
        _ => source,
    };

    if !invert {
        if (target as u32) <= (source as u32 + 1) {
            return true;
        } else {
            return false;
        }
    } else {
        if (target as u32) >= (source as u32 - 1) {
            return true;
        } else {
            return false;
        }
    }
}

fn print_map(
    map: &Vec<Vec<char>>,
    visted: &HashSet<(usize, usize)>,
    history: &Vec<(usize, usize)>,
) {
    let history = history
        .clone()
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let pos = (i, j);
            let text;
            if history.contains(&pos) {
                text = format!("\x1b[31;1m{}\x1b[37;0m", map[i][j]);
            } else if visted.contains(&pos) {
                text = format!("\x1b[34m{}\x1b[37m", map[i][j]);
            } else {
                text = format!("{}", map[i][j]);
            }
            print!("{}", text);
        }
        println!("");
    }
}
