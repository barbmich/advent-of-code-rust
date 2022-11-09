use std::{collections::HashMap, fs::read_to_string};

// I'm aware it's not necessary to convert moves to enum, just trying to practice
#[derive(Debug)]
enum Movement {
    North,
    East,
    South,
    West,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

fn update_current_position(position: Position, movement: &Movement) -> Position {
    match movement {
        Movement::North => Position {
            x: position.x,
            y: position.y + 1,
        },
        Movement::South => Position {
            x: position.x,
            y: position.y - 1,
        },
        Movement::East => Position {
            x: position.x + 1,
            y: position.y,
        },
        Movement::West => Position {
            x: position.x - 1,
            y: position.y,
        },
    }
}

fn update_houses_visited(
    mut hash_map: HashMap<Position, i32>,
    position: Position,
) -> HashMap<Position, i32> {
    let entry = hash_map.get(&position);
    match entry {
        Some(count) => hash_map.insert(position, count + 1),
        None => hash_map.insert(position, 1),
    };
    hash_map
}

fn main() {
    let input = read_to_string("input/day-3.input").unwrap();

    let mut arr_of_movements = vec![];

    for char in input.chars() {
        let movement = match char {
            '^' => Movement::North,
            'v' => Movement::South,
            '>' => Movement::East,
            '<' => Movement::West,
            _ => unreachable!("Only chars present are the ones above."),
        };
        arr_of_movements.push(movement);
    }

    let mut current_position = Position { x: 0, y: 0 };

    let mut houses_visited_pt1 = HashMap::new();
    houses_visited_pt1.insert(current_position, 1);

    for movement in arr_of_movements.iter() {
        current_position = update_current_position(current_position, movement);
        houses_visited_pt1 = update_houses_visited(houses_visited_pt1, current_position)
    }

    println!(
        "the number of houses visited in pt1 is: {}",
        houses_visited_pt1.len()
    );

    let mut santa_position = Position { x: 0, y: 0 };
    let mut robot_position = Position { x: 0, y: 0 };

    let mut houses_visited_pt2 = HashMap::new();
    houses_visited_pt2.insert(santa_position, 2);

    for (idx, movement) in arr_of_movements.iter().enumerate() {
        if idx % 2 == 0 {
            santa_position = update_current_position(santa_position, movement);
            houses_visited_pt2 = update_houses_visited(houses_visited_pt2, santa_position);
        } else {
            robot_position = update_current_position(robot_position, movement);
            houses_visited_pt2 = update_houses_visited(houses_visited_pt2, robot_position);
        }
    }

    println!(
        "the number of houses visited in pt2 is: {}",
        houses_visited_pt2.len()
    );
}
