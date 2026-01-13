use std::fs;
use std::str::FromStr;
use std::time::Instant;
use std::vec;

use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct History {
    vals: Vec<i32>,
}

impl FromStr for History {
    type Err = Error;

    fn from_str(history: &str) -> Result<Self> {
        Ok(Self {
            vals: history
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl History {
    fn extrapolate(&self) -> i32 {
        let mut sequence = self.vals.clone();
        let mut lasts = vec![];

        while sequence.iter().any(|&val| val != 0) {
            lasts.push(sequence.last().copied().unwrap());

            let mut next = vec![];

            for window in sequence.windows(2) {
                next.push(window[1] - window[0]);
            }

            sequence = next;
        }

        lasts.into_iter().rev().sum()
    }

    fn extrapolate_rev(&self) -> i32 {
        let mut sequence = self.vals.clone();
        let mut firsts = vec![];

        while sequence.iter().any(|&val| val != 0) {
            firsts.push(sequence.first().copied().unwrap());

            let mut next = vec![];

            for window in sequence.windows(2) {
                next.push(window[1] - window[0]);
            }

            sequence = next;
        }

        firsts.into_iter().rev().fold(0, |acc, val| val - acc)
    }
}

fn part1(histories: &[History]) -> i32 {
    histories.iter().map(History::extrapolate).sum()
}

fn part2(histories: &[History]) -> i32 {
    histories.iter().map(History::extrapolate_rev).sum()
}

fn main() -> Result<()> {
    let histories = fs::read_to_string("in/day9.txt")?
        .lines()
        .map(History::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&histories);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_708_206_096);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&histories);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_050);
    };

    Ok(())
}
