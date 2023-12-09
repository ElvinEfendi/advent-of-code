use day_4::card;

fn main() {
    let total_point_values = include_str!("../input.txt")
        .lines()
        .map(|s| card::Card::from(s).point_value())
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .sum::<u32>();
    println!("Total point value: {}", total_point_values);
}
