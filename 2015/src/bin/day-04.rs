use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fmt;

fn lowest_positive_number(input: &String, starts_with: &str) {
    let mut counter: u32 = 0;
    let mut hash = Md5::new();

    loop {
        counter += 1;
        let test_key = fmt::format(format_args!("{}{}", input, counter));

        hash.input_str(&test_key);
        let hash_result = hash.result_str();

        if hash_result.starts_with(starts_with) {
            println!(
                "output for hash starting with {} is: {}",
                starts_with,
                test_key.replace(input, "")
            );
            break;
        }

        hash.reset();
    }
}

fn main() {
    let input = String::from("ckczppom");
    lowest_positive_number(&input, "00000");

    lowest_positive_number(&input, "000000");
}
