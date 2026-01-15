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
                let tile = self.get(Pos { x, y }).ok_or(std::fmt::Error)?;
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

    fn start_tile(&self) -> Tile {
        let up = self
            .start
            .up()
            .and_then(|up| self.get(up))
            .is_some_and(|up| {
                matches!(
                    up,
                    Tile::VerticalPipe | Tile::DownRightPipe | Tile::DownLeftPipe
                )
            });
        let right = self
            .start
            .right()
            .and_then(|right| self.get(right))
            .is_some_and(|right| {
                matches!(
                    right,
                    Tile::HorizontalPipe | Tile::UpLeftPipe | Tile::DownLeftPipe
                )
            });
        let down: bool = self
            .start
            .down()
            .and_then(|down| self.get(down))
            .is_some_and(|down| {
                matches!(
                    down,
                    Tile::VerticalPipe | Tile::UpRightPipe | Tile::UpLeftPipe
                )
            });
        let left = self
            .start
            .left()
            .and_then(|left| self.get(left))
            .is_some_and(|left| {
                matches!(
                    left,
                    Tile::HorizontalPipe | Tile::UpRightPipe | Tile::DownRightPipe
                )
            });

        match (up, right, down, left) {
            (true, false, true, false) => Tile::VerticalPipe,
            (false, true, false, true) => Tile::HorizontalPipe,
            (true, true, false, false) => Tile::UpRightPipe,
            (true, false, false, true) => Tile::UpLeftPipe,
            (false, false, true, true) => Tile::DownLeftPipe,
            (false, true, true, false) => Tile::DownRightPipe,
            _ => unreachable!("start tile is not part of a loop"),
        }
    }

    fn trace_loop(&self) -> Vec<Pos> {
        let mut path = vec![self.start];
        let mut prev = self.start;

        let mut curr = self
            .start_tile()
            .dirs(self.start)
            .into_iter()
            .flatten()
            .find(|&next| self.get(next).is_some_and(Tile::is_pipe))
            .unwrap_or_else(|| unreachable!("start tile does not have a valid neighbor"));

        while curr != self.start {
            path.push(curr);

            let next = self
                .get(curr)
                .unwrap_or_else(|| unreachable!("tile in loop is not in the grid"))
                .dirs(curr)
                .into_iter()
                .flatten()
                .find(|&next| next != prev && self.get(next).is_some_and(Tile::is_pipe))
                .unwrap_or(self.start);

            prev = curr;
            curr = next;
        }

        path
    }
}

fn part1(grid: &Grid) -> usize {
    grid.trace_loop().len() / 2
}

fn part2(grid: &Grid) -> usize {
    // Shoelace + Pick's
    fn count_interior(path: &[Pos]) -> usize {
        let n = path.len();

        // Shoelace formula
        let mut area = 0;
        for i in 0..n {
            let curr = path[i];
            let next = path[(i + 1) % n];
            area += (curr.x.cast_signed()) * (next.y.cast_signed());
            area -= (next.x.cast_signed()) * (curr.y.cast_signed());
        }

        let area = area.abs() / 2;

        // Pick's theorem: A = i + b/2 - 1  =>  i = A - b/2 + 1
        let boundary = n.cast_signed();
        (area - boundary / 2 + 1).cast_unsigned()
    }

    count_interior(&grid.trace_loop())
}

fn main() -> Result<()> {
    let grid = Grid::from_str(&fs::read_to_string("in/day10.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&grid);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 6_860);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&grid);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 343);
    };

    Ok(())
}
