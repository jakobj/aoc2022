use std::{cmp::Ordering, fs};

fn main() {
    let filename = "inputs/13.txt";
    let content = fs::read_to_string(filename).expect("Should be able to read file");
    let packet_pairs = parse_all_packet_pairs(&content);
    let in_order = packet_pairs
        .clone()
        .into_iter()
        .map(|pp| compare(&pp.0, &pp.1).unwrap())
        .collect::<Vec<bool>>();
    let sum_of_indices = in_order
        .into_iter()
        .enumerate()
        .map(|(i, o)| if o { i + 1 } else { 0 })
        .sum::<usize>();
    println!(
        "The sum of indices of correctly ordered pairs is {}.",
        sum_of_indices
    );

    let mut all_packets = Vec::new();
    for pp in packet_pairs {
        all_packets.push(pp.0);
        all_packets.push(pp.1);
    }
    let first_divider = ListEntry::List(vec![ListEntry::List(vec![ListEntry::Number(2)])]);
    all_packets.push(first_divider);
    let second_divider = ListEntry::List(vec![ListEntry::List(vec![ListEntry::Number(6)])]);
    all_packets.push(second_divider);
    all_packets.sort_by(|p0, p1| {
        let cmp = compare(p0, p1);
        match cmp {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => panic!("Could not compare packets"),
        }
    });
    let mut decoder_key = 1;
    for (i, p) in all_packets.into_iter().enumerate() {
        // complicated pattern matching to find the divider packets :(
        if let ListEntry::List(l0) = p {
            if l0.len() == 1 {
                if let ListEntry::List(l1) = &l0[0] {
                    if l1.len() == 1 {
                        if let ListEntry::Number(n) = &l1[0] {
                            if *n == 2 || *n == 6 {
                                decoder_key *= i + 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("The decoder key is {}.", decoder_key);
}

#[derive(Clone, Debug)]
enum ListEntry {
    List(Vec<ListEntry>),
    Number(usize),
    None,
}

fn parse_all_packet_pairs(content: &str) -> Vec<(ListEntry, ListEntry)> {
    let mut packets = Vec::new();
    let mut packet_pair = (ListEntry::None, ListEntry::None);
    let mut i = 0;
    for l in content.lines() {
        if l.len() == 0 {
            packets.push(packet_pair.clone());
            continue;
        }
        if i % 2 == 0 {
            packet_pair.0 = parse_single_packet(l);
            i += 1;
        } else {
            packet_pair.1 = parse_single_packet(l);
            i = 0;
        }
    }
    packets.push(packet_pair.clone());
    packets
}

fn parse_single_packet(l: &str) -> ListEntry {
    let l = remove_outer_parentheses(l);
    let mut list = Vec::new();
    for e in split_at_fixed_depth(&l) {
        let v;
        if e.len() == 0 {
            v = ListEntry::List(vec![]);
        } else if e.chars().next().unwrap() == '[' {
            v = parse_single_packet(&e);
        } else {
            v = ListEntry::Number(e.parse().expect("Should be able to parse entry to usize"));
        }
        list.push(v);
    }
    ListEntry::List(list)
}

fn remove_outer_parentheses(e: &str) -> String {
    e.chars().skip(1).take(e.len() - 2).collect::<String>()
}

fn split_at_fixed_depth(l: &str) -> Vec<String> {
    let chars = l.chars().collect::<Vec<char>>();
    let mut s = Vec::new();
    let mut offset = 0;
    let mut depth = 0;
    for i in 0..l.len() {
        if chars[i] == '[' {
            depth += 1;
        }
        if chars[i] == ']' {
            depth -= 1;
        }
        if depth == 0 && chars[i] == ',' {
            s.push(l[offset..i].to_string());
            offset = i + 1;
        }
    }
    s.push(l[offset..].to_string());
    s
}

fn compare(packet0: &ListEntry, packet1: &ListEntry) -> Option<bool> {
    match (packet0, packet1) {
        (ListEntry::Number(n0), ListEntry::Number(n1)) => {
            if n0 < n1 {
                return Some(true);
            } else if n0 > n1 {
                return Some(false);
            } else {
                return None;
            }
        }
        (ListEntry::List(l0), ListEntry::List(l1)) => {
            let mut l0_iter = l0.into_iter();
            let mut l1_iter = l1.into_iter();
            loop {
                let i0 = l0_iter.next();
                let i1 = l1_iter.next();
                match (i0, i1) {
                    (Some(i0), Some(i1)) => {
                        let cmp = compare(i0, i1);
                        match cmp {
                            Some(cmp) => return Some(cmp),
                            None => continue,
                        }
                    }
                    (None, Some(_i1)) => return Some(true),
                    (Some(_i0), None) => return Some(false),
                    (None, None) => return None,
                }
            }
        }
        (ListEntry::Number(n0), ListEntry::List(_l1)) => {
            compare(&ListEntry::List(vec![ListEntry::Number(*n0)]), packet1)
        }
        (ListEntry::List(_l0), ListEntry::Number(n1)) => {
            compare(packet0, &ListEntry::List(vec![ListEntry::Number(*n1)]))
        }
        _ => panic!("Unknown packet"),
    }
}
