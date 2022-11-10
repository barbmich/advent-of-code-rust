use std::fs::read_to_string;

fn forbidden_couple_is_included(word: &str) -> bool {
    let forbidden_couples = vec!["ab", "cd", "pq", "xy"];
    let mut output = false;
    for couple in forbidden_couples.iter() {
        if word.contains(couple) {
            output = true;
            break;
        }
    }
    output
}

fn letter_appears_twice_in_a_row(word: &str) -> bool {
    let chars_arr = word.chars().collect::<Vec<_>>();
    let option = chars_arr.windows(2).find(|x| x[0] == x[1]);
    match option {
        Some(_) => true,
        None => false,
    }
}

fn has_at_least_three_vowels(word: &str) -> bool {
    let vowels_arr = vec!['a', 'e', 'i', 'o', 'u'];
    let mut counter: u32 = 0;
    let chars_arr = word.chars().collect::<Vec<_>>();
    for char in chars_arr.iter() {
        if vowels_arr.contains(char) {
            counter += 1;
        }
    }
    counter > 2
}

fn main() {
    let input = read_to_string("input/day-5.input").unwrap();
    let mut counter: u32 = 0;
    for string in input.lines() {
        // listed separately to make it easy to read
        if forbidden_couple_is_included(string) {
            continue;
        }
        if !letter_appears_twice_in_a_row(string) {
            continue;
        }
        if !has_at_least_three_vowels(string) {
            continue;
        }
        counter += 1;
    }

    println!("total of nice strings: {}", counter);
}
