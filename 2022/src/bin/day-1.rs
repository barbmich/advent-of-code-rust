use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day-1.input").unwrap();

    let str_vector: Vec<&str> = input.split("\n\n").collect();
    let mut calories_vector: Vec<i32> = vec![];
    str_vector.iter().for_each(|elf| {
        calories_vector.push(
            elf.split('\n')
                .map(|item| item.parse::<i32>().unwrap())
                .sum(),
        )
    });

    calories_vector.sort();
    calories_vector.reverse();

    println!("highest amount of calories carried: {}", calories_vector[0]);

    println!(
        "total of highest 3 amounts of calories carried: {}",
        calories_vector.iter().take(3).sum::<i32>()
    );

    let test = '\n';
    println!("{:?}", test.to_string().as_bytes());
}
