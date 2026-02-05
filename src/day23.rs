use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    RightSlope,
    DownSlope,
    LeftSlope,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Path => '.',
            Tile::Forest => '#',
            Tile::UpSlope => '^',
            Tile::RightSlope => '>',
            Tile::DownSlope => 'v',
            Tile::LeftSlope => '<',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(tile: char) -> Result<Self> {
        match tile {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::UpSlope),
            '>' => Ok(Self::RightSlope),
            'v' => Ok(Self::DownSlope),
            '<' => Ok(Self::LeftSlope),
            _ => bail!("invalid tile '{tile}'"),
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let grid = map
            .lines()
            .flat_map(|row| row.chars())
            .map(Tile::try_from)
            .collect::<Result<Vec<_>>>()?;

        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty map"))?
            .len();
        let height = map.lines().count();

        let mut map = Self {
            grid,
            width,
            height,
            start: Pos::default(),
            end: Pos::default(),
        };

        let start = (0..width)
            .map(|x| Pos { x, y: 0 })
            .find(|&pos| map.get(pos).is_some_and(Tile::is_path))
            .ok_or_else(|| anyhow!("missing start position"))?;
        let end = (0..width)
            .map(|x| Pos { x, y: height - 1 })
            .find(|&pos| map.get(pos).is_some_and(Tile::is_path))
            .ok_or_else(|| anyhow!("missing end position"))?;

        map.start = start;
        map.end = end;

        Ok(map)
    }
}

impl Tile {
    const fn is_path(self) -> bool {
        matches!(self, Self::Path)
    }

    const fn is_traversable(self) -> bool {
        !matches!(self, Self::Forest)
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

    fn neighbors(self) -> [Option<Self>; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }
}

impl Map {
    const fn idx(&self, pos: Pos) -> Option<usize> {
        if pos.x < self.width && pos.y < self.height {
            Some((self.width * pos.y) + pos.x)
        } else {
            None
        }
    }

    fn get(&self, pos: Pos) -> Option<Tile> {
        self.idx(pos).map(|idx| self.grid[idx])
    }

    fn compress(&self) -> (Vec<Vec<(usize, usize)>>, usize, usize) {
        // Finds junctions (positions with 3+ traversable neighbors), including
        // start and end positions
        let mut junctions = vec![self.start, self.end];
        for pos in (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| Pos { x, y }))
            .filter(|&pos| pos != self.start && pos != self.end)
            .filter(|&pos| self.get(pos).is_some_and(Tile::is_traversable))
            .filter(|&pos| self.neighbors_raw(pos).into_iter().flatten().count() >= 3)
        {
            junctions.push(pos);
        }

        let junction_idxs = junctions
            .iter()
            .enumerate()
            .map(|(idx, &pos)| (pos, idx))
            .collect::<HashMap<_, _>>();

        // Finds distance between all junctions
        let mut edges = vec![vec![]; junctions.len()];
        for (i, &junction) in junctions.iter().enumerate() {
            for neighbor in self.neighbors_raw(junction).into_iter().flatten() {
                let mut prev = junction;
                let mut curr = neighbor;
                let mut dist = 1;

                while !junction_idxs.contains_key(&curr) {
                    let next = self
                        .neighbors_raw(curr)
                        .into_iter()
                        .flatten()
                        .find(|&next| next != prev)
                        .unwrap();

                    prev = curr;
                    curr = next;
                    dist += 1;
                }

                edges[i].push((junction_idxs[&curr], dist));
            }
        }

        (edges, junction_idxs[&self.start], junction_idxs[&self.end])
    }

    fn neighbors(&self, pos: Pos) -> [Option<Pos>; 4] {
        match self.get(pos) {
            Some(Tile::Path) => pos.neighbors(),
            Some(Tile::UpSlope) => [pos.up(), None, None, None],
            Some(Tile::RightSlope) => [pos.right(), None, None, None],
            Some(Tile::DownSlope) => [pos.down(), None, None, None],
            Some(Tile::LeftSlope) => [pos.left(), None, None, None],
            Some(Tile::Forest) | None => [None; 4],
        }
        .map(|pos| self.get(pos?)?.is_traversable().then_some(pos).flatten())
    }

    fn neighbors_raw(&self, pos: Pos) -> [Option<Pos>; 4] {
        pos.neighbors()
            .map(|pos| self.get(pos?)?.is_traversable().then_some(pos).flatten())
    }

    fn longest_hike(&self) -> usize {
        fn dfs(map: &Map, pos: Pos, visited: &mut Vec<bool>) -> Option<usize> {
            if pos == map.end {
                return Some(0);
            }

            if visited[map.idx(pos)?] {
                return None;
            }

            visited[map.idx(pos)?] = true;

            let best = map
                .neighbors(pos)
                .into_iter()
                .flatten()
                .filter_map(|next| dfs(map, next, visited))
                .map(|dist| dist + 1)
                .max();

            visited[map.idx(pos)?] = false;

            best
        }

        let mut visited = vec![false; self.grid.len()];
        dfs(self, self.start, &mut visited).unwrap()
    }

    fn longest_hike_raw(&self) -> usize {
        fn dfs(
            edges: &[Vec<(usize, usize)>],
            pos: usize,
            end: usize,
            visited: &mut Vec<bool>,
        ) -> Option<usize> {
            if pos == end {
                return Some(0);
            }

            if visited[pos] {
                return None;
            }

            visited[pos] = true;

            let best = edges[pos]
                .iter()
                .filter_map(|&(next, weight)| {
                    dfs(edges, next, end, visited).map(|dist| dist + weight)
                })
                .max();

            visited[pos] = false;

            best
        }

        let (edges, start, end) = self.compress();
        let mut visited = vec![false; edges.len()];
        dfs(&edges, start, end, &mut visited).unwrap()
    }
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day23.txt")?)?;

    {
        let start = Instant::now();
        let part1 = map.longest_hike();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_074);
    };

    {
        let start = Instant::now();
        let part2 = map.longest_hike_raw();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 6_494);
    };

    Ok(())
}
