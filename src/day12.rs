use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    conditions: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

impl TryFrom<char> for Condition {
    type Error = Error;

    fn try_from(condition: char) -> Result<Self> {
        match condition {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => bail!("invalid spring condition '{condition}'"),
        }
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(record: &str) -> Result<Self> {
        let mut parts = record.split_ascii_whitespace();

        let conditions = parts
            .next()
            .ok_or_else(|| anyhow!("missing partially damaged records"))?
            .chars()
            .map(Condition::try_from)
            .collect::<Result<_>>()?;
        let lens_damaged = parts
            .next()
            .ok_or_else(|| anyhow!("missing damaged spring lengths"))?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            conditions,
            damaged_groups: lens_damaged,
        })
    }
}

impl Record {
    fn arrangements(&self) -> usize {
        self.backtrack(0, 0, 0)
    }

    fn backtrack(&self, idx: usize, group: usize, run: usize) -> usize {
        if idx == self.conditions.len() {
            return usize::from(self.is_complete(group, run));
        }

        let mut count = 0;

        count += self.try_place_operational(idx, group, run);
        count += self.try_place_damaged(idx, group, run);

        count
    }

    fn is_complete(&self, group: usize, run: usize) -> bool {
        // All groups are done, and there is no run or;
        // At last group, and current run completes it
        (group == self.damaged_groups.len() && run == 0)
            || (group == self.damaged_groups.len() - 1 && run == self.damaged_groups[group])
    }

    fn try_place_operational(&self, idx: usize, group: usize, run: usize) -> usize {
        if self.conditions[idx] == Condition::Operational
            || self.conditions[idx] == Condition::Unknown
        {
            if run == 0 {
                // Not in a run
                self.backtrack(idx + 1, group, 0)
            } else if group < self.damaged_groups.len() && run == self.damaged_groups[group] {
                // Current run is done
                self.backtrack(idx + 1, group + 1, 0)
            } else {
                // Mid-group and still need more damaged; prune
                0
            }
        } else {
            // Cannot place a '.' on '#'
            0
        }
    }

    fn try_place_damaged(&self, idx: usize, group: usize, run: usize) -> usize {
        if self.conditions[idx] == Condition::Damaged || self.conditions[idx] == Condition::Unknown
        {
            if group < self.damaged_groups.len() && run < self.damaged_groups[group] {
                self.backtrack(idx + 1, group, run + 1)
            } else {
                // No more damaged groups remain, or already have enough damaged; prune
                0
            }
        } else {
            // Cannot place a '#' on '.'
            0
        }
    }
}

fn part1(records: &[Record]) -> usize {
    records.iter().map(Record::arrangements).sum()
}

fn main() -> Result<()> {
    let records = fs::read_to_string("in/day12.txt")?
        .lines()
        .map(Record::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&records);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 7_047);
    };

    Ok(())
}
