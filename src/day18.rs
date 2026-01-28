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
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    cubes: usize,
    color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
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

impl FromStr for Color {
    type Err = Error;

    fn from_str(color: &str) -> Result<Self> {
        if !color.starts_with('#') {
            bail!("missing '#' in RGB value");
        }

        let bytes = color
            .split('#')
            .nth(1)
            .ok_or_else(|| anyhow!("missing bytes in RGB value"))?;

        let red = bytes
            .get(0..2)
            .map(|byte| u8::from_str_radix(byte, 16))
            .ok_or_else(|| anyhow!("missing red color bytes in RGB value"))??;
        let green = bytes
            .get(2..4)
            .map(|byte| u8::from_str_radix(byte, 16))
            .ok_or_else(|| anyhow!("missing green color bytes in RGB value"))??;
        let blue = bytes
            .get(4..)
            .map(|byte| u8::from_str_radix(byte, 16))
            .ok_or_else(|| anyhow!("missing blue color bytes in RGB value"))??;

        Ok(Self { red, green, blue })
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
            .ok_or_else(|| anyhow!("missing cubes in dig plan"))?
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse()?;

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
}

impl Lagoon {
    fn width(&self) -> i32 {
        self.trench
            .iter()
            .map(|&pos| pos.x + 1)
            .max()
            .unwrap_or_default()
    }

    fn height(&self) -> i32 {
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
                    Dir::Up => pos.up(),
                    Dir::Right => pos.right(),
                    Dir::Down => pos.down(),
                    Dir::Left => pos.left(),
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

            for pos in [pos.up(), pos.right(), pos.down(), pos.left()] {
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

fn main() -> Result<()> {
    let steps = fs::read_to_string("in/day18.txt")?
        .lines()
        .map(Step::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = Lagoon::default()
            .dig_edges(&steps)
            .dig_interior()
            .trench
            .len();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 49_897);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_str() -> Result<()> {
        assert!(Color::from_str("70c710").is_err());
        assert!(Color::from_str("#").is_err());
        assert!(Color::from_str("#70").is_err());
        assert!(Color::from_str("#70c7").is_err());
        assert!(Color::from_str("#70c71").is_err());

        let color = Color::from_str("#70c710")?;
        assert_eq!(color.red, 0x70);
        assert_eq!(color.green, 0xc7);
        assert_eq!(color.blue, 0x10);

        Ok(())
    }
}
