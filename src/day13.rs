use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rock,
}

#[derive(Debug)]
struct Pattern {
    grid: Vec<Terrain>,
    height: usize,
    width: usize,
}

impl From<Terrain> for char {
    fn from(terrain: Terrain) -> Self {
        match terrain {
            Terrain::Ash => '.',
            Terrain::Rock => '#',
        }
    }
}

impl TryFrom<char> for Terrain {
    type Error = Error;

    fn try_from(terrain: char) -> Result<Self> {
        match terrain {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => bail!("invalid terrain '{terrain}'"),
        }
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(pattern: &str) -> Result<Self> {
        let grid = pattern
            .lines()
            .flat_map(|row| row.chars())
            .map(Terrain::try_from)
            .collect::<Result<_>>()?;

        let height = pattern.lines().count();
        let width = pattern
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty pattern"))
            .map(str::len)?;

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width {
                f.write_char(char::from(self.get(x, y).ok_or(std::fmt::Error)?))?;
            }
        }

        Ok(())
    }
}

impl Pattern {
    fn get(&self, x: usize, y: usize) -> Option<Terrain> {
        self.grid.get((y * self.width) + x).copied()
    }

    fn count_mismatches_vertical(&self, x: usize) -> usize {
        (0..=x)
            .rev()
            .zip(x + 1..self.width)
            .map(|(left, right)| {
                (0..self.height)
                    .filter(|&y| self.get(left, y) != self.get(right, y))
                    .count()
            })
            .sum()
    }

    fn count_mismatches_horizontal(&self, y: usize) -> usize {
        (0..=y)
            .rev()
            .zip(y + 1..self.height)
            .map(|(up, down)| {
                (0..self.width)
                    .filter(|&x| self.get(x, up) != self.get(x, down))
                    .count()
            })
            .sum()
    }

    fn find_reflection_vertical(&self, mismatches: usize) -> Option<usize> {
        (0..self.width - 1)
            .find(|&x| self.count_mismatches_vertical(x) == mismatches)
            .map(|x| x + 1)
    }

    fn find_reflection_horizontal(&self, mismatches: usize) -> Option<usize> {
        (0..self.height - 1)
            .find(|&y| self.count_mismatches_horizontal(y) == mismatches)
            .map(|y| y + 1)
    }
}

fn solve(patterns: &[Pattern], mismatches: usize) -> Result<usize> {
    let mut sum = 0;

    for (i, pattern) in patterns.iter().enumerate() {
        sum += pattern
            .find_reflection_vertical(mismatches)
            .or_else(|| {
                pattern
                    .find_reflection_horizontal(mismatches)
                    .map(|y| 100 * y)
            })
            .ok_or_else(|| anyhow!("no reflection found in pattern {i}"))?;
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let patterns = fs::read_to_string("in/day13.txt")?
        .split("\n\n")
        .map(Pattern::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::solve(&patterns, 0)?;
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 35_691);
    };

    {
        let start = Instant::now();
        let part2 = self::solve(&patterns, 1)?;
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 39_037);
    };

    Ok(())
}
