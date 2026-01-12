use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    network: Vec<(usize, usize)>,
    start: usize,
    end: usize,
    start2: Vec<usize>,
    end2: HashSet<usize>,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(instruction: char) -> Result<Self> {
        match instruction {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("invalid instruction '{instruction}'"),
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let mut parts = map.split("\n\n");

        let instructions = parts
            .next()
            .ok_or_else(|| anyhow!("missing instructions in map"))?
            .chars()
            .map(Instruction::try_from)
            .collect::<Result<Vec<_>>>()?;

        let network = parts
            .next()
            .ok_or_else(|| anyhow!("missing network in map"))?;
        let node_names = network
            .lines()
            .filter_map(|node| node.split(" = ").next())
            .collect::<Vec<_>>();
        let node_idxs = node_names
            .iter()
            .enumerate()
            .map(|(idx, &name)| (name, idx))
            .collect::<HashMap<_, _>>();

        let network = network
            .lines()
            .filter_map(|node| node.split(" = ").nth(1))
            .map(|dest| dest.trim_start_matches('(').trim_end_matches(')'))
            .map(|dest| dest.split(", "))
            .map(|mut split| split.next().zip(split.next()))
            .map(|split| split.ok_or_else(|| anyhow!("missing dest(s) in network")))
            .map(|split| split.map(|(i, j)| (node_idxs[i], node_idxs[j])))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            instructions,
            network,
            start: node_idxs["AAA"],
            end: node_idxs["ZZZ"],
            start2: node_idxs
                .iter()
                .filter_map(|(&name, &idx)| name.ends_with('A').then_some(idx))
                .collect(),
            end2: node_idxs
                .iter()
                .filter_map(|(&name, &idx)| name.ends_with('Z').then_some(idx))
                .collect(),
        })
    }
}

impl Map {
    fn steps_to_end(&self) -> usize {
        let mut count = 0;
        let mut node = self.start;
        let mut instruction = 0;

        while node != self.end {
            node = match self.instructions[instruction] {
                Instruction::Left => self.network[node].0,
                Instruction::Right => self.network[node].1,
            };

            count += 1;
            instruction += 1;
            instruction %= self.instructions.len();
        }

        count
    }

    fn steps_to_end2(&self, mut node: usize) -> usize {
        let mut count = 0;
        let mut instruction = 0;

        while !self.end2.contains(&node) {
            node = match self.instructions[instruction] {
                Instruction::Left => self.network[node].0,
                Instruction::Right => self.network[node].1,
            };

            count += 1;
            instruction += 1;
            instruction %= self.instructions.len();
        }

        count
    }
}

fn part1(map: &Map) -> usize {
    map.steps_to_end()
}

fn part2(map: &Map) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a / gcd(a, b)) * b
    }

    map.start2
        .iter()
        .map(|&start| map.steps_to_end2(start))
        .fold(1, lcm)
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day8.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&map);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 11_911);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&map);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 10_151_663_816_849);
    };

    Ok(())
}
