fn main() {
    let filename = "inputs/20.txt";
    let content = std::fs::read_to_string(filename).unwrap();
    let (mut first, length) = parse_sequence(&content, 1);

    mix(&mut first, length);
    let grove_coordinates = compute_grove_coordinates(&first, length);
    println!("The grove coordinates are {}.", grove_coordinates);

    let (mut first, length) = parse_sequence(&content, 811589153);
    // print_numbers(&first);
    for i in 0..10 {
        mix(&mut first, length);
        // print!("{}: ", i + 1);
        // print_numbers(&first);
    }
    let grove_coordinates = compute_grove_coordinates(&first, length);
    println!("The grove coordinates are {}.", grove_coordinates);
}

fn parse_sequence(content: &str, multiplier: i64) -> (Node<(usize, i64)>, usize) {
    let mut items = content
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse::<i64>().unwrap() * multiplier))
        .collect::<Vec<(usize, i64)>>();
    let length = items.len();
    let last_item = items.pop().unwrap();
    let mut next = Box::new(Node::new(last_item, None));
    for item in items.into_iter().rev() {
        let node = Box::new(Node::new(item, Some(next)));
        next = node;
    }
    (*next, length)
}

#[derive(Debug, Clone)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(item: T, next: Option<Box<Self>>) -> Self {
        Self { item, next }
    }

    fn find(&self, criterion: &dyn Fn(&T) -> bool, offset: usize) -> usize {
        if criterion(&self.item) {
            return offset;
        }
        if let Some(next) = self.next.as_ref() {
            return next.find(criterion, offset + 1);
        } else {
            panic!("node not found");
        }
    }

    fn remove(&mut self, offset: usize) -> Self {
        if offset == 0 {
            let item = self.item.clone();
            let mut node = self.next.take().unwrap();
            self.item = node.item;
            self.next = node.next.take();
            return Node::new(item, None);
        }

        if offset == 1 {
            let mut node = self.next.take().unwrap();
            self.next = node.next.take();
            return *node;
        }

        if let Some(next) = self.next.as_mut() {
            return next.remove(offset - 1);
        } else {
            panic!("could not remove node");
        }
    }

    fn insert(&mut self, item: T, offset: usize) {
        // assert!(offset > 0);
        if offset == 0 {
            let node = Box::new(Self::new(self.item.clone(), self.next.take()));
            self.item = item;
            self.next = Some(node);
            return;
        }

        if offset == 1 {
            let mut node = Box::new(Self::new(item, None));
            node.next = self.next.take();
            self.next = Some(node);
            return;
        }

        if let Some(next) = self.next.as_mut() {
            next.insert(item, offset - 1);
        } else {
            panic!("could not insert node");
        }
    }

    fn get(&self, offset: usize) -> T {
        if offset == 0 {
            return self.item.clone();
        }
        if let Some(next) = self.next.as_ref() {
            return next.get(offset - 1);
        } else {
            panic!("could not get item");
        }
    }
}

fn print_numbers(mut node: &Node<(usize, i64)>) {
    print!("{}", node.item.1);
    while let Some(next) = node.next.as_ref() {
        print!(",{}", next.item.1);
        node = next;
    }
    println!("");
}

fn determine_offset(offset: i64, move_by: i64, current_length: i64) -> usize {
    let move_by = move_by % current_length;
    if move_by == 0 {
        return offset as usize;
    }
    let tmp;
    if offset + move_by <= 0 {
        // println!("{} {} -> {}", offset, move_by, length + (offset + move_by));
        tmp = current_length + (offset + move_by);
        // panic!();
    } else if offset + move_by > current_length {
        // println!("{} {} -> {}", offset, move_by, move_by - (length - offset as i64) as i64);
        tmp = move_by - (current_length - offset);
    } else {
        tmp = offset + move_by;
    }
    if !(tmp > 0 && tmp <= current_length) {
        println!(
            "current_length {} offset {} move_by {} tmp {}",
            current_length, offset, move_by, tmp
        );
    }
    assert!(tmp > 0 && tmp <= current_length);
    tmp as usize
}

fn mix(first: &mut Node<(usize, i64)>, length: usize) {
    // println!("{:?}", first);
    // print_numbers(&first);
    for i in 0..length {
        // find node
        let offset = first.find(&|item: &(usize, i64)| item.0 == i, 0);
        // remove node from chain
        let node = first.remove(offset);
        let current_length = length - 1;
        // println!("{:?}", first);
        // insert node at new position
        let move_by = node.item.1;
        let new_offset = determine_offset(offset as i64, move_by, current_length as i64);
        // println!("{:?} {}", node, new_offset);
        first.insert(node.item, new_offset);
        // println!("{:?}", first);
        // print!("{:?}: ", node);
        // print_numbers(&first);
    }
}

fn compute_grove_coordinates(first: &Node<(usize, i64)>, length: usize) -> usize {
    let mut sum = 0;
    for offset in [1000, 2000, 3000] {
        let zero_offset = first.find(&|item: &(usize, i64)| item.1 == 0, 0);
        let offset = (zero_offset + offset) % length;
        sum += first.get(offset).1;
    }
    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_offset() {
        assert_eq!(determine_offset(7, -7 - 1, 7), 6);
        assert_eq!(determine_offset(7, -1, 7), 6);
        assert_eq!(determine_offset(7, 0, 7), 7);
        assert_eq!(determine_offset(7, 1, 7), 1);
        assert_eq!(determine_offset(7, 2, 7), 2);
        assert_eq!(determine_offset(7, 7 + 2, 7), 2);
    }
}
