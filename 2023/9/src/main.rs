use day_9::oasis;

fn main() {
    let input = include_str!("../input.txt");

    let part1 = input
        .lines()
        .map(|l| oasis::History::from(l))
        .map(|h| h.predict_next_value())
        .sum::<isize>();
    println!("part 1: {}", part1);

    let part2 = input
        .lines()
        .map(|l| oasis::History::from(l))
        .map(|h| h.predict_previous_value())
        .sum::<isize>();
    println!("part 2: {}", part2);
}
