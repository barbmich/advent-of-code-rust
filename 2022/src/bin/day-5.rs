use std::fs::read_to_string;

#[derive(Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}
fn get_stacks() -> Vec<Vec<char>> {
    vec![
        vec!['N', 'R', 'G', 'P'],
        vec!['J', 'T', 'B', 'L', 'F', 'G', 'D', 'C'],
        vec!['M', 'S', 'V'],
        vec!['L', 'S', 'R', 'C', 'Z', 'P'],
        vec!['P', 'S', 'L', 'V', 'C', 'W', 'D', 'Q'],
        vec!['C', 'T', 'N', 'W', 'D', 'M', 'S'],
        vec!['H', 'D', 'G', 'W', 'P'],
        vec!['Z', 'L', 'P', 'H', 'S', 'C', 'M', 'V'],
        vec!['R', 'P', 'F', 'L', 'W', 'G', 'Z'],
    ]
}

fn main() {
    let input = read_to_string("input/day-5.input").unwrap();
    let moves: Vec<_> = input.split("\n\n").collect::<Vec<_>>()[1]
        .lines()
        .map(|line| {
            let parsed_string: Vec<_> = line
                .split(' ')
                .filter_map(|str| str.parse::<usize>().ok())
                .collect();
            Move {
                qty: parsed_string[0],
                from: parsed_string[1],
                to: parsed_string[2],
            }
        })
        .collect();

    let mut part_1_stacks = get_stacks();

    for m in moves.iter() {
        let final_length = part_1_stacks[m.from - 1].len().saturating_sub(m.qty);
        let tail = part_1_stacks[m.from - 1].split_off(final_length);
        part_1_stacks[m.to - 1].extend(tail);
    }

    let part_1 = part_1_stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    println!(
        "part 1 - the list of crates on top at the end is: {}",
        part_1
    );

    let mut part_2_stacks = get_stacks();

    for m in moves.iter() {
        let final_length = part_2_stacks[m.from - 1].len().saturating_sub(m.qty);
        let mut tail = part_2_stacks[m.from - 1].split_off(final_length);
        tail.reverse();
        part_2_stacks[m.to - 1].extend(tail);
    }

    let part_2 = part_2_stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    println!(
        "part 2 - the list of crates on top at the end is: {}",
        part_2
    );
}
