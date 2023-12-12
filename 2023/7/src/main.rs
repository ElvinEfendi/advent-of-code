use day_7::camel_cards;

fn main() {
    let input = include_str!("../input.txt");
    let mut camel_cards = camel_cards::CamelCards::from(input);
    println!("Part 1: {}", camel_cards.total_winnings());
}
