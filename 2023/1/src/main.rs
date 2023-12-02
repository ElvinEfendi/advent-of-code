use std::collections::HashMap;

fn main() {
    let sum = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
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
