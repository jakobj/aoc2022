use std::{collections::HashMap, fs};

// const SIDE_LENGTH: usize = 4;
const SIDE_LENGTH: usize = 50;

fn main() {
    // let filename = "inputs/22b.txt";
    let filename = "inputs/22.txt";
    let content = fs::read_to_string(filename).unwrap();
    let map = parse_map(&content);
    let sections = cut_map_into_sections(&map);
    let instructions = parse_instructions(&content);
    let (final_position, final_orientation) = navigate(&sections, &instructions);

    let score_orientation = |c| match c {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    let password = (sections[&final_position.section].offset_y as i64 + final_position.y + 1)
        * 1000
        + (sections[&final_position.section].offset_x as i64 + final_position.x + 1) * 4
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

fn cut_map_into_sections(map: &Vec<Vec<Tile>>) -> HashMap<usize, Section> {
    let max_y = map.len();
    let max_x = map.iter().map(|v| v.len()).max().unwrap();
    let mut sections = HashMap::new();
    let mut section_idx = 1;
    'loop_y: for offset_y in (0..max_y).step_by(SIDE_LENGTH) {
        'loop_x: for offset_x in (0..max_x).step_by(SIDE_LENGTH) {
            if offset_x >= map[offset_y].len() {
                continue 'loop_y;
            }

            let mut tiles = Vec::new();
            for delta_y in 0..SIDE_LENGTH {
                let mut row = Vec::new();
                for delta_x in 0..SIDE_LENGTH {
                    let t = map[offset_y + delta_y][offset_x + delta_x];
                    if let Tile::Void = t {
                        continue 'loop_x;
                    }
                    row.push(t);
                }
                tiles.push(row);
            }
            sections.insert(
                section_idx,
                Section {
                    tiles,
                    offset_x,
                    offset_y,
                },
            );
            section_idx += 1;
        }
    }
    sections
}

#[derive(Debug)]
struct Section {
    tiles: Vec<Vec<Tile>>,
    offset_x: usize,
    offset_y: usize,
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

fn navigate(
    sections: &HashMap<usize, Section>,
    instructions: &Vec<Instruction>,
) -> (Position, Orientation) {
    let mut position = Position {
        section: 1,
        y: 0,
        x: 0,
    };
    let mut orientation = Orientation::Right;
    for ins in instructions.iter() {
        match ins {
            Instruction::Move(steps) => {
                (position, orientation) = move_straight(sections, position, orientation, *steps)
            }
            Instruction::Turn(direction) => orientation = turn(orientation, *direction),
        };
    }
    (position, orientation)
}

#[derive(Copy, Clone, Debug)]
struct Position {
    section: usize,
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

fn move_straight(
    sections: &HashMap<usize, Section>,
    mut position: Position,
    mut orientation: Orientation,
    steps: usize,
) -> (Position, Orientation) {
    for _ in 0..steps {
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

        let mut next_position = Position {
            section: position.section,
            x: position.x + delta_x,
            y: position.y + delta_y,
        };
        let mut next_orientation = orientation;
        if (next_position.x < 0 || next_position.x >= SIDE_LENGTH as i64)
            | (next_position.y < 0 || next_position.y >= SIDE_LENGTH as i64)
        {
            (next_position, next_orientation) = transition(position, orientation);
        }

        // stop upon if a wall is at the new position
        if matches!(
            sections[&next_position.section].tiles[next_position.y as usize]
                [next_position.x as usize],
            Tile::Wall
        ) {
            return (position, orientation);
        }

        position = next_position;
        orientation = next_orientation;
    }
    (position, orientation)
}

fn turn(orientation: Orientation, direction: TurnDirection) -> Orientation {
    match direction {
        TurnDirection::Left => orientation.pred(),
        TurnDirection::Right => orientation.succ(),
    }
}

// // transitions for flat test map
// fn transition(position: Position, orientation: Orientation) -> (Position, Orientation) {
//     let max = SIDE_LENGTH as i64 - 1;
//     let Position{ section, x, y } = position;
//     match section {
//         1 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 5, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 1, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 4, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 1, x: max-x, y }, Orientation::Left),
//             }
//         },
//         2 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 2, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 3, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 2, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 4, x: max-x, y }, Orientation::Left),
//             }
//         },
//         3 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 3, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 4, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 3, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 2, x: max-x, y }, Orientation::Left),
//             }
//         },
//         4 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 1, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 2, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 5, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 3, x: max-x, y }, Orientation::Left),
//             }
//         },
//         5 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 4, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 6, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 1, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 6, x: max-x, y }, Orientation::Left),
//             }
//         },
//         6 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 6, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 5, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 6, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 5, x: max-x, y }, Orientation::Left),
//             }
//         },
//         _ => panic!("unknown section"),
//     }
// }

// // transitions for flat map
// fn transition(position: Position, orientation: Orientation) -> (Position, Orientation) {
//     let max = SIDE_LENGTH as i64 - 1;
//     let Position{ section, x, y } = position;
//     match section {
//         1 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 5, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 2, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 3, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 2, x: max-x, y }, Orientation::Left),
//             }
//         },
//         2 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 2, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 1, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 2, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 1, x: max-x, y }, Orientation::Left),
//             }
//         },
//         3 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 1, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 3, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 5, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 3, x: max-x, y }, Orientation::Left),
//             }
//         },
//         4 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 6, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 5, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 6, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 5, x: max-x, y }, Orientation::Left),
//             }
//         },
//         5 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 3, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 4, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 1, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 4, x: max-x, y }, Orientation::Left),
//             }
//         },
//         6 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 4, x, y: max-y }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 6, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 4, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 6, x: max-x, y }, Orientation::Left),
//             }
//         },
//         _ => panic!("unknown section"),
//     }
// }

// // transitions for cube test map
// fn transition(position: Position, orientation: Orientation) -> (Position, Orientation) {
//     let max = SIDE_LENGTH as i64 - 1;
//     let Position{ section, x, y } = position;
//     match section {
//         1 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 2, x: max-x, y }, Orientation::Down),
//                 Orientation::Right => (Position{ section: 6, x, y: max-y }, Orientation::Left),
//                 Orientation::Down => (Position{ section: 4, x, y: max-y }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 3, x: y, y: x }, Orientation::Down),
//             }
//         },
//         2 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 1, x: max-x, y }, Orientation::Down),
//                 Orientation::Right => (Position{ section: 3, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 5, x: max-x, y }, Orientation::Up),
//                 Orientation::Left => (Position{ section: 6, x: max-y, y: max-x }, Orientation::Up),
//             }
//         },
//         3 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 1, x: y, y: x }, Orientation::Right),
//                 Orientation::Right => (Position{ section: 4, x: max-x, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 5, x: 0, y: max-x }, Orientation::Right),
//                 Orientation::Left => (Position{ section: 2, x: max, y }, Orientation::Left),
//             }
//         },
//         4 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 1, x, y: max }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 6, x: max-y, y: 0 }, Orientation::Down),
//                 Orientation::Down => (Position{ section: 5, x, y: 0 }, Orientation::Down),
//                 Orientation::Left => (Position{ section: 3, x: max, y }, Orientation::Left),
//             }
//         },
//         5 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 4, x, y: max }, Orientation::Up),
//                 Orientation::Right => (Position{ section: 6, x: 0, y }, Orientation::Right),
//                 Orientation::Down => (Position{ section: 2, x: max-x, y: max }, Orientation::Up),
//                 Orientation::Left => (Position{ section: 3, x: max-y, y: max }, Orientation::Up),
//             }
//         },
//         6 => {
//             match orientation {
//                 Orientation::Up => (Position{ section: 4, x: max, y: max-x }, Orientation::Left),
//                 Orientation::Right => (Position{ section: 1, x: max, y: max-y }, Orientation::Left),
//                 Orientation::Down => (Position{ section: 2, x: 0, y: max-x }, Orientation::Right),
//                 Orientation::Left => (Position{ section: 5, x: max, y }, Orientation::Left),
//             }
//         },
//         _ => panic!("unknown section"),
//     }
// }

// transitions for cube map
fn transition(position: Position, orientation: Orientation) -> (Position, Orientation) {
    let max = SIDE_LENGTH as i64 - 1;
    let Position{ section, x, y } = position;
    match section {
        1 => {
            match orientation {
                Orientation::Up => (Position{ section: 6, x: 0, y: x }, Orientation::Right),
                Orientation::Right => (Position{ section: 2, x: 0, y }, Orientation::Right),
                Orientation::Down => (Position{ section: 3, x, y: 0 }, Orientation::Down),
                Orientation::Left => (Position{ section: 4, x: 0, y: max-y }, Orientation::Right),
            }
        },
        2 => {
            match orientation {
                Orientation::Up => (Position{ section: 6, x, y: max }, Orientation::Up),
                Orientation::Right => (Position{ section: 5, x: max, y: max-y }, Orientation::Left),
                Orientation::Down => (Position{ section: 3, x: max, y: x }, Orientation::Left),
                Orientation::Left => (Position{ section: 1, x: max, y }, Orientation::Left),
            }
        },
        3 => {
            match orientation {
                Orientation::Up => (Position{ section: 1, x, y: max }, Orientation::Up),
                Orientation::Right => (Position{ section: 2, x: y, y: max }, Orientation::Up),
                Orientation::Down => (Position{ section: 5, x, y: 0 }, Orientation::Down),
                Orientation::Left => (Position{ section: 4, x: y, y: 0 }, Orientation::Down),
            }
        },
        4 => {
            match orientation {
                Orientation::Up => (Position{ section: 3, x: 0, y: x }, Orientation::Right),
                Orientation::Right => (Position{ section: 5, x: 0, y }, Orientation::Right),
                Orientation::Down => (Position{ section: 6, x, y: 0 }, Orientation::Down),
                Orientation::Left => (Position{ section: 1, x: 0, y: max-y }, Orientation::Right),
            }
        },
        5 => {
            match orientation {
                Orientation::Up => (Position{ section: 3, x, y: max }, Orientation::Up),
                Orientation::Right => (Position{ section: 2, x: max, y: max-y }, Orientation::Left),
                Orientation::Down => (Position{ section: 6, x: max, y: x }, Orientation::Left),
                Orientation::Left => (Position{ section: 4, x: max, y }, Orientation::Left),
            }
        },
        6 => {
            match orientation {
                Orientation::Up => (Position{ section: 4, x, y: max }, Orientation::Up),
                Orientation::Right => (Position{ section: 5, x: y, y: max }, Orientation::Up),
                Orientation::Down => (Position{ section: 2, x, y: 0 }, Orientation::Down),
                Orientation::Left => (Position{ section: 1, x: y, y: 0 }, Orientation::Down),
            }
        },
        _ => panic!("unknown section"),
    }
}
