use std::collections::HashMap;

fn main() {
    let digit_words: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ].iter().cloned().collect();

    let sum = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.to_string();

            for (word, digit) in digit_words.iter() {
                line = line.replace(word, format!("{}{}{}", word, digit, word).as_str());
            }

            let digits = line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let first = digits[0];
            let last = digits[digits.len() - 1];

            first * 10 + last
        })
        .sum::<u32>();

    println!("{}", sum);
}
