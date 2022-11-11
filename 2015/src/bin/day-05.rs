use std::{collections::HashMap, fmt, fs::read_to_string};

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

fn letter_appears_twice_with_no_overlaps(word: &str) -> bool {
    let mut hash_map: HashMap<char, u8> = HashMap::new();
    for char in word.chars() {
        hash_map
            .entry(char)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut output = true;
    let mut counter: u8 = 0;
    for (char, num) in hash_map.iter() {
        if num > &2 {
            let trio = fmt::format(format_args!("{}{}{}", char, char, char));
            if word.contains(&trio) {
                output = false;
                break;
            }
            counter += 1;
        } else if num == &2 {
            counter += 1;
        }
    }
    if counter < 2 {
        output = false;
    }
    output
}

fn letter_repeats_with_one_letter_inbetween(word: &str) -> bool {
    let chars_arr = word.chars().collect::<Vec<_>>();
    let option = chars_arr.windows(3).find(|x| x[0] == x[2]);
    match option {
        Some(_) => true,
        None => false,
    }
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

    println!("total n of nice strings in part 1: {}", counter);

    let mut counter_2: u32 = 0;

    for string in input.lines() {
        if !letter_appears_twice_with_no_overlaps(string) {
            continue;
        }
        if !letter_repeats_with_one_letter_inbetween(string) {
            continue;
        }
        counter_2 += 1;
    }
    println!(
        "{}",
        letter_appears_twice_with_no_overlaps("ztxhjwllrckhakut")
    );

    println!(
        "{}",
        letter_repeats_with_one_letter_inbetween("ztxhjwllrckhakut")
    );

    println!("total n of nice strings in part 2: {}", counter_2);
}

#[cfg(test)]
mod tests {
    use crate::{letter_appears_twice_with_no_overlaps, letter_repeats_with_one_letter_inbetween};
    #[test]
    fn it_works() {
        let result = letter_appears_twice_with_no_overlaps("xyxy");
        assert_eq!(result, true);

        let result_2 = letter_appears_twice_with_no_overlaps("aabcdefgaa");
        assert_eq!(result_2, true);

        let result_3 = letter_appears_twice_with_no_overlaps("aaa");
        assert_eq!(result_3, false);
    }

    #[test]
    fn it_works_2() {
        let result = letter_repeats_with_one_letter_inbetween("xyx");
        assert_eq!(result, true);

        let result_2 = letter_repeats_with_one_letter_inbetween("abcdefeghi");
        assert_eq!(result_2, true);

        let result_3 = letter_repeats_with_one_letter_inbetween("aaa");
        assert_eq!(result_3, true);
    }
}
