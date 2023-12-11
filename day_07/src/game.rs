use std::cmp::Ordering;
use std::cmp::Reverse;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
pub enum Card {
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

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub enum HandType {
    HighCard(Card, Vec<Card>),
    OnePair(Card, Vec<Card>),
    TwoPair(Card, Card, Vec<Card>),
    ThreeOfAKind(Card, Vec<Card>),
    FullHouse(Card, Card),
    FourOfAKind(Card, Vec<Card>),
    FiveOfAKind(Card),
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Hand {
    cards: HandType,
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
        if let Some(card) = five {
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
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            Some(Ordering::Equal) => match (self, other) {
                (HandType::HighCard(h1, res1), HandType::HighCard(h2, res2)) => {
                    if h1 == h2 {
                        res1.cmp(res2)
                    } else {
                        h1.cmp(h2)
                    }
                }
                (HandType::OnePair(h1, res1), HandType::OnePair(h2, res2)) => {
                    if h1 == h2 {
                        res1.cmp(res2)
                    } else {
                        h1.cmp(h2)
                    }
                }
                (HandType::TwoPair(h1, h2, res1), HandType::TwoPair(h3, h4, res2)) => {
                    if h1 == h3 {
                        if h2 == h4 {
                            res1.cmp(res2)
                        } else {
                            h2.cmp(h4)
                        }
                    } else {
                        h1.cmp(h3)
                    }
                }
                (HandType::ThreeOfAKind(h1, res1), HandType::ThreeOfAKind(h2, res2)) => {
                    if h1 == h2 {
                        res1.cmp(res2)
                    } else {
                        h1.cmp(h2)
                    }
                }
                (HandType::FullHouse(h1, h2), HandType::FullHouse(h3, h4)) => {
                    if h1 == h3 {
                        h2.cmp(h4)
                    } else {
                        h1.cmp(h3)
                    }
                }
                (HandType::FourOfAKind(h1, res1), HandType::FourOfAKind(h2, res2)) => {
                    if h1 == h2 {
                        res1.cmp(res2)
                    } else {
                        h1.cmp(h2)
                    }
                }
                (HandType::FiveOfAKind(h1), HandType::FiveOfAKind(h2)) => h1.cmp(h2),
                _ => panic!("This should be impossible: {:?} {:?}", self, other),
            },
            None => panic!("Invalid comparison: {:?} {:?}", self, other),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
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
            0 => Card::Two,
            1 => Card::Three,
            2 => Card::Four,
            3 => Card::Five,
            4 => Card::Six,
            5 => Card::Seven,
            6 => Card::Eight,
            7 => Card::Nine,
            8 => Card::Ten,
            9 => Card::Jack,
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
            .collect::<Vec<Card>>()
            .into();
        let bid = s
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Hand { cards, bid }
    }
}
