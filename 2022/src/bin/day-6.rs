use std::{collections::HashSet, fs::read_to_string};

fn parse(input: &str) -> Vec<char> {
    read_to_string(input)
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .chars()
        .collect()
}

fn solve(input: &[char], length: usize) -> Option<usize> {
    for (i, window) in input.windows(length).enumerate() {
        let mut hs = HashSet::new();
        window.iter().for_each(|c| {
            hs.insert(c);
        });
        if hs.len() == length {
            return Some(i + length);
        }
    }
    None
}

fn main() {
    let input = parse("input/day-6.input");
    let part_1 = solve(&input, 4);

    println!("{}", part_1.unwrap());

    let part_2 = solve(&input, 14);

    println!("{}", part_2.unwrap());
}

#[test]
fn test_solve() {
    read_to_string("example/day-6.example")
        .unwrap()
        .lines()
        .for_each(|l| {
            let arr = l.split(',').collect::<Vec<&str>>();
            let input = arr[0].chars().collect::<Vec<char>>();
            let expected_pt_1: usize = arr[1].parse().unwrap();
            let expected_pt_2: usize = arr[2].parse().unwrap();

            assert_eq!(expected_pt_1, solve(&input, 4).unwrap());
            assert_eq!(expected_pt_2, solve(&input, 14).unwrap())
        });
}
