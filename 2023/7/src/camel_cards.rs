use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Debug)]
struct Card(u8);

impl Card {
    fn new(symbol: char) -> Card {
        let value = match symbol {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => symbol.to_digit(10).unwrap() as u8,
        };

        Card(value)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Debug)]
struct Hand {
    cards: [Card; 5],
    category: Category,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.category.partial_cmp(&other.category) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Hand {
        let category = Self::determine_category(&cards);

        Hand {
            cards,
            category,
            bid: bid,
        }
    }

    fn determine_category(cards: &[Card; 5]) -> Category {
        let mut stats: Vec<u8> = cards
            .iter()
            .fold(HashMap::new(), |mut stats, card| {
                let count = stats.entry(card).or_insert(0);
                *count += 1;
                stats
            })
            .values()
            .map(|&count| count)
            .collect();

        while stats.len() < 5 {
            stats.push(0);
        }

        let stats: (u8, u8, u8, u8, u8) = stats
            .into_iter()
            .sorted()
            .collect_tuple()
            .unwrap();

        match stats {
            (1, 1, 1, 1, 1) => Category::HighCard,
            (0, 1, 1, 1, 2) => Category::OnePair,
            (0, 0, 1, 2, 2) => Category::TwoPair,
            (0, 0, 1, 1, 3) => Category::ThreeOfAKind,
            (0, 0, 0, 2, 3) => Category::FullHouse,
            (0, 0, 0, 1, 4) => Category::FourOfAKind,
            (0, 0, 0, 0, 5) => Category::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

pub struct CamelCards {
    hands: Vec<Hand>,
}

impl From<&str> for CamelCards {
    fn from(input: &str) -> Self {
        let mut hands = Vec::new();

        for line in input.lines() {
            let (hand_str, bid_str) = line.split_whitespace().collect_tuple().unwrap();

            let cards: [Card; 5] = hand_str
                .chars()
                .map(|symbol| Card::new(symbol))
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bid = bid_str.parse::<u32>().unwrap();

            hands.push(Hand::new(cards, bid));
        }

        CamelCards { hands }
    }
}

impl CamelCards {
    fn rank_hands(&mut self) {
        self.hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn total_winnings(&mut self) -> u32 {
        self.rank_hands();

        let mut total_winnings = 0;
        let mut multiplier = 1;

        for hand in self.hands.iter() {
            total_winnings += hand.bid * multiplier;
            multiplier += 1;
        }

        total_winnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new('T');
        assert_eq!(card, Card(10));

        let card = Card::new('J');
        assert_eq!(card, Card(11));

        let card = Card::new('A');
        assert_eq!(card, Card(14));

        let card = Card::new('2');
        assert_eq!(card, Card(2));
    }

    #[test]
    fn test_category_compare() {
        assert!(Category::HighCard < Category::OnePair);
        assert!(Category::OnePair < Category::TwoPair);
        assert!(Category::TwoPair < Category::ThreeOfAKind);
        assert!(Category::ThreeOfAKind < Category::FullHouse);
        assert!(Category::FullHouse < Category::FourOfAKind);
        assert!(Category::FourOfAKind < Category::FiveOfAKind);
    }

    #[test]
    fn test_hand_new() {
        let cards = [
            Card::new('2'),
            Card::new('3'),
            Card::new('4'),
            Card::new('5'),
            Card::new('6'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::HighCard, hand.category);

        let cards = [
            Card::new('A'),
            Card::new('2'),
            Card::new('3'),
            Card::new('A'),
            Card::new('4'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::OnePair, hand.category);

        let cards = [
            Card::new('2'),
            Card::new('3'),
            Card::new('4'),
            Card::new('3'),
            Card::new('2'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::TwoPair, hand.category);

        let cards = [
            Card::new('T'),
            Card::new('T'),
            Card::new('T'),
            Card::new('9'),
            Card::new('8'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::ThreeOfAKind, hand.category);

        let cards = [
            Card::new('2'),
            Card::new('3'),
            Card::new('3'),
            Card::new('3'),
            Card::new('2'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::FullHouse, hand.category);

        let cards = [
            Card::new('A'),
            Card::new('A'),
            Card::new('8'),
            Card::new('A'),
            Card::new('A'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::FourOfAKind, hand.category);

        let cards = [
            Card::new('A'),
            Card::new('A'),
            Card::new('A'),
            Card::new('A'),
            Card::new('A'),
        ];
        let hand = Hand::new(cards, 1);
        assert_eq!(Category::FiveOfAKind, hand.category);
    }

    #[test]
    fn test_hand_compare_different_categories() {
        let lower_hand = Hand::new(
            [
                Card::new('2'),
                Card::new('3'),
                Card::new('4'),
                Card::new('5'),
                Card::new('6'),
            ],
            50,
        );
        let higher_hand = Hand::new(
            [
                Card::new('A'),
                Card::new('2'),
                Card::new('3'),
                Card::new('A'),
                Card::new('4'),
            ],
            50,
        );
        assert!(lower_hand < higher_hand);
    }

    #[test]
    fn test_hand_compare_same_categories() {
        let lower_hand = Hand::new(
            [
                Card::new('2'),
                Card::new('3'),
                Card::new('4'),
                Card::new('5'),
                Card::new('6'),
            ],
            50,
        );
        let higher_hand = Hand::new(
            [
                Card::new('2'),
                Card::new('3'),
                Card::new('4'),
                Card::new('5'),
                Card::new('7'),
            ],
            50,
        );
        assert!(lower_hand < higher_hand);
    }

    #[test]
    fn test_camel_cards_from() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let camel_cards = CamelCards::from(input);

        assert_eq!(camel_cards.hands.len(), 5);
        assert_eq!(camel_cards.hands[0].cards, [Card(3), Card(2), Card(10), Card(3), Card(13)]);
        assert_eq!(camel_cards.hands[0].bid, 765);
    }

    #[test]
    fn test_camel_cards_total_winnings() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut camel_cards = CamelCards::from(input);

        assert_eq!(camel_cards.total_winnings(), 6440);
    }
}
