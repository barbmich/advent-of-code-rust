use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Command {
    On,
    Off,
    Toggle,
}

fn coord_str_to_int_arr(value: &str) -> Vec<u16> {
    value
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

fn parse_command(instruction_string: &str) -> (Command, Vec<u16>, Vec<u16>) {
    let info_arr: Vec<&str> = instruction_string.rsplit(" ").collect();

    let to = coord_str_to_int_arr(info_arr[0]);
    let from = coord_str_to_int_arr(info_arr[2]);
    let action = match info_arr[3] {
        "on" => Command::On,
        "off" => Command::Off,
        "toggle" => Command::Toggle,
        _ => unreachable!("should not happen"),
    };

    (action, from, to)
}

trait Actions<T> {
    fn on_on(&self, old_value: &T) -> T;
    fn on_off(&self, old_value: &T) -> T;
    fn on_toggle(&self, old_value: &T) -> T;
}

struct Operations;

impl Actions<bool> for Operations {
    fn on_on(&self, _old_value: &bool) -> bool {
        true
    }
    fn on_off(&self, _old_value: &bool) -> bool {
        false
    }
    fn on_toggle(&self, old_value: &bool) -> bool {
        !old_value
    }
}

impl Actions<u16> for Operations {
    fn on_on(&self, old_value: &u16) -> u16 {
        old_value + 1
    }
    fn on_off(&self, old_value: &u16) -> u16 {
        if old_value > &0 {
            old_value - 1
        } else {
            0
        }
    }
    fn on_toggle(&self, old_value: &u16) -> u16 {
        old_value + 2
    }
}

fn process_data<T: Clone + Copy>(
    input: &String,
    mut grid: Vec<Vec<T>>,
    operations: &impl Actions<T>,
) -> Vec<Vec<T>> {
    for instruction in input.lines() {
        let (action, from, to) = parse_command(instruction);
        let mut new_grid = grid.clone();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                let start_x = from[0];
                let start_y = from[1];
                let end_x = to[0];
                let end_y = to[1];
                if row_idx as u16 >= start_x
                    && col_idx as u16 >= start_y
                    && row_idx as u16 <= end_x
                    && col_idx as u16 <= end_y
                {
                    match action {
                        Command::On => new_grid[row_idx][col_idx] = operations.on_on(col),
                        Command::Off => new_grid[row_idx][col_idx] = operations.on_off(col),
                        Command::Toggle => new_grid[row_idx][col_idx] = operations.on_toggle(col),
                    };
                }
            }
        }
        grid = new_grid;
    }

    grid
}

fn main() {
    let input = read_to_string("input/day-6.input").unwrap();
    let operations = Operations;

    let mut boolean_grid = vec![vec![false; 1000]; 1000];
    boolean_grid = process_data(&input, boolean_grid, &operations);

    let flatten_grid: Vec<bool> = boolean_grid
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    let result: Vec<&bool> = flatten_grid
        .iter()
        .filter(|entry| entry == &&true)
        .collect();

    println!("number of lights on: {}", result.len());

    let mut intensity_grid = vec![vec![0; 1000]; 1000];
    intensity_grid = process_data(&input, intensity_grid, &operations);

    let flatten_grid: Vec<u16> = intensity_grid
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    let filtered_grid: Vec<&u16> = flatten_grid
        .iter()
        .filter(|entry| **entry > 0)
        .to_owned()
        .collect();

    let result: u32 = filtered_grid.iter().map(|x| u32::from(**x)).sum();
    println!("intensity of lights on: {}", result);
}
