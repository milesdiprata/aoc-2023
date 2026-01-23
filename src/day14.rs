use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug)]
struct Platform {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
            Tile::Empty => '.',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(tile: char) -> std::result::Result<Self, Self::Error> {
        match tile {
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::CubeRock),
            '.' => Ok(Self::Empty),
            _ => bail!("invalid platform tile '{tile}'"),
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(tiles: &str) -> Result<Self> {
        let height = tiles.lines().count();
        let width = tiles
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty platform"))?
            .len();

        let tiles = tiles
            .lines()
            .flat_map(|row| row.chars().map(Tile::try_from))
            .collect::<Result<_>>()?;

        Ok(Self {
            tiles,
            height,
            width,
        })
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width {
                f.write_char(char::from(self.get(x, y)))?;
            }
        }

        Ok(())
    }
}

impl Platform {
    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[(y * self.width) + x]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[(y * self.width) + x] = tile;
    }

    fn tilt_north(mut self) -> Self {
        for x in 0..self.width {
            let mut y_next = 0;

            for y in 0..self.height {
                match self.get(x, y) {
                    Tile::RoundRock => {
                        if y != y_next {
                            self.set(x, y_next, Tile::RoundRock);
                            self.set(x, y, Tile::Empty);
                        }

                        y_next += 1;
                    }
                    Tile::CubeRock => y_next = y + 1,
                    Tile::Empty => (),
                }
            }
        }

        self
    }
}

fn part1(platform: Platform) -> usize {
    let platform = platform.tilt_north();

    (0..platform.height)
        .flat_map(|y| (0..platform.width).map(move |x| (x, y)))
        .filter(|&(x, y)| platform.get(x, y) == Tile::RoundRock)
        .map(|(_, y)| platform.height - y)
        .sum()
}

fn main() -> Result<()> {
    let platform = Platform::from_str(&fs::read_to_string("in/day14.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(platform);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 110_821);
    };

    Ok(())
}
