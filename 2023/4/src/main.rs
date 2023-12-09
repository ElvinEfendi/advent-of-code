use day_4::card;

fn expand_cards(cards: &Vec<card::Card>, original_cards: &Vec<card::Card>) -> Vec<card::Card> {
    let mut expanded_cards = cards.clone();
    let mut new_cards = Vec::new();

    for card in cards {
        let number_of_matches = card.get_number_of_matches();
        let id = card.get_id() as usize;

        if number_of_matches > 0 {
            for j in id..(id + number_of_matches) {
                new_cards.push(original_cards[j].clone());
            }
        }
    }

    if new_cards.len() == 0 {
        return expanded_cards;
    }

    for card in expand_cards(&new_cards, original_cards) {
        expanded_cards.push(card);
    }

    expanded_cards
}

fn main() {
    let cards = include_str!("../input.txt")
        .lines()
        .map(|s| card::Card::from(s))
        .collect::<Vec<card::Card>>();

    let total_point_values = cards
        .iter()
        .map(|c| c.point_value().unwrap_or(0))
        .sum::<u32>();
    println!("Total point value: {}", total_point_values);

    let expanded_cards = expand_cards(&cards, &cards);
    println!("Number of expanded cards: {}", expanded_cards.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_cards() {
        let card_data = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = card_data
            .lines()
            .map(|s| card::Card::from(s))
            .collect::<Vec<card::Card>>();

        let expanded_cards = expand_cards(&cards, &cards);
        assert_eq!(expanded_cards.len(), 30);
    }
}
