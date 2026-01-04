use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<(u32, Color)>>,
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(color: &str) -> Result<Self> {
        match color {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => bail!("invalid color '{color}'"),
        }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(game: &str) -> Result<Self> {
        let mut sets = Vec::new();
        let mut parts = game.split(": ");

        let id = parts
            .next()
            .ok_or_else(|| anyhow!("missing game ID part"))?
            .split_ascii_whitespace()
            .nth(1)
            .ok_or_else(|| anyhow!("missing game ID"))?
            .parse()?;

        for set in parts
            .next()
            .ok_or_else(|| anyhow!("missing sets of cubes"))?
            .split("; ")
        {
            let mut draw = Vec::new();

            for cube in set.split(", ") {
                let mut cube = cube.split_ascii_whitespace();

                let count = cube
                    .next()
                    .ok_or_else(|| anyhow!("missing cube count"))?
                    .parse()?;
                let color = cube
                    .next()
                    .ok_or_else(|| anyhow!("missing cube color"))?
                    .parse()?;

                draw.push((count, color));
            }

            sets.push(draw);
        }

        Ok(Self { id, sets })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        const MAX_RED: u32 = 12;
        const MAX_GREEN: u32 = 13;
        const MAX_BLUE: u32 = 14;

        for set in &self.sets {
            for &(count, color) in set {
                let impossible = match color {
                    Color::Red => count > MAX_RED,
                    Color::Green => count > MAX_GREEN,
                    Color::Blue => count > MAX_BLUE,
                };

                if impossible {
                    return false;
                }
            }
        }

        true
    }

    fn min_cubes(&self) -> (u32, u32, u32) {
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

        for set in &self.sets {
            for &(count, color) in set {
                match color {
                    Color::Red => min_red = min_red.max(count),
                    Color::Green => min_green = min_green.max(count),
                    Color::Blue => min_blue = min_blue.max(count),
                }
            }
        }

        (min_red, min_green, min_blue)
    }
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|&game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(Game::min_cubes)
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

fn main() -> Result<()> {
    let games = fs::read_to_string("in/day2.txt")?
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&games);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_449);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&games);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 63_981);
    };

    Ok(())
}
