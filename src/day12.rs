use std::collections::HashMap;
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
    fn unfold(&self, factor: usize) -> Self {
        let conditions = self
            .conditions
            .iter()
            .copied()
            .chain([Condition::Unknown])
            .cycle()
            .take((self.conditions.len() * factor) + factor - 1)
            .collect();

        let damaged_groups = self
            .damaged_groups
            .iter()
            .copied()
            .cycle()
            .take(self.damaged_groups.len() * factor)
            .collect();

        Self {
            conditions,
            damaged_groups,
        }
    }

    fn arrangements(&self) -> usize {
        let mut memo = HashMap::new();
        self.backtrack(&mut memo, 0, 0, 0)
    }

    fn backtrack(
        &self,
        memo: &mut HashMap<(usize, usize, usize), usize>,
        idx: usize,
        group: usize,
        run: usize,
    ) -> usize {
        if let Some(&cached) = memo.get(&(idx, group, run)) {
            return cached;
        }

        let count = if idx == self.conditions.len() {
            usize::from(self.is_complete(group, run))
        } else {
            self.try_place_operational(memo, idx, group, run)
                + self.try_place_damaged(memo, idx, group, run)
        };

        memo.insert((idx, group, run), count);
        count
    }

    fn is_complete(&self, group: usize, run: usize) -> bool {
        // All groups are done, and there is no run or;
        // At last group, and current run completes it
        (group == self.damaged_groups.len() && run == 0)
            || (group == self.damaged_groups.len() - 1 && run == self.damaged_groups[group])
    }

    fn try_place_operational(
        &self,
        memo: &mut HashMap<(usize, usize, usize), usize>,
        idx: usize,
        group: usize,
        run: usize,
    ) -> usize {
        if self.conditions[idx] == Condition::Operational
            || self.conditions[idx] == Condition::Unknown
        {
            if run == 0 {
                // Not in a run
                self.backtrack(memo, idx + 1, group, 0)
            } else if group < self.damaged_groups.len() && run == self.damaged_groups[group] {
                // Current run is done
                self.backtrack(memo, idx + 1, group + 1, 0)
            } else {
                // Mid-group and still need more damaged; prune
                0
            }
        } else {
            // Cannot place a '.' on '#'
            0
        }
    }

    fn try_place_damaged(
        &self,
        memo: &mut HashMap<(usize, usize, usize), usize>,
        idx: usize,
        group: usize,
        run: usize,
    ) -> usize {
        if self.conditions[idx] == Condition::Damaged || self.conditions[idx] == Condition::Unknown
        {
            if group < self.damaged_groups.len() && run < self.damaged_groups[group] {
                self.backtrack(memo, idx + 1, group, run + 1)
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

fn part2(records: &[Record]) -> usize {
    records
        .iter()
        .map(|record| record.unfold(5).arrangements())
        .sum()
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

    {
        let start = Instant::now();
        let part2 = self::part2(&records);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 17_391_848_518_844);
    };

    Ok(())
}
