use std::collections::HashSet;

pub struct Card {
    name: String,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let (name, all_numbers_str) = s.split_once(": ").unwrap();
        let (winning_numbers_str, numbers_str) = all_numbers_str.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let numbers = numbers_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

        Card {
            name: name.to_string(),
            winning_numbers,
            numbers,
        }
    }
}

impl Card {
    pub fn point_value(&self) -> Option<u32> {
        let number_of_matches = self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        if number_of_matches == 0 {
            return None;
        }

        Some(2u32.pow((number_of_matches - 1) as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_str() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(card.name, "Card 1");
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17].into_iter().collect());
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_card_point_value() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.point_value(), Some(8));

        let card = Card::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(card.point_value(), Some(2));

        let card = Card::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(card.point_value(), Some(1));

        let card = Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(card.point_value(), None);
    }
}
