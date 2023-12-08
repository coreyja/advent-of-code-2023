use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq)]
pub(crate) struct Hand<CardType> {
    cards: [CardType; 5],
    pub(crate) bid: usize,
}

impl<CardType> Ord for Hand<CardType>
where
    CardType: Eq + std::hash::Hash + Copy + Ord + std::fmt::Debug,
    Hand<CardType>: Scorable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl<CardType> PartialOrd for Hand<CardType>
where
    Hand<CardType>: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<CardType: From<char> + std::fmt::Debug + Eq + std::hash::Hash + Copy> Hand<CardType> {
    fn parse(input: &str) -> Self {
        let split = input.split_whitespace().collect::<Vec<_>>();
        let cards = split[0];
        let bid = split[1];

        let cards = cards.chars().map(CardType::from).collect::<Vec<_>>();
        let cards = cards.try_into().unwrap();

        let bid = bid.parse().unwrap();

        Self { cards, bid }
    }
}

impl Scorable for Hand<Card> {
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

trait Scorable {
    fn hand_type(&self) -> HandType;
}

impl Scorable for Hand<JokerCard> {
    fn hand_type(&self) -> HandType {
        let mut card_count = HashMap::<JokerCard, usize>::new();

        for c in self.cards {
            card_count.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        let joker_count = card_count.remove(&JokerCard(Card::Jack)).unwrap_or(0);

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

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub(crate) struct JokerCard(Card);

impl Ord for JokerCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 == Card::Jack && other.0 == Card::Jack {
            return std::cmp::Ordering::Equal;
        }
        if self.0 == Card::Jack {
            return std::cmp::Ordering::Less;
        }
        if other.0 == Card::Jack {
            return std::cmp::Ordering::Greater;
        }

        self.0.cmp(&other.0)
    }
}

impl PartialOrd for JokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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

impl From<char> for JokerCard {
    fn from(value: char) -> Self {
        JokerCard(Card::from(value))
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
pub(crate) struct Input<CardType> {
    pub(crate) hands: Vec<Hand<CardType>>,
}

impl<CardType: From<char> + std::fmt::Debug + Eq + std::hash::Hash + Copy> Input<CardType> {
    pub(crate) fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::parse).collect();

        Self { hands }
    }
}

fn part_1(sample_input: &str) -> usize {
    let mut input = Input::<Card>::parse(sample_input);
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

fn part_2(sample_input: &str) -> usize {
    let mut input = Input::<JokerCard>::parse(sample_input);
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

    let sample_part_2_ans = part_2(sample_input);
    dbg!(sample_part_2_ans);

    let my_part_2_ans = part_2(my_input);
    dbg!(my_part_2_ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_of_kinds() {
        let a = Hand::<Card>::parse("33332 1");
        let b = Hand::<Card>::parse("2AAAA 2");

        assert!(a > b);
        assert_eq!(a.hand_type(), HandType::FourOfAKind);
        assert_eq!(b.hand_type(), HandType::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let a = Hand::<Card>::parse("77888 1");
        let b = Hand::<Card>::parse("77788 2");

        assert!(a > b);
        assert_eq!(a.hand_type(), HandType::FullHouse);
        assert_eq!(b.hand_type(), HandType::FullHouse);
    }
}
