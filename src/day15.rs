use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Equals(u8),
    Dash,
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Operation,
}

#[derive(Debug, Default)]
struct Box<'a> {
    labels: Vec<&'a str>,
    lenses: HashMap<&'a str, u8>,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Equals(lens) => f.write_fmt(format_args!("={lens}"))?,
            Self::Dash => f.write_char('-')?,
        }

        Ok(())
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(step: &str) -> Result<Self> {
        if step.ends_with('-') {
            Ok(Self::Dash)
        } else if step.as_bytes()[step.len() - 2] == b'=' {
            Ok(Self::Equals(
                (step.as_bytes()[step.len() - 1] as char)
                    .to_digit(10)
                    .map(u8::try_from)
                    .ok_or_else(|| anyhow!("invalid lens in step"))??,
            ))
        } else {
            bail!("invalid step '{step}'")
        }
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.label, self.op))
    }
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(step: &str) -> Result<Self> {
        let op = Operation::from_str(step)?;
        let split = match op {
            Operation::Equals(_) => '=',
            Operation::Dash => '-',
        };

        Ok(Self {
            label: step
                .split(split)
                .next()
                .ok_or_else(|| anyhow!("missing label in step"))?
                .to_string(),
            op,
        })
    }
}

impl std::fmt::Display for Box<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &label) in self.labels.iter().enumerate() {
            let lens = self.lenses.get(label).ok_or(std::fmt::Error).copied()?;

            if i > 0 {
                f.write_char(' ')?;
            }

            f.write_fmt(format_args!("[{label} {lens}]"))?;
        }

        Ok(())
    }
}

impl Step {
    fn find_box(&self) -> usize {
        usize::from(self::hash(&self.label))
    }
}

impl<'a> Box<'a> {
    fn handle(&mut self, step: &'a Step) {
        let label = step.label.as_str();

        match step.op {
            Operation::Equals(lens) => {
                if let Some(l) = self.lenses.get_mut(label) {
                    *l = lens;
                } else {
                    self.labels.push(label);
                    self.lenses.insert(label, lens);
                }
            }
            Operation::Dash => {
                if self.lenses.contains_key(label) {
                    self.labels.retain(|&l| l != step.label);
                    self.lenses.remove(label);
                }
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn hash(step: &str) -> u8 {
    let mut val = 0_u32;

    for &byte in step.as_bytes() {
        val += u32::from(byte);
        val *= 17;
        val &= 255;
    }

    val as u8
}

fn part1(seq: &[Step]) -> u32 {
    seq.iter()
        .map(|step| self::hash(&step.to_string()))
        .map(u32::from)
        .sum()
}

fn part2(seq: &[Step]) -> usize {
    let mut boxes = std::array::from_fn::<_, 256, _>(|_| Box::default());
    for step in seq {
        let idx = step.find_box();
        boxes[idx].handle(step);
    }

    let mut power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, &label) in b.labels.iter().enumerate() {
            power += (i + 1)
                * (j + 1)
                * b.lenses
                    .get(label)
                    .copied()
                    .map(usize::from)
                    .unwrap_or_default();
        }
    }

    power
}

fn main() -> Result<()> {
    let seq = fs::read_to_string("in/day15.txt")?
        .replace('\n', "")
        .split(',')
        .map(Step::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&seq);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 502_139);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&seq);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 284_132);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
    }
}
