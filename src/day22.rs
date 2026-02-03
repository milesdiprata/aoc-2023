use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Brick {
    ends: (Pos, Pos),
}

#[derive(Debug)]
struct Stack {
    bricks: Vec<Brick>,
}

#[allow(dead_code)]
struct StackPerspective<'a> {
    stack: &'a Stack,
    axis_min: fn(&'a Brick) -> i32,
    axis_max: fn(&'a Brick) -> i32,
    axis_label: char,
}

#[allow(dead_code)]
struct StackPerspectiveX<'a> {
    stack: &'a Stack,
}

#[allow(dead_code)]
struct StackPerspectiveY<'a> {
    stack: &'a Stack,
}

struct SupportGraph {
    len: usize,
    supports: HashMap<usize, HashSet<usize>>,
    supported_by: HashMap<usize, HashSet<usize>>,
}

impl std::fmt::Display for StackPerspective<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h_min = self
            .stack
            .bricks
            .iter()
            .map(self.axis_min)
            .min()
            .unwrap_or_default();
        let h_max = self
            .stack
            .bricks
            .iter()
            .map(self.axis_max)
            .max()
            .unwrap_or_default();
        let z_min = self
            .stack
            .bricks
            .iter()
            .map(Brick::z_min)
            .min()
            .unwrap_or_default();
        let z_max = self
            .stack
            .bricks
            .iter()
            .map(Brick::z_max)
            .max()
            .unwrap_or_default();

        let bricks = self
            .stack
            .bricks
            .iter()
            .enumerate()
            .map(|(i, brick)| u8::try_from(i).map(|i| ((i + b'A') as char, brick)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| std::fmt::Error)?;

        for h in h_min..=h_max {
            if h == i32::midpoint(h_max, h_min) {
                writeln!(f, "{}", self.axis_label)?;
                break;
            }

            f.write_char(' ')?;
        }
        for h in h_min..=h_max {
            write!(f, "{h}")?;
        }

        f.write_char('\n')?;

        for z in (z_min..=z_max).rev() {
            for h in h_min..=h_max {
                let mut matches = bricks.iter().filter(|(_, brick)| {
                    (self.axis_min)(brick) <= h
                        && (self.axis_max)(brick) >= h
                        && brick.z_min() <= z
                        && z <= brick.z_max()
                });

                match (matches.next(), matches.next()) {
                    (Some(&(label, _)), None) => f.write_char(label)?,
                    (Some(_), Some(_)) => f.write_char('?')?,
                    _ => f.write_char('.')?,
                }
            }

            if z == i32::midpoint(z_min, z_max) {
                writeln!(f, " {z} z")?;
            } else {
                writeln!(f, " {z}")?;
            }
        }

        for _ in h_min..=h_max {
            f.write_char('-')?;
        }
        f.write_str(" 0")?;

        Ok(())
    }
}

impl std::fmt::Display for StackPerspectiveX<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            StackPerspective {
                stack: self.stack,
                axis_min: Brick::x_min,
                axis_max: Brick::x_max,
                axis_label: 'x',
            }
        )
    }
}

impl std::fmt::Display for StackPerspectiveY<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            StackPerspective {
                stack: self.stack,
                axis_min: Brick::y_min,
                axis_max: Brick::y_max,
                axis_label: 'y',
            }
        )
    }
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(pos: &str) -> Result<Self> {
        let mut coords = pos.split(',').map(str::parse);

        Ok(Self {
            x: coords
                .next()
                .ok_or_else(|| anyhow!("missing x-coordinate"))??,
            y: coords
                .next()
                .ok_or_else(|| anyhow!("missing y-coordinate"))??,
            z: coords
                .next()
                .ok_or_else(|| anyhow!("missing z-coordinate"))??,
        })
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(brick: &str) -> Result<Self> {
        let mut ends = brick.split('~').map(str::parse);

        Ok(Self {
            ends: (
                ends.next()
                    .ok_or_else(|| anyhow!("missing first brick end coordinate"))??,
                ends.next()
                    .ok_or_else(|| anyhow!("missing first brick end coordinate"))??,
            ),
        })
    }
}

impl FromStr for Stack {
    type Err = Error;

    fn from_str(stack: &str) -> Result<Self> {
        Ok(Self {
            bricks: stack.lines().map(Brick::from_str).collect::<Result<_>>()?,
        })
    }
}

impl Brick {
    fn x_min(&self) -> i32 {
        self.ends.0.x.min(self.ends.1.x)
    }

    fn x_max(&self) -> i32 {
        self.ends.0.x.max(self.ends.1.x)
    }

    fn y_min(&self) -> i32 {
        self.ends.0.y.min(self.ends.1.y)
    }

    fn y_max(&self) -> i32 {
        self.ends.0.y.max(self.ends.1.y)
    }

    fn z_min(&self) -> i32 {
        self.ends.0.z.min(self.ends.1.z)
    }

    fn z_max(&self) -> i32 {
        self.ends.0.z.max(self.ends.1.z)
    }

    fn is_overlapped_xy(&self, other: &Self) -> bool {
        self.x_min() <= other.x_max()
            && other.x_min() <= self.x_max()
            && self.y_min() <= other.y_max()
            && other.y_min() <= self.y_max()
    }

    fn is_supported_by(&self, other: &Self) -> bool {
        self.is_overlapped_xy(other) && other.z_max() + 1 == self.z_min()
    }

    fn update_height(&mut self, z: i32) {
        let height = self.z_max() - self.z_min();

        match self.ends.0.z.cmp(&self.ends.1.z) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                self.ends.0.z = z;
                self.ends.1.z = z + height;
            }
            std::cmp::Ordering::Greater => {
                self.ends.0.z = z + height;
                self.ends.1.z = z;
            }
        }
    }
}

impl Stack {
    #[allow(dead_code)]
    const fn perspective_x(&self) -> StackPerspectiveX<'_> {
        StackPerspectiveX { stack: self }
    }

    #[allow(dead_code)]
    const fn perspective_y(&self) -> StackPerspectiveY<'_> {
        StackPerspectiveY { stack: self }
    }

    fn simulate_fall(&mut self) -> SupportGraph {
        self.bricks.sort_by_key(Brick::z_min);
        for i in 0..self.bricks.len() {
            let z = (0..i)
                .filter(|&j| self.bricks[i].is_overlapped_xy(&self.bricks[j]))
                .map(|j| self.bricks[j].z_max())
                .max()
                .unwrap_or_default()
                + 1;

            self.bricks[i].update_height(z);
        }

        let mut graph = SupportGraph::new(self.bricks.len());
        for i in 0..self.bricks.len() {
            for j in 0..self.bricks.len() {
                if i != j && self.bricks[j].is_supported_by(&self.bricks[i]) {
                    graph.supports.entry(i).or_default().insert(j);
                    graph.supported_by.entry(j).or_default().insert(i);
                }
            }
        }

        graph
    }
}

impl SupportGraph {
    fn new(len: usize) -> Self {
        Self {
            len,
            supports: HashMap::new(),
            supported_by: HashMap::new(),
        }
    }

    fn safe_count(&self) -> usize {
        let mut count = 0;

        for i in 0..self.len {
            if let Some(supports) = self.supports.get(&i) {
                if supports.iter().all(|&j| {
                    self.supported_by
                        .get(&j)
                        .is_some_and(|supporters| supporters.len() >= 2)
                }) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }

        count
    }
}

fn main() -> Result<()> {
    let mut stack = Stack::from_str(&fs::read_to_string("in/day22.txt")?)?;

    {
        let start = Instant::now();
        let part1 = stack.simulate_fall().safe_count();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 401);
    };

    Ok(())
}
