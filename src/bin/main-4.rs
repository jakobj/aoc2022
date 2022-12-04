use std::fs;

fn main() {
    let file_path = "inputs/4.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let count = count_pairs_with_overlap(&contents, true);
    println!(
        "There are {} pairs in which one range fully contains the other.",
        count
    );
    let count = count_pairs_with_overlap(&contents, false);
    println!("There are {} pairs with overlap.", count);
}

fn count_pairs_with_overlap(list: &str, count_only_complete_overlap: bool) -> u64 {
    let mut count = 0;
    for l in list.lines() {
        let ranges = l.split(",").collect::<Vec<&str>>();
        let range0 = str_to_numerical_range(ranges[0]);
        let range1 = str_to_numerical_range(ranges[1]);
        if count_only_complete_overlap {
            if has_complete_overlap(&range0, &range1) {
                count += 1;
            }
        } else {
            if has_overlap(&range0, &range1) {
                count += 1;
            }
        }
    }
    count
}

fn str_to_numerical_range(range: &str) -> Vec<u64> {
    range
        .split("-")
        .map(|s| {
            s.to_string()
                .parse()
                .expect("Should be able to convert limits to u64")
        })
        .collect::<Vec<u64>>()
}

fn has_complete_overlap(range0: &Vec<u64>, range1: &Vec<u64>) -> bool {
    if (range0[0] >= range1[0] && range0[1] <= range1[1])
        || (range0[0] <= range1[0] && range0[1] >= range1[1])
    {
        true
    } else {
        false
    }
}

fn has_overlap(range0: &Vec<u64>, range1: &Vec<u64>) -> bool {
    if (range0[1] < range1[0]) || range0[0] > range1[1] {
        false
    } else {
        true
    }
}
