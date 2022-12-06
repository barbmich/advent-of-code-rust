use std::fs::read_to_string;

#[derive(Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}
fn get_stacks() -> [Vec<char>; 9] {
    return [
        vec!['N', 'R', 'G', 'P'],
        vec!['J', 'T', 'B', 'L', 'F', 'G', 'D', 'C'],
        vec!['M', 'S', 'V'],
        vec!['L', 'S', 'R', 'C', 'Z', 'P'],
        vec!['P', 'S', 'L', 'V', 'C', 'W', 'D', 'Q'],
        vec!['C', 'T', 'N', 'W', 'D', 'M', 'S'],
        vec!['H', 'D', 'G', 'W', 'P'],
        vec!['Z', 'L', 'P', 'H', 'S', 'C', 'M', 'V'],
        vec!['R', 'P', 'F', 'L', 'W', 'G', 'Z'],
    ];
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
        for _ in 0..m.qty {
            let s = part_1_stacks[m.from - 1].pop();
            part_1_stacks[m.to - 1].push(s.unwrap());
        }
    }

    let mut part_1 = String::new();
    for stack in part_1_stacks.iter_mut() {
        part_1.push(stack.pop().unwrap());
    }

    println!("the list of crates on top of the stacks is: {}", part_1);

    let mut part_2_stacks = get_stacks();

    for m in moves.iter() {
        let (remaining_crates, moving_crates) =
            part_2_stacks[m.from - 1].split_at(part_2_stacks[m.from - 1].len() - m.qty);
        let moved_crates = moving_crates.clone();
        part_2_stacks[m.from - 1] = remaining_crates.to_vec();
        part_2_stacks[m.to - 1].append(&mut moved_crates.to_vec());
    }
}
