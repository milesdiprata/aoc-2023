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

#[derive(Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

struct Map {
    grid: Vec<Tile>,
    width: isize,
    height: isize,
    start: Pos,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width {
                let pos = Pos { x, y };
                if pos == self.start {
                    f.write_char('S')?;
                } else {
                    f.write_fmt(format_args!("{}", self.get(pos).ok_or(std::fmt::Error)?))?;
                }
            }
        }

        Ok(())
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::GardenPlot => '.',
            Tile::Rock => '#',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(tile: char) -> Result<Self> {
        match tile {
            'S' | '.' => Ok(Self::GardenPlot),
            '#' => Ok(Self::Rock),
            _ => bail!("invalid tile 'tile'"),
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let grid = map
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid
            .first()
            .map(Vec::len)
            .ok_or_else(|| anyhow!("empty grid"))?;

        let start = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .find(|&(x, y)| grid[y][x] == 'S')
            .map(|(x, y)| (x.cast_signed(), y.cast_signed()))
            .map(|(x, y)| Pos { x, y })
            .ok_or_else(|| anyhow!("missing start position"))?;

        let grid = grid
            .into_iter()
            .flat_map(Vec::into_iter)
            .map(Tile::try_from)
            .collect::<Result<_>>()?;

        Ok(Self {
            grid,
            width: width.cast_signed(),
            height: height.cast_signed(),
            start,
        })
    }
}

impl Tile {
    const fn is_garden_plot(self) -> bool {
        matches!(self, Self::GardenPlot)
    }
}

impl Pos {
    const fn up(self) -> Self {
        Self {
            y: self.y - 1,
            ..self
        }
    }

    const fn right(self) -> Self {
        Self {
            x: self.x + 1,
            ..self
        }
    }

    const fn down(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }

    const fn left(self) -> Self {
        Self {
            x: self.x - 1,
            ..self
        }
    }

    const fn neighbors(self) -> [Self; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }
}

impl Map {
    fn get(&self, pos: Pos) -> Option<Tile> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            Some(self.grid[((pos.y * self.width) + pos.x).cast_unsigned()])
        } else {
            None
        }
    }

    fn get_wrapped(&self, pos: Pos) -> Tile {
        let x = pos.x.rem_euclid(self.width);
        let y = pos.y.rem_euclid(self.height);
        self.grid[((y * self.width) + x).cast_unsigned()]
    }

    fn neighbors_bounded(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        pos.neighbors()
            .into_iter()
            .filter(|&pos| self.get(pos).is_some_and(Tile::is_garden_plot))
    }

    fn neighbors_wrapped(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        pos.neighbors()
            .into_iter()
            .filter(|&pos| self.get_wrapped(pos).is_garden_plot())
    }

    /// Elf is allowed to backtrack (step on a tile, then step back)
    /// After N steps, the elf can be on any tile that has shortest distance
    /// from the start with same parity as N
    ///
    /// E.g.:
    ///
    /// . . .
    /// . S .
    /// . . .
    ///
    /// After exactly N = 2 steps, elf can be at:
    ///   - S (distance 0, even): go right, then come back
    ///   - Any tile at distance 2 (even): the four corners
    ///
    /// Any tile at distance 1 (odd): the four adjacent tiles are NOT reachable,
    /// elf would arrive with one step leftover, which is not enough to go
    /// somewhere and come back
    fn explore(&self, steps_max: usize) -> usize {
        let mut frontier = VecDeque::from([(self.start, 0_usize)]);
        let mut visited = HashSet::from([self.start]);

        // Can reach start position if N is even
        let mut count = usize::from(steps_max.is_multiple_of(2));

        while let Some((pos, dist)) = frontier.pop_front() {
            let next_dist = dist + 1;
            for next in self.neighbors_bounded(pos) {
                if next_dist <= steps_max && visited.insert(next) {
                    frontier.push_back((next, next_dist));

                    if next_dist % 2 == steps_max % 2 {
                        // Tile's shortest distance has same parity as N
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn explore_infinite(&self, steps_max: usize) -> usize {
        let mut frontier = VecDeque::from([(self.start, 0_usize)]);
        let mut visited = HashSet::from([self.start]);
        let mut count = usize::from(steps_max.is_multiple_of(2));

        while let Some((pos, dist)) = frontier.pop_front() {
            let next_dist = dist + 1;
            for next in self.neighbors_wrapped(pos) {
                if next_dist <= steps_max && visited.insert(next) {
                    frontier.push_back((next, next_dist));

                    if next_dist % 2 == steps_max % 2 {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

const fn quadratic_interpolate(f0: isize, f1: isize, f2: isize, n: isize) -> isize {
    let c = f0;
    let a = isize::midpoint(f2 - 2 * f1, f0);
    let b = f1 - f0 - a;
    a * n * n + b * n + c
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day21.txt")?)?;

    {
        let start = Instant::now();
        let part1 = map.explore(64);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_731);
    };

    {
        let start = Instant::now();

        let f0 = map.explore_infinite(65).cast_signed();
        let f1 = map.explore_infinite(65 + 131).cast_signed();
        let f2 = map.explore_infinite(65 + (2 * 131)).cast_signed();
        let n = 202_300; // (26_501_365 - 65) / 131

        let part2 = self::quadratic_interpolate(f0, f1, f2, n);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 617_565_692_567_199);
    };

    Ok(())
}
