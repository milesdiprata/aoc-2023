use std::fs;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

fn parse() -> Result<(Vec<Race>, Race)> {
    let input = fs::read_to_string("in/day6.txt")?;
    let mut input = input.lines();

    let times = input
        .next()
        .ok_or_else(|| anyhow!("missing race times"))?
        .split("Time:")
        .nth(1)
        .ok_or_else(|| anyhow!("misformed race times"))?;
    let dists = input
        .next()
        .ok_or_else(|| anyhow!("missing race distances"))?
        .split("Distance:")
        .nth(1)
        .ok_or_else(|| anyhow!("misformed race distances"))?;

    let races = times
        .split_ascii_whitespace()
        .map(str::parse)
        .zip(dists.split_ascii_whitespace().map(str::parse))
        .map(|(time, dist)| {
            Ok(Race {
                time: time?,
                dist: dist?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let time = times.replace(' ', "").parse()?;
    let dist = dists.replace(' ', "").parse()?;

    Ok((races, Race { time, dist }))
}

impl Race {
    const fn is_record_breaking(&self, speed: u64) -> bool {
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

fn part2(race: &Race) -> usize {
    race.num_record_breaking()
}

fn main() -> Result<()> {
    let (races, race) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&races);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_811_940);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&race);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 30_077_773);
    };

    Ok(())
}
