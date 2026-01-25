use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Contraption {
    grid: Vec<Tile>,
    height: usize,
    width: usize,
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => '.',
            Tile::ForwardMirror => '/',
            Tile::BackwardMirror => '\\',
            Tile::VerticalSplitter => '|',
            Tile::HorizontalSplitter => '-',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(tile: char) -> Result<Self> {
        match tile {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::ForwardMirror),
            '\\' => Ok(Self::BackwardMirror),
            '|' => Ok(Self::VerticalSplitter),
            '-' => Ok(Self::HorizontalSplitter),
            _ => bail!("invalid tile '{tile}'"),
        }
    }
}

impl FromStr for Contraption {
    type Err = Error;

    fn from_str(contraption: &str) -> Result<Self> {
        let grid = contraption
            .lines()
            .flat_map(|row| row.chars())
            .map(Tile::try_from)
            .collect::<Result<_>>()?;

        let height = contraption.lines().count();
        let width = contraption
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty contraption"))?
            .len();

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl std::fmt::Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.height {
                f.write_fmt(format_args!(
                    "{}",
                    self.get(Pos { x, y }).ok_or(std::fmt::Error)?
                ))?;
            }
        }

        Ok(())
    }
}

impl Pos {
    fn up(self) -> Option<Self> {
        Some(Self {
            y: self.y.checked_sub(1)?,
            ..self
        })
    }

    fn right(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(1)?,
            ..self
        })
    }

    fn down(self) -> Option<Self> {
        Some(Self {
            y: self.y.checked_add(1)?,
            ..self
        })
    }

    fn left(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            ..self
        })
    }
}

impl Contraption {
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get((pos.y * self.width) + pos.x).copied()
    }

    fn next(&self, pos: Pos, dir: Dir) -> impl Iterator<Item = (Pos, Dir)> + '_ {
        match self.get(pos) {
            Some(Tile::Empty) => match dir {
                Dir::Up => [pos.up().map(|pos| (pos, dir)), None],
                Dir::Right => [pos.right().map(|pos| (pos, dir)), None],
                Dir::Down => [pos.down().map(|pos| (pos, dir)), None],
                Dir::Left => [pos.left().map(|pos| (pos, dir)), None],
            },
            Some(Tile::ForwardMirror) => match dir {
                Dir::Up => [pos.right().map(|pos| (pos, Dir::Right)), None],
                Dir::Right => [pos.up().map(|pos| (pos, Dir::Up)), None],
                Dir::Down => [pos.left().map(|pos| (pos, Dir::Left)), None],
                Dir::Left => [pos.down().map(|pos| (pos, Dir::Down)), None],
            },
            Some(Tile::BackwardMirror) => match dir {
                Dir::Up => [pos.left().map(|pos| (pos, Dir::Left)), None],
                Dir::Right => [pos.down().map(|pos| (pos, Dir::Down)), None],
                Dir::Down => [pos.right().map(|pos| (pos, Dir::Right)), None],
                Dir::Left => [pos.up().map(|pos| (pos, Dir::Up)), None],
            },
            Some(Tile::VerticalSplitter) => match dir {
                Dir::Up => [pos.up().map(|pos| (pos, dir)), None],
                Dir::Right | Dir::Left => [
                    pos.up().map(|pos| (pos, Dir::Up)),
                    pos.down().map(|pos| (pos, Dir::Down)),
                ],
                Dir::Down => [pos.down().map(|pos| (pos, dir)), None],
            },
            Some(Tile::HorizontalSplitter) => match dir {
                Dir::Up | Dir::Down => [
                    pos.right().map(|pos| (pos, Dir::Right)),
                    pos.left().map(|pos| (pos, Dir::Left)),
                ],
                Dir::Right => [pos.right().map(|pos| (pos, dir)), None],
                Dir::Left => [pos.left().map(|pos| (pos, dir)), None],
            },
            None => [None, None],
        }
        .into_iter()
        .flatten()
        .filter(|&(pos, _)| pos.x < self.width && pos.y < self.height)
    }
}

fn part1(contraption: &Contraption) -> usize {
    const START: Pos = Pos { x: 0, y: 0 };

    let mut frontier = Vec::from([(START, Dir::Right)]);
    let mut visited = HashSet::new();

    while let Some((pos, dir)) = frontier.pop() {
        if !visited.insert((pos, dir)) {
            continue;
        }

        for (pos_next, dir_next) in contraption.next(pos, dir) {
            frontier.push((pos_next, dir_next));
        }
    }

    visited
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<()> {
    let contraption = Contraption::from_str(&fs::read_to_string("in/day16.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&contraption);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 7_067);
    };

    Ok(())
}
