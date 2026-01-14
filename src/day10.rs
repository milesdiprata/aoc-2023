use std::collections::HashSet;
use std::collections::VecDeque;
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
    VerticalPipe,
    HorizontalPipe,
    UpRightPipe,
    UpLeftPipe,
    DownLeftPipe,
    DownRightPipe,
    Ground,
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
    start: Pos,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VerticalPipe => f.write_char('|'),
            Self::HorizontalPipe => f.write_char('-'),
            Self::UpRightPipe => f.write_char('L'),
            Self::UpLeftPipe => f.write_char('J'),
            Self::DownLeftPipe => f.write_char('7'),
            Self::DownRightPipe => f.write_char('F'),
            Self::Ground => f.write_char('.'),
            Self::Start => f.write_char('S'),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(tile: char) -> Result<Self> {
        match tile {
            '|' => Ok(Self::VerticalPipe),
            '-' => Ok(Self::HorizontalPipe),
            'L' => Ok(Self::UpRightPipe),
            'J' => Ok(Self::UpLeftPipe),
            '7' => Ok(Self::DownLeftPipe),
            'F' => Ok(Self::DownRightPipe),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => bail!("invalid tile '{tile}'"),
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width {
                let tile = self.get(Pos { x, y }).unwrap();
                f.write_fmt(format_args!("{tile}"))?;
            }
        }

        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let height = grid.lines().count();
        let width = grid
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty grid"))?
            .len();

        let tiles = grid
            .lines()
            .flat_map(|row| row.chars())
            .map(Tile::try_from)
            .collect::<Result<_>>()?;

        let start = grid
            .lines()
            .enumerate()
            .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, tile)| (x, y, tile)))
            .find_map(|(x, y, tile)| (tile == 'S').then_some(Pos { x, y }))
            .ok_or_else(|| anyhow!("missing start in grid"))?;

        Ok(Self {
            tiles,
            height,
            width,
            start,
        })
    }
}

impl Tile {
    const fn is_pipe(self) -> bool {
        matches!(
            self,
            Self::VerticalPipe
                | Self::HorizontalPipe
                | Self::UpRightPipe
                | Self::UpLeftPipe
                | Self::DownLeftPipe
                | Self::DownRightPipe
        )
    }

    fn dirs(self, pos: Pos) -> [Option<Pos>; 4] {
        match self {
            Self::VerticalPipe => [pos.up(), None, pos.down(), None],
            Self::HorizontalPipe => [None, pos.right(), None, pos.left()],
            Self::UpRightPipe => [pos.up(), pos.right(), None, None],
            Self::UpLeftPipe => [pos.up(), None, None, pos.left()],
            Self::DownLeftPipe => [None, None, pos.down(), pos.left()],
            Self::DownRightPipe => [None, pos.right(), pos.down(), None],
            Self::Ground => [None, None, None, None],
            Self::Start => [pos.up(), pos.right(), pos.down(), pos.left()],
        }
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

impl Grid {
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.tiles.get((self.width * pos.y) + pos.x).copied()
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        self.get(pos)
            .map(|tile| tile.dirs(pos))
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(move |&next| {
                self.get(next).is_some_and(|tile| {
                    tile.is_pipe() && tile.dirs(next).into_iter().flatten().any(|p| p == pos)
                })
            })
    }

    fn find_furthest(&self) -> usize {
        let mut frontier = VecDeque::from([self.start]);
        let mut visited = HashSet::from([self.start]);
        let mut steps = 0;

        while !frontier.is_empty() {
            let mut inserted = false;

            for _ in 0..frontier.len() {
                let pos = frontier.pop_front().unwrap();
                for next in self.neighbors(pos) {
                    if visited.insert(next) {
                        frontier.push_back(next);
                        inserted = true;
                    }
                }
            }

            if inserted {
                steps += 1;
            }
        }

        steps
    }
}

fn main() -> Result<()> {
    let grid = Grid::from_str(&fs::read_to_string("in/day10.txt")?)?;

    {
        let start = Instant::now();
        let part1 = grid.find_furthest();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 6_860);
    };

    Ok(())
}
