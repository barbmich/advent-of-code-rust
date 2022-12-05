use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum Operator {
    Not,
    And,
    Or,
    Lshift,
    Rshift,
}

#[derive(Debug)]
struct Row<'a> {
    operator: Option<Operator>,
    values: Vec<&'a str>,
    output: String,
}

fn parse_line(line: &str) -> Row {
    let split_line: Vec<&str> = line.split("->").collect();
    let first_part_as_vec: Vec<&str> = split_line[0].split(' ').map(|x| x.trim()).collect();
    let operator: Option<Operator>;
    let mut values = vec![];
    if first_part_as_vec.contains(&"NOT") {
        operator = Some(Operator::Not);
        values.push(first_part_as_vec[1]);
    } else if first_part_as_vec.contains(&"AND") {
        operator = Some(Operator::And);
        values.push(first_part_as_vec[0]);
        values.push(first_part_as_vec[2]);
    } else if first_part_as_vec.contains(&"OR") {
        operator = Some(Operator::Or);
        values.push(first_part_as_vec[0]);
        values.push(first_part_as_vec[2]);
    } else if first_part_as_vec.contains(&"LSHIFT") {
        operator = Some(Operator::Lshift);
        values.push(first_part_as_vec[0]);
        values.push(first_part_as_vec[2]);
    } else if first_part_as_vec.contains(&"RSHIFT") {
        operator = Some(Operator::Rshift);
        values.push(first_part_as_vec[0]);
        values.push(first_part_as_vec[2]);
    } else {
        operator = None;
        values.push(first_part_as_vec[0]);
    };

    Row {
        operator,
        values,
        output: split_line[1].trim().to_string(),
    }
}

fn main() {
    let input = read_to_string("input/day-7.input").unwrap();
    let parsed_input: Vec<Row> = input.lines().map(|x| parse_line(x)).collect();
    let mut output: Option<i32> = None;
    let mut hash_map: HashMap<String, Option<i32>> = HashMap::new();

    while output.is_none() {
        for row in parsed_input.iter() {
            if row.operator.is_none() {
                let value = row.values[0];
                match value.parse::<i32>() {
                    Ok(num) => {
                        hash_map.insert(row.output.to_string(), Some(num));
                    }
                    Err(_) => {
                        if let Some(num) = hash_map.get(value) {
                            hash_map.insert(row.output.to_string(), *num);
                        }
                    }
                }
            } else {
                match &row.operator {
                    Some(operator) => match operator {
                        Operator::Not => {
                            if let Ok(num) = row.values[0].parse::<i32>() {
                                hash_map.insert(row.output.to_string(), Some(!num));
                            }
                        }
                        Operator::And => {
                            let value_1 = row.values[0].parse::<i32>();
                            let value_2 = row.values[1].parse::<i32>();
                            if value_1.is_ok() && value_2.is_ok() {
                                let computation = value_1 & value_2;
                                hash_map.insert(row.output.to_string(), Some());
                            }
                        }
                        Operator::Or => {}
                        Operator::Lshift => {}
                        Operator::Rshift => {}
                    },
                    None => {}
                }
            }
        }
        output = Some(4);
    }
}
