use std::cmp::Ordering;
use std::cmp::Reverse;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| Hand::from(line))
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.iter().enumerate().for_each(|(idx, hand)| {
        println!(
            "{:4} * {:3} = {:6} => {:?}",
            idx + 1,
            hand.bid,
            hand.bid * (idx + 1),
            hand
        )
    });
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
pub enum Card {
    Joker,
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

#[derive(PartialEq, Eq, Debug)]
pub enum HandType {
    HighCard(Card, Vec<Card>),
    OnePair(Card, Vec<Card>),
    TwoPair(Card, Card, Vec<Card>),
    ThreeOfAKind(Card, Vec<Card>),
    FullHouse(Card, Card),
    FourOfAKind(Card, Vec<Card>),
    FiveOfAKind(Card),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    hand: HandType,
    cards: Vec<Card>,
    pub bid: usize,
}

impl From<Vec<Card>> for HandType {
    fn from(cards: Vec<Card>) -> HandType {
        let mut counts = [0; 13];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }
        let mut singles = vec![];
        let mut pairs = vec![];
        let mut three: Option<Card> = None;
        let mut four: Option<Card> = None;
        let mut five: Option<Card> = None;
        for (idx, count) in counts.iter().enumerate() {
            match count {
                1 => singles.push(Card::from(idx as u8)),
                2 => pairs.push(Card::from(idx as u8)),
                3 => three = Some(Card::from(idx as u8)),
                4 => four = Some(Card::from(idx as u8)),
                5 => five = Some(Card::from(idx as u8)),
                _ => (),
            }
        }
        singles.sort_by_key(|c| Reverse(*c));
        pairs.sort_by_key(|c| Reverse(*c));
        let hand = if let Some(card) = five {
            HandType::FiveOfAKind(card)
        } else if let Some(card) = four {
            HandType::FourOfAKind(card, singles)
        } else if let Some(card) = three {
            if pairs.len() == 1 {
                HandType::FullHouse(card, pairs[0])
            } else {
                HandType::ThreeOfAKind(card, singles)
            }
        } else if pairs.len() == 2 {
            HandType::TwoPair(pairs[0], pairs[1], singles)
        } else if pairs.len() == 1 {
            HandType::OnePair(pairs[0], singles)
        } else {
            HandType::HighCard(singles[0], singles[1..].to_vec())
        };
        hand.upgrade(&cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.hand, &other.hand) {
            (HandType::FiveOfAKind(_), HandType::FiveOfAKind(_)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::FourOfAKind(_, _), HandType::FourOfAKind(_, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::FullHouse(_, _), HandType::FullHouse(_, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::ThreeOfAKind(_, _), HandType::ThreeOfAKind(_, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::TwoPair(_, _, _), HandType::TwoPair(_, _, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::OnePair(_, _), HandType::OnePair(_, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            (HandType::HighCard(_, _), HandType::HighCard(_, _)) => {
                Some(self.cards.cmp(&other.cards))
            }
            _ => Some(self.hand.cmp(&other.hand)),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.partial_cmp(&other.hand) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            Some(Ordering::Equal) => self.cards.cmp(&other.cards),
            None => panic!("Invalid comparison: {:?} {:?}", self, other),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variant_index().cmp(&other.variant_index())
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

impl From<u8> for Card {
    fn from(n: u8) -> Self {
        match n {
            0 => Card::Joker,
            1 => Card::Two,
            2 => Card::Three,
            3 => Card::Four,
            4 => Card::Five,
            5 => Card::Six,
            6 => Card::Seven,
            7 => Card::Eight,
            8 => Card::Nine,
            9 => Card::Ten,
            10 => Card::Queen,
            11 => Card::King,
            12 => Card::Ace,
            _ => panic!("Invalid card: {}", n),
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let cards = s
            .trim()
            .split_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .map(|s| Card::from(s))
            .collect::<Vec<Card>>();
        let hand = HandType::from(cards.to_owned()).upgrade(&cards);
        let bid = s
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Hand { hand, bid, cards }
    }
}

impl HandType {
    fn variant_index(&self) -> usize {
        match self {
            HandType::HighCard(_, _) => 0,
            HandType::OnePair(_, _) => 1,
            HandType::TwoPair(_, _, _) => 2,
            HandType::ThreeOfAKind(_, _) => 3,
            HandType::FullHouse(_, _) => 4,
            HandType::FourOfAKind(_, _) => 5,
            HandType::FiveOfAKind(_) => 6,
        }
    }
    fn upgrade(&self, cards: &Vec<Card>) -> HandType {
        dbg!(&cards);
        let number_of_jokers = jokers(&cards);
        let top_card = match self {
            HandType::FiveOfAKind(top) => {
                top.clone()
            }
            HandType::FourOfAKind(top, _) => {
                top.clone()
            }
            HandType::FullHouse(top, _) => {
                top.clone()
            }
            HandType::ThreeOfAKind(_, _) => {
                Some(self.cards.cmp(&other.cards))
            }
            HandType::TwoPair(_, _, => {
                Some(self.cards.cmp(&other.cards))
            }
            HandType::OnePair(_, _) => {
                Some(self.cards.cmp(&other.cards))
            }
            HandType::HighCard(_, _) => {
                Some(self.cards.cmp(&other.cards))
            }
            _ => Some(self.hand.cmp(&other.hand)),
        }

        HandType::from(cards.to_owned())
    }
}

fn jokers(cards: &Vec<Card>) -> i32 {
    let mut jokers = 0;
    for card in cards.iter() {
        if *card == Card::Joker {
            jokers += 1;
        }
    }
    jokers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input), 5905);
    }
}
