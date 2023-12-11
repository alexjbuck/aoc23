use std::cmp::Ordering;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 2b: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| Hand::from(line))
        .collect::<Vec<Hand>>();
    hands.sort();
    // hands.iter().enumerate().for_each(|(idx, hand)| {
    //     println!(
    //         "{:4} * {:4} = {:6} => {:?}",
    //         idx + 1,
    //         hand.bid,
    //         hand.bid * (idx + 1),
    //         hand
    //     )
    // });
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug, Hash)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    hand: HandType,
    cards: Vec<Card>,
    pub bid: usize,
}

impl From<Vec<Card>> for HandType {
    fn from(cards: Vec<Card>) -> HandType {
        let mut counts = [0 as usize; 13];
        for card in cards.iter() {
            match card {
                Card::Joker => (),
                _ => counts[*card as usize] += 1,
            }
        }
        let most_common_card = counts
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(c, _)| c)
            .unwrap_or(Card::Ace as usize);
        counts[most_common_card] = counts[most_common_card] + jokers(&cards);
        let mut pairs = 0;
        let mut three = false;
        let mut four = false;
        let mut five = false;
        for count in counts.iter() {
            match count {
                2 => pairs += 1,
                3 => three = true,
                4 => four = true,
                5 => five = true,
                _ => (),
            }
        }
        let hand = if five {
            HandType::FiveOfAKind
        } else if four {
            HandType::FourOfAKind
        } else if three {
            if pairs == 1 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if pairs == 2 {
            HandType::TwoPair
        } else if pairs == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };
        hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
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
        let hand = HandType::from(cards.to_owned());
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

fn jokers(cards: &Vec<Card>) -> usize {
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
