use std::fs;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

fn parse() -> Result<Vec<Race>> {
    let input = fs::read_to_string("in/day6.txt")?;
    let mut input = input.lines();

    let times = input
        .next()
        .ok_or_else(|| anyhow!("missing race times"))?
        .split("Time:")
        .nth(1)
        .ok_or_else(|| anyhow!("misformed race times"))?
        .split_ascii_whitespace()
        .map(str::parse::<u32>);
    let dists = input
        .next()
        .ok_or_else(|| anyhow!("missing race distances"))?
        .split("Distance:")
        .nth(1)
        .ok_or_else(|| anyhow!("misformed race distances"))?
        .split_ascii_whitespace()
        .map(str::parse::<u32>);

    times
        .zip(dists)
        .map(|(time, dist)| {
            Ok(Race {
                time: time?,
                dist: dist?,
            })
        })
        .collect::<Result<Vec<_>>>()
}

impl Race {
    const fn is_record_breaking(&self, speed: u32) -> bool {
        speed * (self.time - speed) > self.dist
    }

    fn num_record_breaking(&self) -> usize {
        (0..self.time - 1)
            .filter(|&speed| self.is_record_breaking(speed))
            .count()
    }
}

fn part1(races: &[Race]) -> usize {
    races.iter().map(Race::num_record_breaking).product()
}

fn main() -> Result<()> {
    let races = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&races);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_811_940);
    };

    Ok(())
}
