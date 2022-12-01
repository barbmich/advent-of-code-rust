use std::{fs::read_to_string, vec};

fn main() {
    let input = read_to_string("input/day-1.input").unwrap();

    let mut highest_value = 0;
    let mut current_elf = 0;
    input.lines().for_each(|line| match line.parse::<i32>() {
        Ok(value) => {
            current_elf += value;
        }
        Err(..) => {
            highest_value = if highest_value > current_elf {
                highest_value
            } else {
                current_elf
            };
            current_elf = 0;
        }
    });

    println!("{}", highest_value);

    // let array: Vec<&str> = input.split("\n\n").collect();
    // let mut array_of_items: Vec<Vec<i32>> = vec![];
    // array.iter().for_each(|elf| {
    //     array_of_items.push(
    //         elf.split("\n")
    //             .map(|item| item.parse::<i32>().unwrap())
    //             .collect(),
    //     )
    // });
    // array_of_items.iter().map(f)
    // println!("{:?}", array_of_items);
}
