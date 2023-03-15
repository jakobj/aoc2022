use std::fs;

fn main() {
    let filename = "inputs/22.txt";
    let content = fs::read_to_string(filename).unwrap();
    let map = parse_map(&content);
    let instructions = parse_instructions(&content);
    let (final_position, final_orientation) = navigate(&map, &instructions);

    let score_orientation = |c| match c {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    let password = (final_position.y + 1) * 1000
        + (final_position.x + 1) * 4
        + score_orientation(final_orientation);
    println!("The final password is {}.", password);
}

fn parse_map(content: &str) -> Vec<Vec<Tile>> {
    let mut map = Vec::new();
    for l in content.lines() {
        if l.len() == 0 {
            break;
        }

        let mut row = Vec::new();
        for c in l.chars() {
            row.push(Tile::from(c));
        }
        map.push(row);
    }

    // make map rectangular (simplifies logic later)
    let max_cols = map.iter().map(|r| r.len()).max().unwrap();
    for row in map.iter_mut() {
        row.resize(max_cols, Tile::Void);
    }
    map
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Open,
    Void,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("unkown tile type"),
        }
    }
}

fn parse_instructions(content: &str) -> Vec<Instruction> {
    fn is_number(c: char) -> bool {
        if let Some(_) = c.to_digit(10) {
            return true;
        } else {
            return false;
        }
    }

    let raw_instructions = content
        .lines()
        .last()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut instructions = Vec::new();
    let mut current = 0;
    while current < raw_instructions.len() {
        if is_number(raw_instructions[current]) {
            // consume all digits and convert them to a number
            let mut steps = Vec::new();
            while current < raw_instructions.len() && is_number(raw_instructions[current]) {
                steps.push(raw_instructions[current]);
                current += 1;
            }
            let steps = steps
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            instructions.push(Instruction::Move(steps));
        } else {
            let direction = raw_instructions[current];
            current += 1;
            instructions.push(Instruction::Turn(TurnDirection::from(direction)));
        }
    }
    instructions
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Move(usize),
    Turn(TurnDirection),
}

#[derive(Copy, Clone, Debug)]
enum TurnDirection {
    Left,
    Right,
}

impl From<char> for TurnDirection {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unknown turn direction"),
        }
    }
}

fn navigate(map: &Vec<Vec<Tile>>, instructions: &Vec<Instruction>) -> (Position, Orientation) {
    let mut position = Position {
        y: 0,
        x: find_open_position_in_row(map, 0, true).unwrap(),
    };
    let mut orientation = Orientation::Right;
    for ins in instructions.iter() {
        match ins {
            Instruction::Move(steps) => {
                position = move_straight(map, position, orientation, *steps)
            }
            Instruction::Turn(direction) => orientation = turn(orientation, *direction),
        };
    }
    (position, orientation)
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Down,
    Left,
    Right,
    Up,
}

impl Orientation {
    fn succ(&self) -> Self {
        match self {
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Up => Orientation::Right,
        }
    }

    fn pred(&self) -> Self {
        match self {
            Orientation::Right => Orientation::Up,
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
        }
    }
}

fn find_open_position_in_row(map: &Vec<Vec<Tile>>, y: i64, from_front: bool) -> Option<i64> {
    if from_front {
        for (x, t) in map[y as usize].iter().enumerate() {
            if matches!(t, Tile::Open) {
                return Some(x as i64);
            } else if matches!(t, Tile::Wall) {
                return None;
            }
        }
        return None;
    } else {
        for (x, t) in map[y as usize].iter().enumerate().rev() {
            if matches!(t, Tile::Open) {
                return Some(x as i64);
            } else if matches!(t, Tile::Wall) {
                return None;
            }
        }
        return None;
    }
}

fn find_open_position_in_col(map: &Vec<Vec<Tile>>, x: i64, from_front: bool) -> Option<i64> {
    if from_front {
        for y in 0..map.len() {
            let t = map[y][x as usize];
            if matches!(t, Tile::Open) {
                return Some(y as i64);
            } else if matches!(t, Tile::Wall) {
                return None;
            }
        }
        return None;
    } else {
        for y in (0..map.len()).rev() {
            let t = map[y][x as usize];
            if matches!(t, Tile::Open) {
                return Some(y as i64);
            } else if matches!(t, Tile::Wall) {
                return None;
            }
        }
        return None;
    }
}

fn move_straight(
    map: &Vec<Vec<Tile>>,
    position: Position,
    orientation: Orientation,
    steps: usize,
) -> Position {
    let delta_x = match orientation {
        Orientation::Right => 1,
        Orientation::Left => -1,
        _ => 0,
    };

    let delta_y = match orientation {
        Orientation::Down => 1,
        Orientation::Up => -1,
        _ => 0,
    };

    let Position { mut x, mut y } = position;
    for _ in 0..steps {
        if delta_x != 0 {
            // moving horizontally
            let mut next_x = x + delta_x;
            if next_x < 0
                || (next_x >= 0
                    && delta_x < 0
                    && matches!(map[y as usize][next_x as usize], Tile::Void))
            {
                next_x = match find_open_position_in_row(map, y, false) {
                    Some(x) => x,
                    None => x,
                }
            } else if next_x >= map[y as usize].len() as i64
                || (next_x < map[y as usize].len() as i64
                    && delta_x > 0
                    && matches!(map[y as usize][next_x as usize], Tile::Void))
            {
                next_x = match find_open_position_in_row(map, y, true) {
                    Some(x) => x,
                    None => x,
                }
            } else if matches!(map[y as usize][next_x as usize], Tile::Wall) {
                return Position { x, y };
            }
            x = next_x;
        } else {
            // moving vertically
            let mut next_y = y + delta_y;
            if next_y < 0
                || (next_y >= 0
                    && delta_y < 0
                    && matches!(map[next_y as usize][x as usize], Tile::Void))
            {
                next_y = match find_open_position_in_col(map, x, false) {
                    Some(y) => y,
                    None => y,
                }
            } else if next_y >= map.len() as i64
                || (next_y < map.len() as i64
                    && delta_y > 0
                    && matches!(map[next_y as usize][x as usize], Tile::Void))
            {
                next_y = match find_open_position_in_col(map, x, true) {
                    Some(y) => y,
                    None => y,
                }
            } else if matches!(map[next_y as usize][x as usize], Tile::Wall) {
                return Position { x, y };
            }
            y = next_y;
        }
    }
    Position { x, y }
}

fn turn(orientation: Orientation, direction: TurnDirection) -> Orientation {
    match direction {
        TurnDirection::Left => orientation.pred(),
        TurnDirection::Right => orientation.succ(),
    }
}
