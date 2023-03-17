use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/23.txt";
    let content = fs::read_to_string(filename).unwrap();

    let elves = parse_elf_positions(&content);
    let (elves, _) = distribute_elves(elves, 10);
    let count: usize = to_string(&elves)
        .chars()
        .map(|c| if c == '.' { 1 } else { 0 })
        .sum();
    println!(
        "There are {} empty ground tiles in the rectangle spanned by the elves.",
        count
    );

    let elves = parse_elf_positions(&content);
    let (_, round) = distribute_elves(elves, 100_000);
    println!(
        "The number of the first round where no Elf moves is {}.",
        round + 1
    );
}

fn parse_elf_positions(content: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    for (y, l) in content.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                elves.push(Elf {
                    position: Position {
                        x: x as i64,
                        y: y as i64,
                    },
                    proposal: None,
                });
            }
        }
    }
    elves
}

#[derive(Clone, Copy, Debug)]
struct Elf {
    position: Position,
    proposal: Option<Position>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

fn distribute_elves(mut elves: Vec<Elf>, rounds: usize) -> (Vec<Elf>, usize) {
    let mut previous_elf_configuration = format!("{:?}", elves);

    let mut round = 0;
    while round < rounds {
        let mut elves_with_proposals = Vec::new();
        let positions = elves
            .iter()
            .map(|e| e.position)
            .collect::<HashSet<Position>>();
        for e in elves.iter() {
            elves_with_proposals.push(propose(e, &positions, round));
        }

        elves.clear();
        let proposed_positions = determine_proposed_positions(&elves_with_proposals);
        for e in elves_with_proposals {
            if proposed_positions[&e.proposal.unwrap()] < 2 {
                elves.push(Elf {
                    position: e.proposal.unwrap(),
                    proposal: None,
                });
            } else {
                elves.push(Elf {
                    position: e.position,
                    proposal: None,
                });
            }
        }

        let elf_configuration = format!("{:?}", elves);
        if elf_configuration == previous_elf_configuration {
            return (elves, round);
        }
        previous_elf_configuration = elf_configuration;

        round += 1;
    }
    (elves, round)
}

fn propose(e: &Elf, positions: &HashSet<Position>, round: usize) -> Elf {
    // first check whether another elf is nearby
    let mut elf_nearby = false;
    for delta_y in [-1, 0, 1] {
        for delta_x in [-1, 0, 1] {
            if delta_y == 0 && delta_x == 0 {
                continue;
            }
            if positions.contains(&Position {
                x: e.position.x + delta_x,
                y: e.position.y + delta_y,
            }) {
                elf_nearby = true;
            }
        }
    }
    if !elf_nearby {
        return Elf {
            position: e.position,
            proposal: Some(e.position),
        };
    }

    // then choose a move if a(t least) one elf is nearby
    for i in 0..4 {
        let idx = (round + i) % 4;
        let proposal = match idx {
            0 => move_vertical(e.position, positions, -1),
            1 => move_vertical(e.position, positions, 1),
            2 => move_horizontal(e.position, positions, -1),
            3 => move_horizontal(e.position, positions, 1),
            _ => panic!("should not be reached"),
        };
        if proposal.is_some() {
            return Elf {
                position: e.position,
                proposal,
            };
        }
    }

    Elf {
        position: e.position,
        proposal: Some(e.position),
    }
}

fn move_vertical(
    position: Position,
    positions: &HashSet<Position>,
    delta_y: i64,
) -> Option<Position> {
    for delta_x in [-1, 0, 1] {
        if positions.contains(&Position {
            x: position.x + delta_x,
            y: position.y + delta_y,
        }) {
            return None;
        }
    }
    Some(Position {
        x: position.x,
        y: position.y + delta_y,
    })
}

fn move_horizontal(
    position: Position,
    positions: &HashSet<Position>,
    delta_x: i64,
) -> Option<Position> {
    for delta_y in [-1, 0, 1] {
        if positions.contains(&Position {
            x: position.x + delta_x,
            y: position.y + delta_y,
        }) {
            return None;
        }
    }
    Some(Position {
        x: position.x + delta_x,
        y: position.y,
    })
}

fn determine_proposed_positions(elves_with_proposals: &Vec<Elf>) -> HashMap<Position, usize> {
    let mut proposed_positions = HashMap::new();
    for e in elves_with_proposals.iter() {
        let count;
        if !proposed_positions.contains_key(&e.proposal.unwrap()) {
            count = 1
        } else {
            count = proposed_positions[&e.proposal.unwrap()] + 1;
        }
        proposed_positions.insert(e.proposal.unwrap(), count);
    }
    proposed_positions
}

fn to_string(elves: &Vec<Elf>) -> String {
    let min_y = elves.iter().map(|e| e.position.y).min().unwrap();
    let max_y = elves.iter().map(|e| e.position.y).max().unwrap();
    let min_x = elves.iter().map(|e| e.position.x).min().unwrap();
    let max_x = elves.iter().map(|e| e.position.x).max().unwrap();

    let length_y = max_y - min_y + 1;
    let length_x = max_x - min_x + 1;

    let mut map = Vec::new();
    for _ in 0..length_y {
        map.push(vec!['.'; length_x as usize]);
    }

    for e in elves {
        map[(e.position.y - min_y) as usize][(e.position.x - min_x) as usize] = '#';
    }

    let mut s = String::new();
    for y in 0..length_y {
        for x in 0..length_x {
            s.push(map[y as usize][x as usize]);
        }
        s.push('\n');
    }
    s.pop();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_to_string() {
        let content = ".....
..##.
..#..
.....
..##.
.....";
        let expected = "##
#.
..
##";
        let elves = parse_elf_positions(&content);
        let s = to_string(&elves);
        assert_eq!(s, expected);
    }

    #[test]
    fn test_distribute_small() {
        let content = ".....
..##.
..#..
.....
..##.
.....";
        let expected = "..#..
....#
#....
....#
.....
..#..";
        let elves = parse_elf_positions(&content);
        let (elves, _) = distribute_elves(elves, 3);
        let s = to_string(&elves);
        assert_eq!(s, expected);

        // since this is a configuration where no elves need to move, nothing
        // should change if we distribute for some more rounds
        let (elves, _) = distribute_elves(elves, 3);
        let s = to_string(&elves);
        assert_eq!(s, expected);
    }

    #[test]
    fn test_distribute_large() {
        let content = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";
        let expected = "......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..";
        let elves = parse_elf_positions(&content);
        let (elves, _) = distribute_elves(elves, 10);
        let s = to_string(&elves);
        assert_eq!(s, expected);
    }
}
