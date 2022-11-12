use std::fs::read_to_string;

#[derive(Debug)]
enum Command {
    On,
    Off,
    Toggle,
}

fn parse_command(instruction_string: &str) -> (Command, Vec<u32>, Vec<u32>) {
    let info_arr: Vec<&str> = instruction_string.rsplit(" ").collect();
    let to: Vec<&str> = info_arr[0].split(',').collect();
    let to_int: Vec<u32> = to.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let from: Vec<&str> = info_arr[2].split(',').collect();
    let from_int: Vec<u32> = from.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let action = info_arr[3];
    let action_enum = match action {
        "on" => Command::On,
        "off" => Command::Off,
        "toggle" => Command::Toggle,
        _ => unreachable!("should not happen"),
    };
    (action_enum, from_int, to_int)
}
fn main() {
    let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

    let input = read_to_string("input/day-6.input").unwrap();

    for instruction in input.lines() {
        let (action, from, to) = parse_command(instruction);
        let mut new_grid = grid.clone();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                let start_x = from[0];
                let start_y = from[1];
                let end_x = to[0];
                let end_y = to[1];
                if row_idx as u32 >= start_x
                    && col_idx as u32 >= start_y
                    && row_idx as u32 <= end_x
                    && col_idx as u32 <= end_y
                {
                    match action {
                        Command::On => new_grid[row_idx][col_idx] = true,
                        Command::Off => new_grid[row_idx][col_idx] = false,
                        Command::Toggle => new_grid[row_idx][col_idx] = !col,
                    };
                }
            }
        }
        grid = new_grid;
    }

    let flatten_grid: Vec<bool> = grid
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    let result: Vec<&bool> = flatten_grid
        .iter()
        .filter(|entry| entry == &&true)
        .collect();

    println!("number of lights on: {}", result.len());
}
