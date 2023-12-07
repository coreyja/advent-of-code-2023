use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}
impl Hand {
    fn parse(input: &str) -> Self {
        let split = input.split_whitespace().collect::<Vec<_>>();
        let cards = split[0];
        let bid = split[1];

        let cards = cards.chars().map(Card::from).collect::<Vec<_>>();
        let cards = cards.try_into().unwrap();

        let bid = bid.parse().unwrap();

        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut card_count = HashMap::<Card, usize>::new();

        for c in self.cards {
            card_count.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        if card_count.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if card_count.values().any(|c| *c == 4) {
            return HandType::FourOfAKind;
        }

        if card_count.values().any(|c| *c == 3) && card_count.values().any(|c| *c == 2) {
            return HandType::FullHouse;
        }

        if card_count.values().any(|c| *c == 3) {
            return HandType::ThreeOfAKind;
        }

        if card_count.values().filter(|c| **c == 2).count() == 2 {
            return HandType::TwoPair;
        }

        if card_count.values().any(|c| *c == 2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Input {
    hands: Vec<Hand>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::parse).collect();

        Self { hands }
    }
}

fn part_1(sample_input: &str) -> usize {
    let mut input = Input::parse(sample_input);
    input.hands.sort();

    input
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let multiplier = i + 1;

            h.bid * multiplier
        })
        .sum()
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_of_kinds() {
        let a = Hand::parse("33332 1");
        let b = Hand::parse("2AAAA 2");

        assert!(a > b);
        assert_eq!(a.hand_type(), HandType::FourOfAKind);
        assert_eq!(b.hand_type(), HandType::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let a = Hand::parse("77888 1");
        let b = Hand::parse("77788 2");

        assert!(a > b);
        assert_eq!(a.hand_type(), HandType::FullHouse);
        assert_eq!(b.hand_type(), HandType::FullHouse);
    }
}
