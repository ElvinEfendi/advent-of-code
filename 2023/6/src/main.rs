use itertools::Itertools;
use day_6::race;

fn main() {
    let (times_str, distances_str): (&str, &str) = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_once(":").unwrap().1.trim()
        })
        .collect_tuple()
        .unwrap();

    let result: u32 = times_str
        .split_whitespace()
        .zip(distances_str.split_whitespace())
        .map(|(time_str, distance_str)| {
            let max_time = time_str.parse::<u64>().unwrap();
            let best_distance = distance_str.parse::<u64>().unwrap();

            let race = race::Race::new(max_time, best_distance);
            race.number_of_beating_held_times()
        })
        .product();
    println!("{}", result);
}
