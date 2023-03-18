use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/24.txt";
    let content = fs::read_to_string(filename).unwrap();
    let initial_position = determine_position(&content, 'E');
    let final_position = determine_position(&content, 'Z');
    let blizzards = determine_blizzards(&content);
    let layout = determine_layout(&content);

    let mut total = 0;
    let (final_state, blizzards) = find_path(initial_position, final_position, &blizzards, layout);
    println!(
        "You and the elves can reach the goal in {} minutes.",
        final_state.minute
    );
    total += final_state.minute;

    let (final_state, blizzards) = find_path(final_position, initial_position, &blizzards, layout);
    println!("The way back takes {} minutes.", final_state.minute);
    total += final_state.minute;

    let (final_state, _) = find_path(initial_position, final_position, &blizzards, layout);
    println!(
        "The trip back to the goal again takes {} minutes.",
        final_state.minute
    );
    total += final_state.minute;

    println!("The total time is {} minutes.", total);
}

fn determine_position(content: &str, marker: char) -> Position {
    for (y, l) in content.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == marker {
                return Position {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }
    panic!("could not determine position of marker: '{}'", marker);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn next(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::None => *self,
        }
    }
}

fn determine_blizzards(content: &str) -> Vec<Blizzard> {
    let markers = ['^', '>', 'v', '<'];
    let mut blizzards = Vec::new();
    for (y, l) in content.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if markers.iter().any(|&m| m == c) {
                blizzards.push(Blizzard {
                    position: Position {
                        x: x as i64,
                        y: y as i64,
                    },
                    direction: Direction::from(c),
                });
            }
        }
    }
    blizzards
}

#[derive(Clone, Copy, Debug)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("unknown Direction: '{}'", c),
        }
    }
}

impl From<Direction> for char {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::None => '0',
        }
    }
}

fn determine_layout(content: &str) -> Layout {
    let height = content.lines().count();
    let width = content.lines().next().unwrap().chars().count();
    Layout { height, width }
}

#[derive(Clone, Copy, Debug)]
struct Layout {
    height: usize,
    width: usize,
}

impl Layout {
    fn is_valid_position(&self, position: Position) -> bool {
        // handle initial and final position separately
        if (position.x == 1 && position.y == 0)
            || (position.x == self.width as i64 - 2 && position.y == self.height as i64 - 1)
        {
            return true;
        }
        if position.y < 1 || position.y >= self.height as i64 - 1 {
            return false;
        }
        if position.x < 1 || position.x >= self.width as i64 - 1 {
            return false;
        }
        true
    }

    fn wrap_around(&self, position: Position, direction: Direction) -> Position {
        let max_y = self.height as i64 - 2;
        let max_x = self.width as i64 - 2;
        match direction {
            Direction::Up => {
                if position.y == 1 {
                    return Position {
                        x: position.x,
                        y: max_y,
                    };
                }
            }
            Direction::Right => {
                if position.x == max_x {
                    return Position {
                        x: 1,
                        y: position.y,
                    };
                }
            }
            Direction::Down => {
                if position.y == max_y {
                    return Position {
                        x: position.x,
                        y: 1,
                    };
                }
            }
            Direction::Left => {
                if position.x == 1 {
                    return Position {
                        x: max_x,
                        y: position.y,
                    };
                }
            }
            Direction::None => panic!("couldn't wrap: {:?} {:?}", position, direction),
        };
        panic!("couldn't wrap: {:?} {:?}", position, direction);
    }
}

fn find_path(
    initial_position: Position,
    final_position: Position,
    blizzards: &Vec<Blizzard>,
    layout: Layout,
) -> (State, Vec<Blizzard>) {
    // BFS
    let mut queue = BinaryHeap::new();
    queue.push(State {
        position: initial_position,
        minute: 0,
    });

    // store blizzards and positions to reuse over states with identical number
    // of minutes passed
    let mut blizzards_by_step = Vec::new();
    blizzards_by_step.push(blizzards.clone());
    let mut blizzard_positions_by_step = Vec::new();
    blizzard_positions_by_step.push(blizzards_to_positions(&blizzards));

    // avoid revisiting identical configurations
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        let next_minute = state.minute + 1;

        // since blizzard configuration only depends on minute, we just use that
        // instead of location of all blizzards for hashing
        let s = format!("{:?} {}", state.position, next_minute);
        if visited.contains(&s) {
            continue;
        }
        visited.insert(s);

        if next_minute >= blizzard_positions_by_step.len() {
            let next_blizzards = update_blizzards(&blizzards_by_step.last().unwrap(), layout);
            let next_blizzard_positions = blizzards_to_positions(&next_blizzards);
            blizzards_by_step.push(next_blizzards);
            blizzard_positions_by_step.push(next_blizzard_positions);
        }
        let next_blizzard_positions = &blizzard_positions_by_step[next_minute];

        for direction in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::None,
        ] {
            let next_position = state.position.next(direction);

            if !layout.is_valid_position(next_position) {
                continue;
            }

            if next_position == final_position {
                return (
                    State {
                        position: next_position,
                        minute: next_minute,
                    },
                    blizzards_by_step[next_minute].clone(),
                );
            }

            if !next_blizzard_positions.contains(&next_position) {
                queue.push(State {
                    position: next_position,
                    minute: next_minute,
                });
            }
        }
    }
    panic!("could not find path");
}

#[derive(Clone, Debug)]
struct State {
    position: Position,
    minute: usize,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.minute.eq(&other.minute)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.minute.cmp(&self.minute)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.minute.cmp(&self.minute))
    }
}

fn blizzards_to_positions(blizzards: &Vec<Blizzard>) -> HashSet<Position> {
    blizzards
        .iter()
        .map(|&b| b.position)
        .collect::<HashSet<Position>>()
}

fn update_blizzards(blizzards: &Vec<Blizzard>, layout: Layout) -> Vec<Blizzard> {
    let mut updated_blizzards = Vec::new();
    for b in blizzards.iter() {
        let mut next_position = b.position.next(b.direction);
        if !layout.is_valid_position(next_position) {
            next_position = layout.wrap_around(b.position, b.direction);
        }
        updated_blizzards.push(Blizzard {
            position: next_position,
            direction: b.direction,
        });
    }
    updated_blizzards
}

fn _print(position: Position, blizzards: &Vec<Blizzard>, layout: Layout) {
    let mut map = Vec::new();
    for _ in 0..layout.height {
        map.push(vec!['.'; layout.width]);
    }

    for y in 0..layout.height {
        map[y][0] = '#';
        map[y][layout.width - 1] = '#';
    }

    for x in 0..layout.width {
        map[0][x] = '#';
        map[layout.height - 1][x] = '#';
    }

    map[position.y as usize][position.x as usize] = 'x';

    for b in blizzards.iter() {
        map[b.position.y as usize][b.position.x as usize] = b.direction.into();
    }

    let s = map
        .iter()
        .map(|r| r.iter().collect::<String>() + "\n")
        .collect::<String>();
    println!("{}", s);
}
