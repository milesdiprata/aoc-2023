use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
    steps: u8,
}

struct Map {
    grid: Vec<u8>,
    height: usize,
    width: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let grid = map
            .lines()
            .flat_map(|row| row.chars())
            .map(|loss| loss.to_digit(10))
            .map(|loss| loss.map(u8::try_from))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("map contains invalid digit(s)"))?
            .into_iter()
            .collect::<Result<_, _>>()?;

        let height = map.lines().count();
        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty map"))?
            .len();

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl Dir {
    const fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
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

impl Map {
    fn get(&self, pos: Pos) -> Option<u8> {
        if pos.x < self.width && pos.y < self.height {
            Some(self.grid[(pos.y * self.width) + pos.x])
        } else {
            None
        }
    }

    fn next<'a>(&'a self, state: &'a State) -> impl Iterator<Item = State> + 'a {
        const DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

        DIRS.into_iter().filter_map(move |dir| {
            if dir == state.dir.reverse() {
                return None;
            }

            let steps = if dir == state.dir { state.steps + 1 } else { 1 };
            if steps > 3 {
                return None;
            }

            let pos = match dir {
                Dir::Up => state.pos.up(),
                Dir::Right => state.pos.right(),
                Dir::Down => state.pos.down(),
                Dir::Left => state.pos.left(),
            }?;

            self.get(pos)?;

            Some(State { pos, dir, steps })
        })
    }

    fn min_heat_loss(&self) -> Option<u32> {
        let start = Pos { x: 0, y: 0 };
        let goal = Pos {
            x: self.width - 1,
            y: self.height - 1,
        };

        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push(Reverse((
            0_u32,
            State {
                pos: start,
                dir: Dir::Right,
                steps: 0,
            },
        )));
        heap.push(Reverse((
            0_u32,
            State {
                pos: start,
                dir: Dir::Down,
                steps: 0,
            },
        )));

        while let Some(Reverse((cost, state))) = heap.pop() {
            if state.pos == goal {
                return Some(cost);
            }

            if !visited.insert(state.clone()) {
                continue;
            }

            for state in self.next(&state) {
                let cost = cost + u32::from(self.get(state.pos)?);
                heap.push(Reverse((cost, state)));
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day17.txt")?)?;

    {
        let start = Instant::now();
        let part1 = map.min_heat_loss().unwrap_or_default();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 959);
    };

    Ok(())
}
