use std::fs::read_to_string;

const DIFFERENCE_FROM_UPPERCASE_BYTE_VALUE: u8 = 38;
const DIFFERENCE_FROM_LOWERCASE_BYTE_VALUE: u8 = 96;

fn get_value(num: &u8) -> u8 {
    match num {
        65..=90 => num - DIFFERENCE_FROM_UPPERCASE_BYTE_VALUE,
        97..=122 => num - DIFFERENCE_FROM_LOWERCASE_BYTE_VALUE,
        _ => unreachable!("This shouldn't happen"),
    }
}

fn get_u8_of_item(line: &str) -> Option<u8> {
    let items_nr = line.len();
    let mut output: Option<u8> = None;
    let (compartment_1, compartment_2) = line.split_at(items_nr / 2);
    compartment_1.chars().for_each(|c| {
        if compartment_2.contains(c) {
            let u8 = c.to_string().as_bytes()[0];
            output = Some(u8 as u8);
        }
    });
    output
}

fn get_u8_of_priority(line1: &str, line2: &str, line3: &str) -> Option<u8> {
    let mut output = None;
    line1.chars().for_each(|c| {
        if line2.contains(c) && line3.contains(c) {
            let u8 = c.to_string().as_bytes()[0];
            output = Some(u8 as u8);
        }
    });
    output
}

fn main() {
    let input = read_to_string("input/day-3.input").unwrap();
    let mut items_needed: Vec<u8> = vec![];

    for backpack in input.lines() {
        items_needed.push(get_u8_of_item(backpack).unwrap());
    }

    let part_1: u32 = items_needed
        .iter()
        .fold(0, |acc, num| get_value(num) as u32 + acc);
    println!(
        "part 1 - Sum of all items appearing in both compartments: {}",
        part_1
    );

    let mut priorities = vec![];
    let mut lines = input.lines();

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let priority = get_u8_of_priority(line1, line2, line3).unwrap();
        priorities.push(priority);
    }

    let part_2: u32 = priorities
        .iter()
        .fold(0, |acc, num| get_value(num) as u32 + acc);

    println!("part 2 - Sum of all priorities: {}", part_2);
}

#[test]
fn get_u8_of_item_tests() {
    fn helper(c: char) -> Option<u8> {
        Some(c.to_string().as_bytes()[0] as u8)
    }
    let a = get_u8_of_item("vJrwpWtwJgWrhcsFMMfFFhFp");
    assert_eq!(a, helper('p'));
    let b = get_u8_of_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(b, helper('L'));
    let c = get_u8_of_item("PmmdzqPrVvPwwTWBwg");
    assert_eq!(c, helper('P'));
    let d = get_u8_of_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
    assert_eq!(d, helper('v'));
    let e = get_u8_of_item("ttgJtRGJQctTZtZT");
    assert_eq!(e, helper('t'));
    let f = get_u8_of_item("CrZsJsPPZsGzwwsLwLmpwMDw");
    assert_eq!(f, helper('s'));
    let last_char = get_u8_of_item("aqCvdC");
    assert_eq!(last_char, helper('C'));
}

#[allow(non_snake_case)]
#[test]
fn get_value_tests() {
    let a = get_value(&97u8);
    assert_eq!(a, 1);
    let z = get_value(&122u8);
    assert_eq!(z, 26);
    let A = get_value(&65u8);
    assert_eq!(A, 27);
    let Z = get_value(&90u8);
    assert_eq!(Z, 52);
}

#[test]
fn get_u8_of_priority_tests() {
    fn helper(c: char) -> Option<u8> {
        Some(c.to_string().as_bytes()[0] as u8)
    }

    let a = get_u8_of_priority(
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    );
    assert_eq!(a, helper('r'));
    let b = get_u8_of_priority(
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    assert_eq!(b, helper('Z'));
}
