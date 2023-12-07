use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq)]
pub(crate) struct Hand {
    cards: [Card; 5],
    pub(crate) bid: usize,
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

        let joker_count = card_count.remove(&Card::Jack).unwrap_or(0);

        if card_count.len() <= 1 {
            return HandType::FiveOfAKind;
        }

        if card_count.values().any(|c| *c + joker_count == 4) {
            return HandType::FourOfAKind;
        }

        // Zero Joker Count Case
        if card_count.values().any(|c| *c == 3) && card_count.values().any(|c| *c == 2) {
            return HandType::FullHouse;
        }
        // One Joker Count Case
        if joker_count == 1 && card_count.values().filter(|c| **c == 2).count() == 2 {
            return HandType::FullHouse;
        }
        // Two Joker Count Case
        if joker_count == 2
            && card_count.values().any(|c| *c == 2)
            && card_count.values().any(|c| *c == 1)
        {
            return HandType::FullHouse;
        }

        if card_count.values().any(|c| *c + joker_count == 3) {
            return HandType::ThreeOfAKind;
        }

        // No Joker Case
        if card_count.values().filter(|c| **c == 2).count() == 2 {
            return HandType::TwoPair;
        }
        //One Joker Case
        if joker_count == 1 && card_count.values().filter(|c| **c == 2).count() == 1 {
            return HandType::TwoPair;
        }

        if card_count.values().any(|c| *c + joker_count == 2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub(crate) struct Input {
    pub(crate) hands: Vec<Hand>,
}

impl Input {
    pub(crate) fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::parse).collect();

        Self { hands }
    }
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
