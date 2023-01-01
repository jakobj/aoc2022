use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/17.txt";
    let jet_pattern = fs::read_to_string(filename).expect("Should be able to read file");
    let rocks = let_rocks_fall(jet_pattern.trim());
    // print_tower(&rocks);
    let top = find_top(&rocks) + 1;
    println!("The tower will be {} units tall.", top);
}

fn let_rocks_fall(jet_pattern: &str) -> Vec<Rock> {
    let jet_pattern = jet_pattern.chars().collect::<Vec<char>>();
    let mut ticks = Vec::with_capacity(1000);
    let mut tick = 0;
    let mut rocks = Vec::with_capacity(1000);
    let mut rock = Rock::new(0, 3);
    let mut top = 0;
    let pattern_height = 10;
    let max_n_rocks = 1_000_000_000_000;
    // let max_n_rocks = 2022;
    let mut n_rocks = 0;
    while n_rocks < max_n_rocks {
        let jet = jet_pattern[tick % jet_pattern.len()];
        match jet {
            '<' => rock.move_left(&rocks),
            '>' => rock.move_right(&rocks),
            _ => panic!("Unknown jet direction"),
        }
        let success = rock.move_down(&rocks);

        if !success {
            if rock.top() > top {
                top = rock.top();
            }
            n_rocks += 1;
            rocks.push(rock);
            ticks.push(tick);
            rock = Rock::new(rocks.len() % 5, top + 4);
        }

        if rocks.len() > 100 && rocks.len() % 100 == 0 {
            // fast forward
            if let Some((start_first, start_second)) = search_pattern(&rocks, pattern_height) {
                let delta_rocks = start_second - start_first;
                if max_n_rocks - n_rocks > delta_rocks {
                    let delta_ticks = ticks[start_second] - ticks[start_first];
                    let delta_bottom = rocks[start_second].bottom - rocks[start_first].bottom;

                    rocks.truncate(start_second);
                    ticks.truncate(start_second);
                    n_rocks = rocks.len();
                    tick = *ticks.last().unwrap();
                    top = find_top(&rocks);

                    let n_missing = max_n_rocks - n_rocks;
                    let n_patterns = n_missing / delta_rocks;
                    n_rocks += n_patterns * delta_rocks;
                    tick += n_patterns * delta_ticks;
                    top += n_patterns * delta_bottom;
                    for i in 1..delta_rocks {
                        rocks[start_first + i].bottom += n_patterns * delta_bottom;
                    }

                    rock = Rock::new(rocks.len() % 5, top + 4);
                }
            }
        }

        tick += 1;
    }
    assert!(n_rocks <= max_n_rocks);
    rocks
}

#[derive(Clone, Copy, Debug)]
enum RockKind {
    Hbar,
    Plus,
    RL,
    VBar,
    Square,
}

#[derive(Clone, Copy, Debug)]
struct Rock {
    kind: RockKind,
    bottom: usize,
    left: usize,
}

impl Rock {
    fn new(kind_idx: usize, bottom: usize) -> Self {
        match kind_idx {
            0 => Self {
                kind: RockKind::Hbar,
                bottom,
                left: 2,
            },
            1 => Self {
                kind: RockKind::Plus,
                bottom,
                left: 2,
            },
            2 => Self {
                kind: RockKind::RL,
                bottom,
                left: 2,
            },
            3 => Self {
                kind: RockKind::VBar,
                bottom,
                left: 2,
            },
            4 => Self {
                kind: RockKind::Square,
                bottom,
                left: 2,
            },
            _ => panic!("Unknown kind index"),
        }
    }

    fn move_left(&mut self, rocks: &Vec<Rock>) {
        if self.left == 0 {
            return;
        }
        assert!(self.left > 0);
        match self.kind {
            RockKind::Hbar => {
                if is_occupied(self.bottom, self.left - 1, rocks) {
                    return;
                }
            }
            RockKind::Plus => {
                if is_occupied(self.bottom, self.left + 1 - 1, rocks)
                    || is_occupied(self.bottom + 1, self.left - 1, rocks)
                    || is_occupied(self.bottom + 2, self.left + 1 - 1, rocks)
                {
                    return;
                }
            }
            RockKind::RL => {
                if is_occupied(self.bottom, self.left - 1, rocks)
                    || is_occupied(self.bottom + 1, self.left + 2 - 1, rocks)
                    || is_occupied(self.bottom + 2, self.left + 2 - 1, rocks)
                {
                    return;
                }
            }
            RockKind::VBar => {
                for i in 0..4 {
                    if is_occupied(self.bottom + i, self.left - 1, rocks) {
                        return;
                    }
                }
            }
            RockKind::Square => {
                for i in 0..2 {
                    if is_occupied(self.bottom + i, self.left - 1, rocks) {
                        return;
                    }
                }
            }
        }
        self.left -= 1;
        assert!(self.left <= 6);
    }

    fn move_right(&mut self, rocks: &Vec<Rock>) {
        if self.right() >= 6 {
            return;
        }
        match self.kind {
            RockKind::Hbar => {
                if is_occupied(self.bottom, self.right() + 1, rocks) {
                    return;
                }
            }
            RockKind::Plus => {
                if is_occupied(self.bottom, self.right() - 1 + 1, rocks)
                    || is_occupied(self.bottom + 1, self.right() + 1, rocks)
                    || is_occupied(self.bottom + 2, self.right() - 1 + 1, rocks)
                {
                    return;
                }
            }
            RockKind::RL => {
                for i in 0..3 {
                    if is_occupied(self.bottom + i, self.right() + 1, rocks) {
                        return;
                    }
                }
            }
            RockKind::VBar => {
                for i in 0..4 {
                    if is_occupied(self.bottom + i, self.right() + 1, rocks) {
                        return;
                    }
                }
            }
            RockKind::Square => {
                for i in 0..2 {
                    if is_occupied(self.bottom + i, self.right() + 1, rocks) {
                        return;
                    }
                }
            }
        }
        self.left += 1;
        assert!(self.left <= 6);
    }

    fn move_down(&mut self, rocks: &Vec<Rock>) -> bool {
        if self.bottom == 0 {
            return false;
        }
        assert!(self.bottom > 0);
        match self.kind {
            RockKind::Hbar => {
                for i in 0..4 {
                    if is_occupied(self.bottom - 1, self.left + i, rocks) {
                        return false;
                    }
                }
            }
            RockKind::Plus => {
                if is_occupied(self.bottom - 1, self.left + 1, rocks)
                    || is_occupied(self.bottom + 1 - 1, self.left, rocks)
                    || is_occupied(self.bottom + 1 - 1, self.right(), rocks)
                {
                    return false;
                }
            }
            RockKind::RL => {
                for i in 0..3 {
                    if is_occupied(self.bottom - 1, self.left + i, rocks) {
                        return false;
                    }
                }
            }
            RockKind::VBar => {
                if is_occupied(self.bottom - 1, self.left, rocks) {
                    return false;
                }
            }
            RockKind::Square => {
                for i in 0..2 {
                    if is_occupied(self.bottom - 1, self.left + i, rocks) {
                        return false;
                    }
                }
            }
        };
        self.bottom -= 1;
        true
    }

    fn occupies(&self, i: usize, j: usize) -> bool {
        if i < self.bottom {
            return false;
        }
        if j < self.left {
            return false;
        }
        assert!(i >= self.bottom);
        assert!(j >= self.left);
        let y = i - self.bottom;
        let x = j - self.left;
        match self.kind {
            RockKind::Hbar => {
                if y == 0 && x < 4 {
                    return true;
                }
            }
            RockKind::Plus => {
                if (y == 0 && x == 1) || (y == 1 && x < 3) || (y == 2 && x == 1) {
                    return true;
                }
            }
            RockKind::RL => {
                if (y == 0 && x < 3) || (y == 1 && x == 2) || (y == 2 && x == 2) {
                    return true;
                }
            }
            RockKind::VBar => {
                if y < 4 && x == 0 {
                    return true;
                }
            }
            RockKind::Square => {
                if y < 2 && x < 2 {
                    return true;
                }
            }
        };
        false
    }

    fn right(&self) -> usize {
        match self.kind {
            RockKind::Hbar => self.left + 3,
            RockKind::Plus => self.left + 2,
            RockKind::RL => self.left + 2,
            RockKind::VBar => self.left,
            RockKind::Square => self.left + 1,
        }
    }

    fn top(&self) -> usize {
        match self.kind {
            RockKind::Hbar => self.bottom,
            RockKind::Plus => self.bottom + 2,
            RockKind::RL => self.bottom + 2,
            RockKind::VBar => self.bottom + 3,
            RockKind::Square => self.bottom + 1,
        }
    }
}

fn is_occupied(i: usize, j: usize, rocks: &[Rock]) -> bool {
    for r in rocks.iter().rev() {
        if r.occupies(i, j) {
            return true;
        }
        if i > 100 && r.top() < i - 100 {
            break;
        }
    }
    false
}

fn print_tower(rocks: &[Rock]) {
    let max_row = find_top(rocks) + 5;
    let mut screen = Vec::new();
    screen.push("  0+-------+".to_string());
    let mut i = 0;
    while i < max_row {
        let mut row = String::new();
        row.push_str(format!("{:02}", i + 1).as_str());
        row.push('|');
        for j in 0..7 {
            if is_occupied(i, j, rocks) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        row.push('|');
        screen.push(row);
        i += 1;
    }
    screen.reverse();
    for l in screen {
        println!("{}", l);
    }
}

fn find_top(rocks: &[Rock]) -> usize {
    rocks.iter().map(|r| r.top()).max().unwrap()
}

fn search_pattern(rocks: &[Rock], count: usize) -> Option<(usize, usize)> {
    let mut patterns = HashMap::new();
    for lowest_rock in 0..rocks.len() - count {
        let bottom = rocks[lowest_rock].bottom;
        let mut key = String::new();
        for r in &rocks[lowest_rock..lowest_rock + count] {
            let k = match r.kind {
                RockKind::Hbar => 0,
                RockKind::Plus => 1,
                RockKind::RL => 2,
                RockKind::Square => 3,
                RockKind::VBar => 4,
            };
            key.push_str(&k.to_string());
            key.push_str(&r.left.to_string());
            key.push_str(&(r.bottom - bottom).to_string());
        }
        if patterns.contains_key(&key) {
            return Some((patterns[&key], lowest_rock));
        }
        patterns.insert(key, lowest_rock);
    }
    None
}
