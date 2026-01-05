use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

struct Schematic {
    width: i32,
    height: i32,
    grid: Vec<char>,
}

impl std::fmt::Display for Schematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width {
                if let Some(symbol) = self.get(Pos { x, y }) {
                    f.write_char(symbol)?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(schematic: &str) -> Result<Self> {
        Ok(Self {
            width: i32::try_from(
                schematic
                    .lines()
                    .next()
                    .ok_or_else(|| anyhow!("empty schematic"))?
                    .len(),
            )?,
            height: i32::try_from(schematic.lines().count())?,
            grid: schematic.lines().flat_map(|row| row.chars()).collect(),
        })
    }
}

impl Pos {
    const fn up(self) -> Self {
        Self {
            y: self.y - 1,
            ..self
        }
    }

    const fn up_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    const fn right(self) -> Self {
        Self {
            x: self.x + 1,
            ..self
        }
    }

    const fn right_down(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    const fn down(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }

    const fn down_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    const fn left(self) -> Self {
        Self {
            x: self.x - 1,
            ..self
        }
    }

    const fn up_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
}

impl Schematic {
    fn get(&self, pos: Pos) -> Option<char> {
        let width = usize::try_from(self.width).ok()?;
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;

        self.grid.get((y * width) + x).copied()
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [
            pos.up(),
            pos.up_right(),
            pos.right(),
            pos.right_down(),
            pos.down(),
            pos.down_left(),
            pos.left(),
            pos.up_left(),
        ]
        .into_iter()
        .filter(|&pos| self.get(pos).is_some_and(|symbol| symbol.is_ascii_digit()))
    }

    fn parse_no(&self, mut pos: Pos, visited: &mut HashSet<Pos>) -> u32 {
        let mut no = 0;

        while self
            .get(pos.left())
            .is_some_and(|symbol| symbol.is_ascii_digit())
        {
            pos = pos.left();
        }

        while let Some(digit) = self.get(pos).and_then(|symbol| symbol.to_digit(10)) {
            visited.insert(pos);
            no *= 10;
            no += digit;
            pos = pos.right();
        }

        no
    }

    fn part_nos(&self) -> Vec<u32> {
        let mut nos = Vec::new();
        let mut visited = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };

                if self
                    .get(pos)
                    .is_some_and(|symbol| symbol != '.' && !symbol.is_ascii_digit())
                {
                    for neighbor in self.neighbors(pos) {
                        if !visited.contains(&neighbor) {
                            nos.push(self.parse_no(neighbor, &mut visited));
                        }
                    }
                }
            }
        }

        nos
    }
}

fn part1(schematic: &Schematic) -> u32 {
    schematic.part_nos().into_iter().sum()
}

fn main() -> Result<()> {
    let schematic = Schematic::from_str(&fs::read_to_string("in/day3.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&schematic);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 551_094);
    };

    Ok(())
}
