use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day-1.input").unwrap();

    let mut puzzle_1_floor = 0;

    for c in input.chars() {
        match c {
            '(' => puzzle_1_floor += 1,
            ')' => puzzle_1_floor -= 1,
            _ => {
                unreachable!("Only characters in input should be ( and )")
            }
        }
    }

    println!("final floor is: {}", puzzle_1_floor);

    let mut puzzle_2_floor = 0;
    for (idx, char) in input.char_indices() {
        match char {
            '(' => puzzle_2_floor += 1,
            ')' => puzzle_2_floor -= 1,
            _ => {
                unreachable!("Only characters in input should be ( and )")
            }
        }

        if puzzle_2_floor < 0 {
            println!("Santa got to the basement on index {}", idx + 1);
            break;
        }
    }
}
