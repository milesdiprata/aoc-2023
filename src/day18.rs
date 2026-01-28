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
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    cubes: u32,
    color: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
struct Lagoon {
    trench: HashSet<Pos>,
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(dir: char) -> Result<Self> {
        match dir {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => bail!("invalid direction '{dir}'"),
        }
    }
}

impl TryFrom<u32> for Dir {
    type Error = Error;

    fn try_from(dir: u32) -> Result<Self> {
        match dir {
            0 => Ok(Self::Right),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Up),
            _ => bail!("invalid direction '{dir}'"),
        }
    }
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(step: &str) -> Result<Self> {
        let mut parts = step.split_ascii_whitespace();

        let dir = parts
            .next()
            .ok_or_else(|| anyhow!("missing direction in dig plan"))?
            .chars()
            .next()
            .map(Dir::try_from)
            .ok_or_else(|| anyhow!("missing direction in dig plan"))??;
        let cubes = parts
            .next()
            .ok_or_else(|| anyhow!("missing cubes in dig plan"))?
            .parse()?;
        let color = parts
            .next()
            .ok_or_else(|| anyhow!("missing color in dig plan"))?
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('#')
            .nth(1)
            .map(|byte| u32::from_str_radix(byte, 16))
            .ok_or_else(|| anyhow!("missing '#' in color code"))??;

        Ok(Self { dir, cubes, color })
    }
}

impl std::fmt::Display for Lagoon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width() {
                f.write_char(if self.trench.contains(&Pos { x, y }) {
                    '#'
                } else {
                    '.'
                })?;
            }
        }

        Ok(())
    }
}

impl Step {
    fn extract(&self) -> Result<Self> {
        let dir = Dir::try_from(self.color & 0xF)?;
        let cubes = (self.color & 0x00FF_FFF0) >> 4;

        Ok(Self {
            dir,
            cubes,
            ..*self
        })
    }
}

impl Pos {
    const fn up(self, steps: i64) -> Self {
        Self {
            y: self.y - steps,
            ..self
        }
    }

    const fn right(self, steps: i64) -> Self {
        Self {
            x: self.x + steps,
            ..self
        }
    }

    const fn down(self, steps: i64) -> Self {
        Self {
            y: self.y + steps,
            ..self
        }
    }

    const fn left(self, steps: i64) -> Self {
        Self {
            x: self.x - steps,
            ..self
        }
    }
}

impl Lagoon {
    fn width(&self) -> i64 {
        self.trench
            .iter()
            .map(|&pos| pos.x + 1)
            .max()
            .unwrap_or_default()
    }

    fn height(&self) -> i64 {
        self.trench
            .iter()
            .map(|&pos| pos.y + 1)
            .max()
            .unwrap_or_default()
    }

    fn dig_edges(mut self, steps: &[Step]) -> Self {
        let mut pos = Pos { x: 0, y: 0 };
        self.trench.insert(pos);

        for step in steps {
            for _ in 0..step.cubes {
                pos = match step.dir {
                    Dir::Up => pos.up(1),
                    Dir::Right => pos.right(1),
                    Dir::Down => pos.down(1),
                    Dir::Left => pos.left(1),
                };
                self.trench.insert(pos);
            }
        }

        self
    }

    fn dig_interior(mut self) -> Self {
        let x_min = self
            .trench
            .iter()
            .map(|&pos| pos.x)
            .min()
            .unwrap_or_default();
        let x_max = self
            .trench
            .iter()
            .map(|&pos| pos.x)
            .max()
            .unwrap_or_default();
        let y_min = self
            .trench
            .iter()
            .map(|&pos| pos.y)
            .min()
            .unwrap_or_default();
        let y_max = self
            .trench
            .iter()
            .map(|&pos| pos.y)
            .max()
            .unwrap_or_default();

        let mut exterior = HashSet::new();
        let mut stack = Vec::from([Pos {
            x: x_min - 1,
            y: y_min - 1,
        }]);

        while let Some(pos) = stack.pop() {
            if pos.x < x_min - 1 || pos.y < y_min - 1 || pos.x > x_max + 1 || pos.y > y_max + 1 {
                continue;
            }

            if self.trench.contains(&pos) {
                continue;
            }

            if exterior.contains(&pos) {
                continue;
            }

            exterior.insert(pos);

            for pos in [pos.up(1), pos.right(1), pos.down(1), pos.left(1)] {
                stack.push(pos);
            }
        }

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let pos = Pos { x, y };

                if !exterior.contains(&pos) {
                    self.trench.insert(pos);
                }
            }
        }

        self
    }
}

fn part1(steps: &[Step]) -> usize {
    Lagoon::default()
        .dig_edges(steps)
        .dig_interior()
        .trench
        .len()
}

fn part2(steps: &[Step]) -> i64 {
    let mut pos = Pos { x: 0, y: 0 };
    let mut corners = Vec::with_capacity(steps.len());
    let mut boundary = 0;

    for step in steps {
        pos = match step.dir {
            Dir::Up => pos.up(i64::from(step.cubes)),
            Dir::Right => pos.right(i64::from(step.cubes)),
            Dir::Down => pos.down(i64::from(step.cubes)),
            Dir::Left => pos.left(i64::from(step.cubes)),
        };

        corners.push(pos);
        boundary += i64::from(step.cubes);
    }

    let mut area = 0;

    // Shoelace formula
    for i in 0..corners.len() {
        let j = (i + 1) % corners.len();
        area += corners[i].x * corners[j].y;
        area -= corners[j].x * corners[i].y;
    }
    area = area.abs() / 2;

    // Pick's theorem: i + b = A + b/2 + 1
    area + boundary / 2 + 1
}

fn main() -> Result<()> {
    let steps = fs::read_to_string("in/day18.txt")?
        .lines()
        .map(Step::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&steps);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 49_897);
    };

    {
        let start = Instant::now();
        let steps = steps
            .iter()
            .map(Step::extract)
            .collect::<Result<Vec<_>>>()?;
        let part2 = self::part2(&steps);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 194_033_958_221_830);
    };

    Ok(())
}
