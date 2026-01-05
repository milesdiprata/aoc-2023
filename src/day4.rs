use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Card {
    _id: usize,
    winning: Vec<u32>,
    mine: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(card: &str) -> Result<Self> {
        let mut parts = card.split(": ");

        let id = parts
            .next()
            .ok_or_else(|| anyhow!("missing card info"))?
            .split_ascii_whitespace()
            .nth(1)
            .ok_or_else(|| anyhow!("missing card ID"))?
            .parse()?;

        let mut nums = parts
            .next()
            .ok_or_else(|| anyhow!("missing scratchcard numbers"))?
            .split(" | ");
        let winning = nums
            .next()
            .ok_or_else(|| anyhow!("missing scratchcard winning numbers"))?
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let mine = nums
            .next()
            .ok_or_else(|| anyhow!("missing scratchcard user numbers"))?
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect();

        Ok(Self {
            _id: id,
            winning,
            mine,
        })
    }
}

impl Card {
    fn points(&self) -> u32 {
        let mut points = 0;

        for &win in &self.winning {
            if self.mine.contains(&win) {
                points = match points {
                    0 => 1,
                    points => points << 1,
                };
            }
        }

        points
    }

    fn wins(&self) -> usize {
        self.winning
            .iter()
            .filter(|&win| self.mine.contains(win))
            .count()
    }
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(Card::points).sum()
}

fn part2(cards: &[Card]) -> usize {
    let mut counts = vec![1_usize; cards.len()];

    for i in 0..counts.len() {
        for j in i + 1..=i + cards[i].wins() {
            counts[j] += counts[i];
        }
    }

    counts.into_iter().sum()
}

fn main() -> Result<()> {
    let cards = fs::read_to_string("in/day4.txt")?
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&cards);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 25_183);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&cards);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 5_667_240);
    };

    Ok(())
}
