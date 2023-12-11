use day_5::almanac;

fn main() {
    let almanac = almanac::Almanac::from(include_str!("../input.txt"));
    println!("Lowest location number: {}", almanac.find_lowest_location());
}
