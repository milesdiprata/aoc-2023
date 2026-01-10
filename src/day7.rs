use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
struct Hand {
    cards: [Card; 5],
    bid: u32,
    ty: HandType,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(card: char) -> Result<Self> {
        match card {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => bail!("invalid card '{card}'"),
        }
    }
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
        let mut counts = HashMap::<Card, u8>::with_capacity(5);
        for card in cards {
            *counts.entry(card).or_default() += 1;
        }

        let three_of_a_kind = counts.values().any(|&count| count == 3);
        let pair = counts.values().any(|&count| count == 2);

        if counts.values().any(|&count| count == 5) {
            Self::FiveOfAKind
        } else if counts.values().any(|&count| count == 4) {
            Self::FourOfAKind
        } else if three_of_a_kind && pair {
            Self::FullHouse
        } else if three_of_a_kind {
            Self::ThreeOfAKind
        } else if counts.values().filter(|&&count| count == 2).count() == 2 {
            Self::TwoPair
        } else if pair {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(hand: &str) -> Result<Self> {
        let mut parts = hand.split_ascii_whitespace();

        let cards = parts
            .next()
            .ok_or_else(|| anyhow!("missing cards in hand"))?
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("failed to convert cards into char array"))?;
        let bid = parts
            .next()
            .ok_or_else(|| anyhow!("missing bid in hand"))?
            .parse()?;

        Ok(Self::new(cards, bid))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        Self {
            cards,
            bid,
            ty: HandType::from(cards),
        }
    }
}

fn part1(hands: &mut [Hand]) -> usize {
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1, hand))
        .map(|(rank, hand)| rank * hand.bid as usize)
        .sum()
}

fn main() -> Result<()> {
    let mut hands = fs::read_to_string("in/day7.txt")?
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut hands);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 246_424_613);
    };

    Ok(())
}
